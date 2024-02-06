window.BENCHMARK_DATA = {
  "lastUpdate": 1707239790940,
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
      }
    ]
  }
}