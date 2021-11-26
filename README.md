# README 

thoraudit is a simple cli tool for auditing a thornode's running state changes back to the blockheight when it first came online. 


## Usage

`thoraudit [node_address]`

ie:

`thoraudiT thor1h0xd53y8yvfsu5t8p6t4ky52h9dzfz3mvsem3z`

## Output

```
url: https://thornode.thorchain.info/thorchain/node/thor1h0xd53y8yvfsu5t8p6t4ky52h9dzfz3mvsem3z
NodeInfo {
    node_address: "thor1h0xd53y8yvfsu5t8p6t4ky52h9dzfz3mvsem3z",
    status: "Active",
    bond: "72752878504842",
    bond_address: "thor18r4hpkhsvc7ts25gkrzu7h7q3s9n4zs6j5qfnv",
    active_block_height: 1218789,
    status_since: 1218789,
    version: "0.74.1",
    current_award: "189010398977",
    slash_points: 58992,
    preflight_status: PreflightStatus {
        status: "Ready",
        reason: "OK",
        code: 0,
    },
}
url: https://thornode.thorchain.info/thorchain/node/thor1h0xd53y8yvfsu5t8p6t4ky52h9dzfz3mvsem3z?height=1218788
NodeInfo {
    node_address: "thor1h0xd53y8yvfsu5t8p6t4ky52h9dzfz3mvsem3z",
    status: "Ready",
    bond: "67990519962714",
    bond_address: "thor18r4hpkhsvc7ts25gkrzu7h7q3s9n4zs6j5qfnv",
    active_block_height: 0,
    status_since: 1218783,
    version: "0.56.3",
    current_award: "0",
    slash_points: 5,
    preflight_status: PreflightStatus {
        status: "Ready",
        reason: "OK",
        code: 0,
    },
}
url: https://thornode.thorchain.info/thorchain/node/thor1h0xd53y8yvfsu5t8p6t4ky52h9dzfz3mvsem3z?height=1218782
NodeInfo {
    node_address: "thor1h0xd53y8yvfsu5t8p6t4ky52h9dzfz3mvsem3z",
    status: "Standby",
    bond: "56790519962714",
    bond_address: "thor18r4hpkhsvc7ts25gkrzu7h7q3s9n4zs6j5qfnv",
    active_block_height: 0,
    status_since: 1175583,
    version: "0.56.3",
    current_award: "0",
    slash_points: 5,
    preflight_status: PreflightStatus {
        status: "Ready",
        reason: "OK",
        code: 0,
    },
}
...
```
