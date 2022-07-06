# sentry package
Service that listens to Ethereum's P2P network, serves information to other nodes, and provides gRPC interface to clients to interact with the network.
This package heavily learned from the sentry package of `akula` repo.

## Call hierarchy
- `akula.rs`
- `p2p/node/node.rs`
- 

Note:
- It is using RLPx, we need to convert to use Protobuf
- Do we have penalize peer???