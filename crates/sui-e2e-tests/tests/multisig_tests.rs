// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use fastcrypto::traits::EncodeDecodeBase64;
use shared_crypto::intent::{Intent, IntentMessage};
use sui_core::authority_client::AuthorityAPI;
use sui_macros::sim_test;
use sui_sdk::wallet_context::WalletContext;
use sui_test_transaction_builder::TestTransactionBuilder;
use sui_types::{
    base_types::{ObjectRef, SuiAddress},
    crypto::{get_key_pair, PublicKey, Signature, SuiKeyPair, ZkLoginPublicIdentifier},
    error::{SuiError, SuiResult},
    multisig::{MultiSig, MultiSigPublicKey},
    multisig_legacy::MultiSigPublicKeyLegacy,
    signature::GenericSignature,
    transaction::Transaction,
    utils::{keys, load_test_vectors, make_upgraded_multisig_tx},
    zk_login_authenticator::ZkLoginAuthenticator,
};
use test_cluster::TestClusterBuilder;
use tracing::info;

async fn do_upgraded_multisig_test() -> SuiResult {
    let test_cluster = TestClusterBuilder::new().build().await;
    let tx = make_upgraded_multisig_tx();

    test_cluster
        .authority_aggregator()
        .authority_clients
        .values()
        .next()
        .unwrap()
        .authority_client()
        .handle_transaction(tx)
        .await
        .map(|_| ())
}

#[sim_test]
async fn test_upgraded_multisig_feature_deny() {
    use sui_protocol_config::ProtocolConfig;

    let _guard = ProtocolConfig::apply_overrides_for_testing(|_, mut config| {
        config.set_upgraded_multisig_for_testing(false);
        config
    });

    let err = do_upgraded_multisig_test().await.unwrap_err();

    assert!(matches!(err, SuiError::UnsupportedFeatureError { .. }));
}

#[sim_test]
async fn test_upgraded_multisig_feature_allow() {
    use sui_protocol_config::ProtocolConfig;

    let _guard = ProtocolConfig::apply_overrides_for_testing(|_, mut config| {
        config.set_upgraded_multisig_for_testing(true);
        config
    });

    let res = do_upgraded_multisig_test().await;

    // we didn't make a real transaction with a valid object, but we verify that we pass the
    // feature gate.
    assert!(matches!(res.unwrap_err(), SuiError::UserInputError { .. }));
}

async fn fund_address_and_return_gas(
    context: &mut WalletContext,
    rgp: u64,
    funding_address: SuiAddress,
) -> ObjectRef {
    // fund the multisig address.
    let (sender, gas) = context.get_one_gas_object().await.unwrap().unwrap();
    let tx = context.sign_transaction(
        &TestTransactionBuilder::new(sender, gas, rgp)
            .transfer_sui(Some(20000000000), funding_address)
            .build(),
    );
    context.execute_transaction_must_succeed(tx).await;

    context
        .get_one_gas_object_owned_by_address(funding_address)
        .await
        .unwrap()
        .unwrap()
}
#[sim_test]
async fn test_multisig_e2e() {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let context = &mut test_cluster.wallet;

    let keys = keys();
    let pk0 = keys[0].public(); // ed25519
    let pk1 = keys[1].public(); // secp256k1
    let pk2 = keys[2].public(); // secp256r1

    let multisig_pk = MultiSigPublicKey::construct(
        vec![(pk0.clone(), 1), (pk1.clone(), 1), (pk2.clone(), 1)],
        2,
    );
    let multisig_addr = SuiAddress::from(&multisig_pk);

    // fund wallet and get a gas object to use later.
    let gas = fund_address_and_return_gas(context, rgp, multisig_addr).await;

    // 1. sign with key 0 and 1 executes successfully.
    let tx1 = TestTransactionBuilder::new(multisig_addr, gas, rgp)
        .transfer_sui(None, SuiAddress::ZERO)
        .build_and_sign_multisig(multisig_pk.clone(), &[&keys[0], &keys[1]], 0b011);
    let res = context.execute_transaction_must_succeed(tx1).await;
    assert!(res.status_ok().unwrap());

    // 2. sign with key 1 and 2 executes successfully.
    let gas = fund_address_and_return_gas(context, rgp, multisig_addr).await;
    let tx2 = TestTransactionBuilder::new(multisig_addr, gas, rgp)
        .transfer_sui(None, SuiAddress::ZERO)
        .build_and_sign_multisig(multisig_pk.clone(), &[&keys[1], &keys[2]], 0b110);
    let res = context.execute_transaction_must_succeed(tx2).await;
    assert!(res.status_ok().unwrap());

    // 3. signature 2 and 1 swapped fails to execute.
    let gas = fund_address_and_return_gas(context, rgp, multisig_addr).await;
    let tx3 = TestTransactionBuilder::new(multisig_addr, gas.clone(), rgp)
        .transfer_sui(None, SuiAddress::ZERO)
        .build_and_sign_multisig(multisig_pk.clone(), &[&keys[2], &keys[1]], 0b110);
    let res = context.execute_transaction_may_fail(tx3).await;
    assert!(res
        .unwrap_err()
        .to_string()
        .contains("Invalid sig for pk=AQIOF81ZOeRrGWZBlozXWZELold+J/pz/eOHbbm+xbzrKw=="));

    // 4. sign with key 0 only is below threshold, fails to execute.
    let tx4 = TestTransactionBuilder::new(multisig_addr, gas.clone(), rgp)
        .transfer_sui(None, SuiAddress::ZERO)
        .build_and_sign_multisig(multisig_pk.clone(), &[&keys[0]], 0b001);
    let res = context.execute_transaction_may_fail(tx4).await;
    assert!(res
        .unwrap_err()
        .to_string()
        .contains("Insufficient weight=1 threshold=2"));

    // 5. multisig with no single sig fails to execute.
    let tx5 = TestTransactionBuilder::new(multisig_addr, gas.clone(), rgp)
        .transfer_sui(None, SuiAddress::ZERO)
        .build_and_sign_multisig(multisig_pk.clone(), &[], 0b001);
    let res = context.execute_transaction_may_fail(tx5).await;
    assert!(res
        .unwrap_err()
        .to_string()
        .contains("Invalid signature was given to the function"));

    // 6. multisig two dup sigs fails to execute.
    let tx6 = TestTransactionBuilder::new(multisig_addr, gas.clone(), rgp)
        .transfer_sui(None, SuiAddress::ZERO)
        .build_and_sign_multisig(multisig_pk.clone(), &[&keys[0], &keys[0]], 0b011);
    let res = context.execute_transaction_may_fail(tx6).await;
    assert!(res
        .unwrap_err()
        .to_string()
        .contains("Invalid ed25519 pk bytes"));

    // 7. mismatch pks in sig with multisig address fails to execute.
    let kp3: SuiKeyPair = SuiKeyPair::Secp256r1(get_key_pair().1);
    let pk3 = kp3.public();
    let wrong_multisig_pk =
        MultiSigPublicKey::new(vec![pk0.clone(), pk1.clone(), pk3], vec![1, 1, 1], 2).unwrap();
    let wrong_sender = SuiAddress::from(&wrong_multisig_pk);
    let gas = fund_address_and_return_gas(context, rgp, wrong_sender).await;
    let tx7 = TestTransactionBuilder::new(wrong_sender, gas, rgp)
        .transfer_sui(None, SuiAddress::ZERO)
        .build_and_sign_multisig(wrong_multisig_pk.clone(), &[&keys[0], &keys[2]], 0b101);
    let res = context.execute_transaction_may_fail(tx7).await;
    assert!(res
        .unwrap_err()
        .to_string()
        .contains("Invalid sig for pk=AgIr5mvYQW6NF/3TU23ZCggycepE9sxZkBV/r27s6d/YXQ=="));
}

#[sim_test]
async fn test_multisig_with_zklogin_scenerios() {
    let mut test_cluster = TestClusterBuilder::new().with_default_jwks().build().await;
    test_cluster.wait_for_authenticator_state_update().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let context = &mut test_cluster.wallet;

    let keys = keys();
    let pk0 = keys[0].public(); // ed25519
    let pk1 = keys[1].public(); // secp256k1
    let pk2 = keys[2].public(); // secp256r1

    // construct a multisig address with 4 pks (ed25519, secp256k1, secp256r1, zklogin) with threshold = 1.
    let (eph_kp, _eph_pk, zklogin_inputs) =
        &load_test_vectors("../sui-types/src/unit_tests/zklogin_test_vectors.json")[0];
    let (eph_kp_1, _, _) =
        &load_test_vectors("../sui-types/src/unit_tests/zklogin_test_vectors.json")[1];
    let zklogin_pk = PublicKey::ZkLogin(
        ZkLoginPublicIdentifier::new(zklogin_inputs.get_iss(), zklogin_inputs.get_address_seed())
            .unwrap(),
    );
    // let multisig_pk = MultiSigPublicKey::new(
    //     vec![pk0.clone(), pk1.clone(), pk2.clone(), zklogin_pk.clone()],
    //     vec![1, 1, 1, 1],
    //     1,
    // )
    // .unwrap();

    // // fund the multisig address.
    // let multisig_addr = SuiAddress::from(&multisig_pk);
    // let gas = fund_address_and_return_gas(context, rgp, multisig_addr).await;
    // let tx_data = TestTransactionBuilder::new(multisig_addr, gas.clone(), rgp)
    //     .transfer_sui(None, SuiAddress::ZERO)
    //     .build();
    // let intent_msg = IntentMessage::new(Intent::sui_transaction(), tx_data.clone());
    // let wrong_intent_msg = IntentMessage::new(Intent::personal_message(), tx_data.clone());

    // // 1. a multisig with a bad ed25519 sig fails to execute.
    // let wrong_sig: GenericSignature = Signature::new_secure(&wrong_intent_msg, &keys[0]).into();
    // let multisig = GenericSignature::MultiSig(
    //     MultiSig::combine(vec![wrong_sig], multisig_pk.clone()).unwrap(),
    // );
    // let tx_1 = Transaction::from_generic_sig_data(tx_data.clone(), vec![multisig]);
    // let res = context.execute_transaction_may_fail(tx_1).await;
    // assert!(res
    //     .unwrap_err()
    //     .to_string()
    //     .contains(format!("Invalid sig for pk={}", pk0.encode_base64()).as_str()));

    // // 2. a multisig with a bad secp256k1 sig fails to execute.
    // let wrong_sig_2: GenericSignature = Signature::new_secure(&wrong_intent_msg, &keys[1]).into();
    // let multisig = GenericSignature::MultiSig(
    //     MultiSig::combine(vec![wrong_sig_2], multisig_pk.clone()).unwrap(),
    // );
    // let tx_2 = Transaction::from_generic_sig_data(tx_data.clone(), vec![multisig]);
    // let res = context.execute_transaction_may_fail(tx_2).await;
    // assert!(res
    //     .unwrap_err()
    //     .to_string()
    //     .contains(format!("Invalid sig for pk={}", pk1.encode_base64()).as_str()));

    // // 3. a multisig with a bad secp256r1 sig fails to execute.
    // let wrong_sig_3: GenericSignature = Signature::new_secure(&wrong_intent_msg, &keys[2]).into();
    // let multisig = GenericSignature::MultiSig(
    //     MultiSig::combine(vec![wrong_sig_3], multisig_pk.clone()).unwrap(),
    // );
    // let tx_3 = Transaction::from_generic_sig_data(tx_data.clone(), vec![multisig]);
    // let res = context.execute_transaction_may_fail(tx_3).await;
    // assert!(res
    //     .unwrap_err()
    //     .to_string()
    //     .contains(format!("Invalid sig for pk={}", pk2.encode_base64()).as_str()));

    // // 4. a multisig with a bad ephemeral sig inside zklogin sig fails to execute.
    // let wrong_eph_sig = Signature::new_secure(&wrong_intent_msg, eph_kp);
    // let wrong_zklogin_sig = GenericSignature::ZkLoginAuthenticator(ZkLoginAuthenticator::new(
    //     zklogin_inputs.clone(),
    //     10,
    //     wrong_eph_sig,
    // ));
    // let multisig = GenericSignature::MultiSig(
    //     MultiSig::combine(vec![wrong_zklogin_sig], multisig_pk.clone()).unwrap(),
    // );
    // let tx_4 = Transaction::from_generic_sig_data(tx_data.clone(), vec![multisig]);
    // let res = context.execute_transaction_may_fail(tx_4).await;
    // let pk3 = PublicKey::ZkLogin(
    //     ZkLoginPublicIdentifier::new(zklogin_inputs.get_iss(), zklogin_inputs.get_address_seed())
    //         .unwrap(),
    // );
    // assert!(res
    //     .unwrap_err()
    //     .to_string()
    //     .contains(format!("Invalid sig for pk={}", pk3.encode_base64()).as_str()));

    // // 5. a multisig with a zklogin sig with wrong ephemeral sig fails to execute.
    // let wrong_eph_sig = Signature::new_secure(&wrong_intent_msg, eph_kp);
    // let wrong_zklogin_sig = GenericSignature::ZkLoginAuthenticator(ZkLoginAuthenticator::new(
    //     zklogin_inputs.clone(),
    //     10,
    //     wrong_eph_sig,
    // ));
    // let multisig = GenericSignature::MultiSig(
    //     MultiSig::combine(vec![wrong_zklogin_sig], multisig_pk.clone()).unwrap(),
    // );
    // let tx_5 = Transaction::from_generic_sig_data(tx_data.clone(), vec![multisig]);
    // let res = context.execute_transaction_may_fail(tx_5).await;
    // assert!(res
    //     .unwrap_err()
    //     .to_string()
    //     .contains(format!("Invalid sig for pk={}", pk3.encode_base64()).as_str()));

    // // 6. a multisig with a mismatch ephermeal sig and zklogin inputs fails to execute.
    // let eph_sig = Signature::new_secure(&intent_msg, eph_kp_1);
    // let zklogin_sig_mismatch = GenericSignature::ZkLoginAuthenticator(
    //     ZkLoginAuthenticator::new(zklogin_inputs.clone(), 10, eph_sig),
    // );
    // let multisig = GenericSignature::MultiSig(
    //     MultiSig::combine(vec![zklogin_sig_mismatch], multisig_pk.clone()).unwrap(),
    // );
    // let tx_6 = Transaction::from_generic_sig_data(tx_data.clone(), vec![multisig]);
    // let res = context.execute_transaction_may_fail(tx_6).await;
    // assert!(res
    //     .unwrap_err()
    //     .to_string()
    //     .contains(format!("Invalid sig for pk={}", pk3.encode_base64()).as_str()));

    // // 7. a multisig with an inconsistent max_epoch with zk proof itself fails to execute.
    // let eph_sig = Signature::new_secure(&intent_msg, eph_kp);
    // let zklogin_sig_wrong_zklogin_inputs = GenericSignature::ZkLoginAuthenticator(
    //     ZkLoginAuthenticator::new(zklogin_inputs.clone(), 9, eph_sig), // max_epoch set to 9 instead of 10
    // );
    // let multisig = GenericSignature::MultiSig(
    //     MultiSig::combine(vec![zklogin_sig_wrong_zklogin_inputs], multisig_pk.clone()).unwrap(),
    // );
    // let tx_7 = Transaction::from_generic_sig_data(tx_data.clone(), vec![multisig]);
    // let res = context.execute_transaction_may_fail(tx_7).await;
    // assert!(res
    //     .unwrap_err()
    //     .to_string()
    //     .contains("Groth16 proof verify failed"));

    // // assert positive case for all above scenerios.
    // // 1a. good ed25519 sig used in multisig executes successfully.
    // let gas = fund_address_and_return_gas(context, rgp, multisig_addr).await;
    // let tx_data = TestTransactionBuilder::new(multisig_addr, gas.clone(), rgp)
    //     .transfer_sui(None, SuiAddress::ZERO)
    //     .build();
    // let intent_msg = IntentMessage::new(Intent::sui_transaction(), tx_data.clone());
    // let sig_0: GenericSignature = Signature::new_secure(&intent_msg, &keys[0]).into();
    // let multisig =
    //     GenericSignature::MultiSig(MultiSig::combine(vec![sig_0], multisig_pk.clone()).unwrap());
    // let tx_8 = Transaction::from_generic_sig_data(tx_data, vec![multisig]);
    // let _ = context.execute_transaction_must_succeed(tx_8).await;

    // // 2a. good secp256k1 sig used in multisig executes successfully.
    // let gas = fund_address_and_return_gas(context, rgp, multisig_addr).await;
    // let tx_data = TestTransactionBuilder::new(multisig_addr, gas, rgp)
    //     .transfer_sui(None, SuiAddress::ZERO)
    //     .build();
    // let intent_msg = IntentMessage::new(Intent::sui_transaction(), tx_data.clone());
    // let sig_1: GenericSignature = Signature::new_secure(&intent_msg, &keys[1]).into();
    // let multisig =
    //     GenericSignature::MultiSig(MultiSig::combine(vec![sig_1], multisig_pk.clone()).unwrap());
    // let tx_9 = Transaction::from_generic_sig_data(tx_data, vec![multisig]);
    // let _ =context.execute_transaction_must_succeed(tx_9).await;

    // // 3a. good secp256r1 sig used in multisig executes successfully.
    // let gas = fund_address_and_return_gas(context, rgp, multisig_addr).await;
    // let tx_data = TestTransactionBuilder::new(multisig_addr, gas, rgp)
    //     .transfer_sui(None, SuiAddress::ZERO)
    //     .build();
    // let intent_msg = IntentMessage::new(Intent::sui_transaction(), tx_data.clone());
    // let sig_2: GenericSignature = Signature::new_secure(&intent_msg, &keys[2]).into();
    // let multisig =
    //     GenericSignature::MultiSig(MultiSig::combine(vec![sig_2], multisig_pk.clone()).unwrap());
    // let tx_9 = Transaction::from_generic_sig_data(tx_data, vec![multisig]);
    // let _ = context.execute_transaction_must_succeed(tx_9).await;

    // // 4a. good zklogin sig used in multisig executes successfully.
    // let gas = fund_address_and_return_gas(context, rgp, multisig_addr).await;
    // let tx_data = TestTransactionBuilder::new(multisig_addr, gas, rgp)
    //     .transfer_sui(None, SuiAddress::ZERO)
    //     .build();
    // let intent_msg = IntentMessage::new(Intent::sui_transaction(), tx_data.clone());
    // let sig_4: GenericSignature = ZkLoginAuthenticator::new(
    //     zklogin_inputs.clone(),
    //     10,
    //     Signature::new_secure(&intent_msg, eph_kp),
    // )
    // .into();
    // let multisig =
    //     GenericSignature::MultiSig(MultiSig::combine(vec![sig_4], multisig_pk.clone()).unwrap());
    // let tx_10 = Transaction::from_generic_sig_data(tx_data.clone(), vec![multisig]);
    // let _ =context.execute_transaction_must_succeed(tx_10).await;

    // // 8. wrong bitmap fails to execute.
    // let gas = fund_address_and_return_gas(context, rgp, multisig_addr).await;
    // let tx_data = TestTransactionBuilder::new(multisig_addr, gas, rgp)
    //     .transfer_sui(None, SuiAddress::ZERO)
    //     .build();
    // let intent_msg = IntentMessage::new(Intent::sui_transaction(), tx_data.clone());
    // let sig: GenericSignature = Signature::new_secure(&intent_msg, &keys[0]).into();
    // let multisig = GenericSignature::MultiSig(MultiSig::new(vec![sig.to_compressed().unwrap()], 0b0010, multisig_pk.clone()));
    // let tx_11 = Transaction::from_generic_sig_data(tx_data.clone(), vec![multisig]);
    // let res = context.execute_transaction_may_fail(tx_11).await;
    // assert!(res
    //     .unwrap_err()
    //     .to_string()
    //     .contains("Invalid ed25519 pk bytes"));

    // // 9. invalid bitmap b10000 when the max bitmap for 4 pks is b1111, fails to execute.
    // let multisig = GenericSignature::MultiSig(MultiSig::new(vec![sig.clone().to_compressed().unwrap()], 1 << 4, multisig_pk.clone()));
    // let tx_12 = Transaction::from_generic_sig_data(tx_data.clone(), vec![multisig]);
    // let res = context.execute_transaction_may_fail(tx_12).await;
    // assert!(res
    //     .unwrap_err()
    //     .to_string()
    //     .contains("Invalid public keys index"));

    // 10. malformed multisig pk where threshold = 0, fails to execute.
    let bad_multisig_pk = MultiSigPublicKey::construct(vec![(pk0.clone(), 1), (pk1.clone(), 1), (pk2.clone(), 1)],0);
    let bad_multisig_addr = SuiAddress::from(&bad_multisig_pk);
    let gas_1 = fund_address_and_return_gas(context, rgp, bad_multisig_addr).await;
    let tx_data_1 = TestTransactionBuilder::new(bad_multisig_addr, gas_1, rgp)
        .transfer_sui(None, SuiAddress::ZERO)
        .build();
    let intent_msg_1 = IntentMessage::new(Intent::sui_transaction(), tx_data_1.clone());
    let sig_1: GenericSignature = Signature::new_secure(&intent_msg_1, &keys[0]).into();
    let multisig_1 = GenericSignature::MultiSig(MultiSig::new(vec![sig_1.to_compressed().unwrap()], 0b001, bad_multisig_pk.clone()));
    let tx_13 = Transaction::from_generic_sig_data(tx_data_1.clone(), vec![multisig_1]);
    let res = context.execute_transaction_may_fail(tx_13).await;
    info!("dwqml;tkres: {:?}", res);
    assert!(res
        .unwrap_err()
        .to_string()
        .contains("Invalid multisig pk"));

    // // malformed multisig a pk has weight = 0, fails to execute.
    // let bad_multisig_pk = MultiSigPublicKey::new(
    //     vec![pk1.clone(), pk2.clone(), pk3.clone()],
    //     vec![1, 1, 0],
    //     1,
    // )
    // .unwrap();
    // let multisig = GenericSignature::MultiSig(MultiSig::new(vec![sig_1.to_compressed().unwrap()], 2, bad_multisig_pk));
    // let tx_9 = Transaction::from_generic_sig_data(tx_data_8.clone(), vec![multisig]);
    // let _ = context.execute_transaction_may_fail(tx_9).await;

    // // pass in 2 sigs when only 1 pk in multisig_pk, fails to execute.
    // let small_multisig_pk = MultiSigPublicKey::construct(vec![(pk2.clone(), 1)],1);
    // let multisig = GenericSignature::MultiSig(MultiSig::new(vec![sig_1.to_compressed().unwrap(), sig_1.to_compressed().unwrap()], 2, small_multisig_pk));
    // let tx_9 = Transaction::from_generic_sig_data(tx_data_8.clone(), vec![multisig]);
    // let _ = context.execute_transaction_may_fail(tx_9).await;

    // // pass a multisig where there is dup pk in multisig_pk, fails to execute.
    // let bad_multisig_pk = MultiSigPublicKey::construct(
    //     vec![(pk2.clone(), 1), (pk2.clone(), 1)],
    //     1,
    // );
    // let multisig = GenericSignature::MultiSig(MultiSig::new(vec![sig_1.to_compressed().unwrap()], 1, bad_multisig_pk));
    // let tx_9 = Transaction::from_generic_sig_data(tx_data_8.clone(), vec![multisig]);
    // let _ = context.execute_transaction_may_fail(tx_9).await;

    // // a sig with 11 pks fails

    // // total weight of all pks < threshold

    // // empty pk map
    // let bad_multisig_pk = MultiSigPublicKey::construct(vec![],1,);
    // let multisig = GenericSignature::MultiSig(MultiSig::new(vec![sig_1.to_compressed().unwrap()], 1, bad_multisig_pk));
    // let tx_9 = Transaction::from_generic_sig_data(tx_data_8.clone(), vec![multisig]);
    // let _ = context.execute_transaction_may_fail(tx_9).await;

    // // invalid compressed sig bytes for zklogin authenticator bytes
    // let multisig = GenericSignature::MultiSig(MultiSig::new(vec![CompressedSignature::ZkLogin(ZkLoginAuthenticatorAsBytes(vec![]))], 1, multisig_pk_2));
    // let tx_9 = Transaction::from_generic_sig_data(tx_data_8.clone(), vec![multisig]);
    // let _ = context.execute_transaction_may_fail(tx_9).await;
}
#[sim_test]
async fn test_multisig_legacy_e2e() {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;

    let keys = keys();
    let pk1 = keys[0].public();
    let pk2 = keys[1].public();
    let pk3 = keys[2].public();

    let multisig_pk_legacy = MultiSigPublicKeyLegacy::new(
        vec![pk1.clone(), pk2.clone(), pk3.clone()],
        vec![1, 1, 1],
        2,
    )
    .unwrap();
    let multisig_pk = MultiSigPublicKey::new(
        vec![pk1.clone(), pk2.clone(), pk3.clone()],
        vec![1, 1, 1],
        2,
    )
    .unwrap();
    let multisig_addr = SuiAddress::from(&multisig_pk);
    let context = &mut test_cluster.wallet;
    let gas = fund_address_and_return_gas(context, rgp, multisig_addr).await;
    let transfer_from_multisig = TestTransactionBuilder::new(multisig_addr, gas, rgp)
        .transfer_sui(Some(1000000), SuiAddress::ZERO)
        .build_and_sign_multisig_legacy(multisig_pk_legacy, &[&keys[0], &keys[1]]);

    context
        .execute_transaction_must_succeed(transfer_from_multisig)
        .await;
}

#[sim_test]
async fn test_zklogin_inside_multisig_feature_deny() {
    use sui_protocol_config::ProtocolConfig;

    let _guard = ProtocolConfig::apply_overrides_for_testing(|_, mut config| {
        config.set_upgraded_multisig_for_testing(false);
        config
    });

    let err = do_upgraded_multisig_test().await.unwrap_err();

    assert!(matches!(err, SuiError::UnsupportedFeatureError { .. }));
}

// #[sim_test]
// async fn test_zklogin_inside_multisig_scenerios() {
//     let err = do_upgraded_multisig_test().await.unwrap_err();

//     assert!(matches!(err, SuiError::UnsupportedFeatureError { .. }));
// }
