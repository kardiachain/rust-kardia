//! Global state cache. Maintains recently queried/committed state values
//! Tracks changes over the span of a few recent blocks and handles forks
//! by tracking/removing cache entries for conflicting changes.
const STATE_CACHE_BLOCKS: usize = 12;

type ChildStorageKey = (Vec<u8>, Vec<u8>);
