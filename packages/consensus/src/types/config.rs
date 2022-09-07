use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ConsensusConfig {
    // Reactor sleep duration parameters are in milliseconds
    pub peer_gossip_sleep_duration: Duration,
    pub peer_query_maj23_sleep_duration: Duration,
}

impl ConsensusConfig {
    pub fn new_default() -> Self {
        Self {
            // WalPath:                     filepath.Join(DefaultDataDir(), "cs.wal", "wal"),
            // TimeoutPropose:              3000 * time.Millisecond,
            // TimeoutProposeDelta:         500 * time.Millisecond,
            // TimeoutPrevote:              1000 * time.Millisecond,
            // TimeoutPrevoteDelta:         500 * time.Millisecond,
            // TimeoutPrecommit:            1000 * time.Millisecond,
            // TimeoutPrecommitDelta:       500 * time.Millisecond,
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