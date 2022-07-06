# consensus package
This package implements the DPoS-BFT consensus engine. It maintains internal state and peer state via transporting messages via `kardia-sentry` service.
Its main components are:
- ticker for processing data in round and steps
- consensus reactor: handle messages from peer or internal state changes
- models: state, peer state, message

Checkout at `akula/consensus/beacon.rs`, it runs as a server. It exposes Eth RPC services for querying blockchain DB.