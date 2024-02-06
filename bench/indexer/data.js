window.BENCHMARK_DATA = {
  "lastUpdate": 1707254913212,
  "repoUrl": "https://github.com/MystenLabs/sui",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "81660174+mwtian@users.noreply.github.com",
            "name": "mwtian",
            "username": "mwtian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9b3db06fa7fc3c95ae764e2a64dd0ab01fe5945b",
          "message": "[Consensus] add basic RPC client and server (#16035)\n\n## Description \r\n\r\nSet up basic anemo client and server, using unary RPCs.\r\n\r\n## Test Plan \r\n\r\nUnit test\r\n\r\n---\r\nIf your changes are not user-facing and do not break anything, you can\r\nskip the following section. Otherwise, please briefly describe what has\r\nchanged under the Release Notes section.\r\n\r\n### Type of Change (Check all that apply)\r\n\r\n- [ ] protocol change\r\n- [ ] user-visible impact\r\n- [ ] breaking change for a client SDKs\r\n- [ ] breaking change for FNs (FN binary must upgrade)\r\n- [ ] breaking change for validators or node operators (must upgrade\r\nbinaries)\r\n- [ ] breaking change for on-chain data layout\r\n- [ ] necessitate either a data wipe or data migration\r\n\r\n### Release notes",
          "timestamp": "2024-02-06T00:46:56Z",
          "tree_id": "aed524b3ad261f4229f49d0a9b1ea464f733769f",
          "url": "https://github.com/MystenLabs/sui/commit/9b3db06fa7fc3c95ae764e2a64dd0ab01fe5945b"
        },
        "date": 1707181010929,
        "tool": "cargo",
        "benches": [
          {
            "name": "get_checkpoint",
            "value": 333140,
            "range": "± 23971",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "113186356+Rajwanshi1@users.noreply.github.com",
            "name": "Rajwanshi1",
            "username": "Rajwanshi1"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "99ccaa83d31afa934b4a36e51f2892ff91ee4975",
          "message": "Compute metadata flag to optimize gas.  (#15275)\n\n## Description \r\n\r\nAdded compute_metadata flag to compute metadata only when required by\r\nthe caller. Return val for metadata has been changed to Option<T>.\r\n\r\n## Test Plan \r\n\r\nThis proposed change does'nt affect signature of any public func. Hence,\r\nexisting tests should suffice.\r\n\r\n---\r\nIf your changes are not user-facing and not a breaking change, you can\r\nskip the following section. Otherwise, please indicate what changed, and\r\nthen add to the Release Notes section as highlighted during the release\r\nprocess.\r\n\r\n### Type of Change (Check all that apply)\r\n\r\n- [ ] protocol change\r\n- [ ] user-visible impact\r\n- [ ] breaking change for a client SDKs\r\n- [ ] breaking change for FNs (FN binary must upgrade)\r\n- [ ] breaking change for validators or node operators (must upgrade\r\nbinaries)\r\n- [ ] breaking change for on-chain data layout\r\n- [ ] necessitate either a data wipe or data migration\r\n\r\n### Release notes\r\n\r\n---------\r\n\r\nCo-authored-by: Sarthak Rajwanshi <sarthak.rajwanshi@gmail.com>\r\nCo-authored-by: patrick <patrickkuo@me.com>",
          "timestamp": "2024-02-06T12:08:54-05:00",
          "tree_id": "96f29fda158231f0738bd8621b6e80ef41c3dd82",
          "url": "https://github.com/MystenLabs/sui/commit/99ccaa83d31afa934b4a36e51f2892ff91ee4975"
        },
        "date": 1707239786201,
        "tool": "cargo",
        "benches": [
          {
            "name": "get_checkpoint",
            "value": 371959,
            "range": "± 27281",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "rvantonder@gmail.com",
            "name": "Rijnard van Tonder",
            "username": "rvantonder"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2db429258728beffbcf15a72858b90e1343c4832",
          "message": "move: simplify TestPlan::new interface (refactor) (#16094)\n\n## Description \r\n\r\n`TestPlan::new` takes `AnnotatedCompiledUnit` when it only needs a subvalue\r\n`NamedCompiledModule`\r\n\r\n## Test Plan \r\n\r\nJust a refactor.\r\n\r\n---\r\nIf your changes are not user-facing and do not break anything, you can\r\nskip the following section. Otherwise, please briefly describe what has\r\nchanged under the Release Notes section.\r\n\r\n### Type of Change (Check all that apply)\r\n\r\n- [ ] protocol change\r\n- [ ] user-visible impact\r\n- [ ] breaking change for a client SDKs\r\n- [ ] breaking change for FNs (FN binary must upgrade)\r\n- [ ] breaking change for validators or node operators (must upgrade\r\nbinaries)\r\n- [ ] breaking change for on-chain data layout\r\n- [ ] necessitate either a data wipe or data migration\r\n\r\n### Release notes",
          "timestamp": "2024-02-06T10:06:02-08:00",
          "tree_id": "ebda651db83ced1bfcb07745da7bdc39d9fc690c",
          "url": "https://github.com/MystenLabs/sui/commit/2db429258728beffbcf15a72858b90e1343c4832"
        },
        "date": 1707243202806,
        "tool": "cargo",
        "benches": [
          {
            "name": "get_checkpoint",
            "value": 365719,
            "range": "± 30795",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "51927076+phoenix-o@users.noreply.github.com",
            "name": "phoenix",
            "username": "phoenix-o"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f54311ed5ae8d7dccd4f0ea7ab667ad8ea03d724",
          "message": "[data ingestion] archival worker (#16039)\n\nincludes a slight change to the data ingestion interface to support the\r\nuse case. Now tasks can skip progress updates in the meta store,\r\nallowing the accumulation of state with multiple checkpoints before\r\nsaving and ensuring correct behavior during random crash/restart.\r\nNote: this workload is expected to be run with concurrency=1 only",
          "timestamp": "2024-02-06T13:08:07-05:00",
          "tree_id": "9f60654e652e02f1e36b28c27ff135e4f5cbe86b",
          "url": "https://github.com/MystenLabs/sui/commit/f54311ed5ae8d7dccd4f0ea7ab667ad8ea03d724"
        },
        "date": 1707243352859,
        "tool": "cargo",
        "benches": [
          {
            "name": "get_checkpoint",
            "value": 386042,
            "range": "± 36526",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "halfprice@users.noreply.github.com",
            "name": "Zhe Wu",
            "username": "halfprice"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d9be11951e330b33bd929e3a26d01a69e3fcf9f1",
          "message": "Overload monitor simulation tests (#16079)\n\n## Description \r\n\r\nCreate a simulation test to test overload monitor. Particularly, it\r\ntests 5 scenarios\r\n- No overload\r\n- Slightly overload\r\n- Heavy overload\r\n- Single spike\r\n- Consistent spikes\r\n\r\nThe tests use a simple load generator and executor. Both use a\r\nAuthorityState.overload_info to keep track of requests. The overload\r\nmonitor inside the same AuthorityState can be used as an overload\r\nmonitor for the load generator.\r\n\r\nThis test discovered several improvements from the previous load\r\ngenerator. Some notable ones are:\r\n- When we are already in shedding mode, say 50%, and decide to shed 10%,\r\npreviously the new shedding percentage will be 10% but it really should\r\nbe 50% + 50% * 10% = 55%.\r\n- When txn ready rate < execution rate, it doesn't mean that the\r\noverload is over. We need to protect against future spikes by only\r\ngradually reduce shedding percentage.\r\n\r\n## Test Plan \r\n\r\nSimulation tests\r\n\r\n---\r\nIf your changes are not user-facing and do not break anything, you can\r\nskip the following section. Otherwise, please briefly describe what has\r\nchanged under the Release Notes section.\r\n\r\n### Type of Change (Check all that apply)\r\n\r\n- [ ] protocol change\r\n- [ ] user-visible impact\r\n- [ ] breaking change for a client SDKs\r\n- [ ] breaking change for FNs (FN binary must upgrade)\r\n- [ ] breaking change for validators or node operators (must upgrade\r\nbinaries)\r\n- [ ] breaking change for on-chain data layout\r\n- [ ] necessitate either a data wipe or data migration\r\n\r\n### Release notes",
          "timestamp": "2024-02-06T12:52:32-08:00",
          "tree_id": "5a63b92d78c48f85feb98f044f53e32d92aaaf92",
          "url": "https://github.com/MystenLabs/sui/commit/d9be11951e330b33bd929e3a26d01a69e3fcf9f1"
        },
        "date": 1707253198505,
        "tool": "cargo",
        "benches": [
          {
            "name": "get_checkpoint",
            "value": 362162,
            "range": "± 26262",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "tmn@mystenlabs.com",
            "name": "Todd Nowacki",
            "username": "tnowacki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3696f5f7347ccd27cc479e5ffc525eab7b4e3a37",
          "message": "Fixed sui_deny_list_object_id comment (#16101)\n\n## Description \r\n\r\nSmall doc fix\r\n\r\n## Test Plan \r\n\r\nHow did you test the new or updated feature?\r\n\r\n---\r\nIf your changes are not user-facing and do not break anything, you can\r\nskip the following section. Otherwise, please briefly describe what has\r\nchanged under the Release Notes section.\r\n\r\n### Type of Change (Check all that apply)\r\n\r\n- [ ] protocol change\r\n- [ ] user-visible impact\r\n- [ ] breaking change for a client SDKs\r\n- [ ] breaking change for FNs (FN binary must upgrade)\r\n- [ ] breaking change for validators or node operators (must upgrade\r\nbinaries)\r\n- [ ] breaking change for on-chain data layout\r\n- [ ] necessitate either a data wipe or data migration\r\n\r\n### Release notes",
          "timestamp": "2024-02-06T13:21:02-08:00",
          "tree_id": "03c2544331738f60b3b3afcc6d71ef483bce4bae",
          "url": "https://github.com/MystenLabs/sui/commit/3696f5f7347ccd27cc479e5ffc525eab7b4e3a37"
        },
        "date": 1707254911283,
        "tool": "cargo",
        "benches": [
          {
            "name": "get_checkpoint",
            "value": 358607,
            "range": "± 21994",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}