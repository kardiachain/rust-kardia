use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ConsensusConfig {
    // Reactor sleep duration parameters are in milliseconds
    pub timeout_propose: Duration,
    pub timeout_propose_delta: Duration,
    pub timeout_prevote: Duration,
    pub timeout_prevote_delta: Duration,
    pub timeout_precommit: Duration,
    pub timeout_precommit_delta: Duration,
    pub peer_gossip_sleep_duration: Duration,
    pub peer_query_maj23_sleep_duration: Duration,
}

impl ConsensusConfig {
    pub fn new_default() -> Self {
        Self {
            // WalPath:                     filepath.Join(DefaultDataDir(), "cs.wal", "wal"),
            timeout_propose: Duration::from_millis(3000),
            timeout_propose_delta: Duration::from_millis(500),
            timeout_prevote: Duration::from_millis(1000),
            timeout_prevote_delta: Duration::from_millis(500),
            timeout_precommit: Duration::from_millis(1000),
            timeout_precommit_delta: Duration::from_millis(500),
            // TimeoutCommit:               1000 * time.Millisecond,
            // IsSkipTimeoutCommit:         false,
            // IsCreateEmptyBlocks:         true,
            // CreateEmptyBlocksInterval:   3500 * time.Millisecond,
            // PeerGossipSleepDuration:     100 * time.Millisecond,
            // PeerQueryMaj23SleepDuration: 2000 * time.Millisecond,
            peer_gossip_sleep_duration: Duration::from_millis(100),
            peer_query_maj23_sleep_duration: Duration::from_millis(2000),
        }
    }
}
