# rpc package
This package provides RPC interface for communicating with the node.
It is used in these places:
- `akula/bin/akula-rpc.rs`: single instance RPC connect directly to Mbdx
- `akula/bin/akula.rs`: embeded directly to akula node
- `akula/consensus/beacon.rs`: embeded in the beacon consensus.