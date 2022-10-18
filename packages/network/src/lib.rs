pub mod config;
pub mod protocol;
pub mod peer_info;

/// The maximum allowed number of established connections per peer.
///
/// Typically, and by design of the network behaviours in this crate,
/// there is a single established connection per peer. However, to
/// avoid unnecessary and nondeterministic connection closure in
/// case of (possibly repeated) simultaneous dialing attempts between
/// two peers, the per-peer connection limit is not set to 1 but 2.
const MAX_CONNECTIONS_PER_PEER: usize = 2;