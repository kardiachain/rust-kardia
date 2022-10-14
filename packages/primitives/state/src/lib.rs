//! Kardiachain state implementation.

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
mod basic;
#[cfg(feature = "std")]
pub use log::{debug, error as log_error, warn};
pub mod backend;
pub(crate) mod overlayed_changes;
#[cfg(feature = "std")]
pub use tracing::trace;
mod error;
mod ext;
#[cfg(feature = "std")]
mod in_memory_backend;
mod stats;
mod trie_backend;
mod trie_backend_essence;
#[cfg(feature = "std")]
pub use execution::*;
#[cfg(feature = "std")]
pub use std_reexport::*;
#[cfg(feature = "std")]
mod read_only;
#[cfg(feature = "std")]
mod testing;

#[cfg(feature = "std")]
mod std_reexport {
    pub use crate::{
        error::{Error, ExecutionError},
        in_memory_backend::{new_in_mem, new_in_mem_hash_key},
        trie_backend::create_proof_check_backend,
    };
    pub use kp_trie::{
        trie_types::{TrieDBMutV0, TrieDBMutV1},
        CompactProof, DBValue, LayoutV0, LayoutV1, MemoryDB, StorageProof, TrieMut,
    };
}

pub use crate::{
    backend::Backend,
    error::{Error, ExecutionError},
    overlayed_changes::{
        ChildStorageCollection, IndexOperation, OffchainChangesCollection, OverlayedChanges,
        StorageChanges, StorageCollection, StorageKey, StorageTransactionCache, StorageValue,
    },
    stats::{StateMachineStats, UsageInfo, UsageUnit},
    trie_backend::{TrieBackend, TrieBackendBuilder},
    trie_backend_essence::{Storage, TrieBackendStorage},
};

/// In no_std we skip logs for state_machine, this macro
/// is a noops.
#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! warn {
	(target: $target:expr, $message:expr $( , $arg:ident )* $( , )?) => {
		{
			$(
				let _ = &$arg;
			)*
		}
	};
	($message:expr, $( $arg:expr, )*) => {
		{
			$(
				let _ = &$arg;
			)*
		}
	};
}

/// Default error type to use with state machine trie backend.
#[cfg(feature = "std")]
pub type DefaultError = String;
/// Error type to use with state machine trie backend.
#[cfg(not(feature = "std"))]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct DefaultError;

#[cfg(not(feature = "std"))]
impl kp_std::fmt::Display for DefaultError {
    fn fmt(&self, f: &mut kp_std::fmt::Formatter) -> kp_std::fmt::Result {
        write!(f, "DefaultError")
    }
}

#[cfg(feature = "std")]
mod execution {
    use kp_trie::MemoryDB;

    use crate::TrieBackend;

    /// Trie backend with in-memory storage.
    pub type InMemoryBackend<H> = TrieBackend<MemoryDB<H>, H>;
}
