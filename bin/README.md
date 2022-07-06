# bin package
This package contains 3 binaries:
- `kardia-rpc`: service that connects to Kardia P2P network.
- `kardia-sentry`: service that connects the node to the p2p network of the blockchain. `core` and  `consensus` subcribe to this service for further processings.
- `kardia`: the heaviest service that contains `tx_pool`, `downloader` (connects to BitTorrent P2P network) daemon.