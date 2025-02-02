// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use mysten_metrics::start_prometheus_server;
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    sync::Arc,
    time::Duration,
};
use sui_bridge::{
    action_executor::BridgeActionExecutor,
    client::bridge_authority_aggregator::BridgeAuthorityAggregator,
    config::{BridgeClientConfig, BridgeNodeConfig},
    eth_syncer::EthSyncer,
    orchestrator::BridgeOrchestrator,
    server::{handler::BridgeRequestHandler, run_server},
    storage::BridgeOrchestratorTables,
    sui_syncer::SuiSyncer,
};
use sui_config::Config;
use tokio::task::JoinHandle;
use tracing::info;

// TODO consolidate this with sui-node/src/main.rs, but where to put it?
const GIT_REVISION: &str = {
    if let Some(revision) = option_env!("GIT_REVISION") {
        revision
    } else {
        let version = git_version::git_version!(
            args = ["--always", "--abbrev=12", "--dirty", "--exclude", "*"],
            fallback = ""
        );

        if version.is_empty() {
            panic!("unable to query git revision");
        }
        version
    }
};
const VERSION: &str = const_str::concat!(env!("CARGO_PKG_VERSION"), "-", GIT_REVISION);

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
#[clap(name = env!("CARGO_BIN_NAME"))]
#[clap(version = VERSION)]
struct Args {
    #[clap(long)]
    pub config_path: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = BridgeNodeConfig::load(&args.config_path).unwrap();

    // Init metrics server
    let metrics_address =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.metrics_port);
    let registry_service = start_prometheus_server(metrics_address);
    let prometheus_registry = registry_service.default_registry();
    mysten_metrics::init_metrics(&prometheus_registry);
    info!("Metrics server started at port {}", config.metrics_port);

    // Init logging
    let (_guard, _filter_handle) = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .with_prom_registry(&prometheus_registry)
        .init();

    let (server_config, client_config) = config.validate().await?;

    // Start Client
    let _handles = if let Some(client_config) = client_config {
        start_client_components(client_config).await
    } else {
        Ok(vec![])
    }?;

    // Start Server
    let socket_address = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        server_config.server_listen_port,
    );
    run_server(
        &socket_address,
        BridgeRequestHandler::new(
            server_config.key,
            server_config.sui_client,
            server_config.eth_client,
        ),
    )
    .await;
    Ok(())
}

// TODO: is there a way to clean up the overrides after it's stored in DB?
async fn start_client_components(
    client_config: BridgeClientConfig,
) -> anyhow::Result<Vec<JoinHandle<()>>> {
    let store: std::sync::Arc<BridgeOrchestratorTables> =
        BridgeOrchestratorTables::new(&client_config.db_path.join("client"));
    let stored_module_cursors = store
        .get_sui_event_cursors(&client_config.sui_bridge_modules)
        .map_err(|e| anyhow::anyhow!("Unable to get sui event cursors from storage: {e:?}"))?;
    let mut sui_modules_to_watch = HashMap::new();
    for (module, cursor) in client_config
        .sui_bridge_modules
        .iter()
        .zip(stored_module_cursors)
    {
        if client_config
            .sui_bridge_modules_last_processed_event_id_override
            .contains_key(module)
        {
            sui_modules_to_watch.insert(
                module.clone(),
                client_config.sui_bridge_modules_last_processed_event_id_override[module],
            );
            info!(
                "Overriding cursor for sui bridge module {} to {:?}. Stored cursor: {:?}",
                module,
                client_config.sui_bridge_modules_last_processed_event_id_override[module],
                cursor
            );
        } else if let Some(cursor) = cursor {
            sui_modules_to_watch.insert(module.clone(), cursor);
        } else {
            return Err(anyhow::anyhow!(
                "No cursor found for sui bridge module {} in storage or config override",
                module
            ));
        }
    }

    let stored_eth_cursors = store
        .get_eth_event_cursors(&client_config.eth_bridge_contracts)
        .map_err(|e| anyhow::anyhow!("Unable to get eth event cursors from storage: {e:?}"))?;
    let mut eth_contracts_to_watch = HashMap::new();
    for (contract, cursor) in client_config
        .eth_bridge_contracts
        .iter()
        .zip(stored_eth_cursors)
    {
        if client_config
            .eth_bridge_contracts_start_block_override
            .contains_key(contract)
        {
            eth_contracts_to_watch.insert(
                *contract,
                client_config.eth_bridge_contracts_start_block_override[contract],
            );
            info!(
                "Overriding cursor for eth bridge contract {} to {}. Stored cursor: {:?}",
                contract, client_config.eth_bridge_contracts_start_block_override[contract], cursor
            );
        } else if let Some(cursor) = cursor {
            // +1: The stored value is the last block that was processed, so we start from the next block.
            eth_contracts_to_watch.insert(*contract, cursor + 1);
        } else {
            return Err(anyhow::anyhow!(
                "No cursor found for eth contract {} in storage or config override",
                contract
            ));
        }
    }

    let sui_client = client_config.sui_client.clone();

    let mut all_handles = vec![];
    let (task_handles, eth_events_rx, _) =
        EthSyncer::new(client_config.eth_client.clone(), eth_contracts_to_watch)
            .run()
            .await
            .expect("Failed to start eth syncer");
    all_handles.extend(task_handles);

    let (task_handles, sui_events_rx) =
        SuiSyncer::new(client_config.sui_client, sui_modules_to_watch)
            .run(Duration::from_secs(2))
            .await
            .expect("Failed to start sui syncer");
    all_handles.extend(task_handles);

    let committee = Arc::new(
        sui_client
            .get_bridge_committee()
            .await
            .expect("Failed to get committee"),
    );
    let bridge_auth_agg = BridgeAuthorityAggregator::new(committee);

    let bridge_action_executor = BridgeActionExecutor::new(
        sui_client.clone(),
        Arc::new(bridge_auth_agg),
        store.clone(),
        client_config.key,
        client_config.sui_address,
        client_config.gas_object_ref.0,
    );

    let orchestrator =
        BridgeOrchestrator::new(sui_client, sui_events_rx, eth_events_rx, store.clone());

    all_handles.extend(orchestrator.run(bridge_action_executor));
    Ok(all_handles)
}
