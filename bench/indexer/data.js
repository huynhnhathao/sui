window.BENCHMARK_DATA = {
  "lastUpdate": 1707181015155,
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
      }
    ]
  }
}