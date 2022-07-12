# rpc package
This package provides RPC interface for communicating with the node.
It is used in these places:
- `bin/kardia-rpc.rs`: single instance RPC connect directly to Mbdx
- `bin/kardia.rs`: embeded directly to kardia node
- `consensus/beacon.rs`: embeded in the beacon consensus.