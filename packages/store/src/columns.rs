use std::fmt;

/// This enum holds the information about the columns that we use within the
/// RocksDB storage.
///
/// You can think about our storage as 2-dimensional table (with key and column
/// as indexes/coordinates).
///
/// Note that the names of the variants in this enumeration correspond to the
/// name of the RocksDB column families.  As such, it is *not* safe to rename
/// a variant.
///
/// The only exception is adding an underscore at the beginning of the name to
/// indicate that the column has been deprecated.  Deprecated columns are not
/// used except for the database migration code which needs to deal with the
/// deprecation.  Make sure to add `#[strum(serialize = "OriginalName")]`
/// attribute in front of the variant when you deprecate a column.
#[derive(
    PartialEq,
    Copy,
    Clone,
    Debug,
    Hash,
    Eq,
    enum_map::Enum,
    strum_macros::EnumIter,
    strum_macros::IntoStaticStr,
)]
pub enum DBCol {
    // Column that stores current database version.
    DBVersion,
    // Column that stores latest know header's hash.
    HeadHeader,
    // Column that stores latest know full block's hash.
    HeadBlock,
    // Column that stores latest know incomplete block's hash during fast sync.
    HeadFastBlock,
    // Column that stores latest know finalized block hash.
    HeadFinalizedBlock,
    // Column that stores latest pivot block used by fast sync.
    LastPivotKey,
    // Colums that stores the number of trie entries imported during fast sync.
    FastTrieProgress,
    // Column that stores flags that the snapshot should not be maintained due to initial sync.
    SnapshotDisable,
    // Column that stores latest snapshot hash.
    SnapshotRoot,
    // Column that stores the snapshot generation marker across restarts.
    SnapshotJournal,
    SnapshotGenerator,
    // Column that stores the snapshot recovery marker across restarts.
    SnapshotRecovery,
    // Column that stores the skeleton sync status across restarts.
    SnapshotSyncStatus,
    SkeletonSyncStatus,
    // Column that stores the oldest block whose transactions have been indexed.
    TxIndexTail,
    FastTxLookupLimit,
    // Column that stores the list of bad blocks seen by local
    BadBlock,
    // Column that stores the kai2 transition status.
    TransitionStatus,
    // Column that stores the list of local crashes
    UnclearShutdown,
    // Column that stores block contents.
    Block,
    // Column that stores block bodies.
    BlockBody,
    // Column that stores block receipts.
    BlockReciepts,
    // Column that stores mapping from block number to block hash.
    BlockNumber,
    // Column that stores mapping from block hash to block number.
    BlockHash,
    // Column that stores block headers.
    Header,
    // Column that stores tx lookup metadata.
    TxLookup,
    // Column that store bloombits.
    BloomBits,
    // Column that stores bloom bits index.
    BoomBitsIndex,
    // Column that stores snapshot account.
    SnapshotAccount,
    // Column that stores snapshot storage.
    SnapshotStorage,
    // Colume that stores contract code.
    Code,
    // Colum that stores preimage.
    Preimage,
    // Column that stores chain config.
    Config,
    // Column that stores chain genesis.
    Genesis,
    // Colum that store skeleton header.
    SkeletonHeader,
    // Colum that stores state.
    State,
}

impl DBCol {
    /// Whether data in this column is effectively immutable.
    ///
    /// Data in such columns is never overwriten, though it can be deleted by gc
    /// eventually. Specifically, for a given key:
    ///
    /// * It's OK to insert a new value.
    /// * It's also OK to insert a value which is equal to the one already
    ///   stored.
    /// * Inserting a different value would crash the node in debug, but not in
    ///   release.
    /// * GC (and only GC) is allowed to remove any value.
    ///
    /// In some sence, insert-only column acts as an rc-column, where rc is
    /// always one.
    pub const fn is_insert_only(&self) -> bool {
        match self {
            DBCol::Block | DBCol::BlockBody => true,
            _ => false,
        }
    }

    /// Whethere this column is reference-counted.
    ///
    /// A reference-counted column is one where we store additional 8-byte value
    /// at the end of the payload with the current reference counter value.  For
    /// such columns you must not use `set`, `set_ser` or `delete` operations,
    /// but 'increment_refcount' and `decrement_refcount` instead.
    ///
    /// Under the hood, we’re using custom merge operator (see
    /// [`crate::db::RocksDB::refcount_merge`]) to properly ‘join’ the
    /// refcounted cells.  This means that the 'value' for a given key must
    /// never change.
    ///
    /// Example:
    ///
    /// ```ignore
    /// increment_refcount("foo", "bar");
    /// // good - after this call, the RC will be equal to 3.
    /// increment_refcount_by("foo", "bar", 2);
    /// // bad - the value is still 'bar'.
    /// increment_refcount("foo", "baz");
    /// // ok - the value will be removed now. (as rc == 0)
    /// decrement_refcount_by("foo", "", 3)
    /// ```
    ///
    /// Quick note on negative refcounts: if we have a key that ends up having
    /// a negative refcount, we have to store this value (negative ref) in the
    /// database.
    ///
    /// Example:
    ///
    /// ```ignore
    /// increment_refcount("a", "b");
    /// decrement_refcount_by("a", 3);
    /// // Now we have the entry in the database with key "a", empty payload and
    /// // refcount value of -2,
    /// ```
    pub const fn is_rc(&self) -> bool {
        match self {
            DBCol::State => true,
            _ => false,
        }
    }
}

impl fmt::Display for DBCol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}


#[test]
fn column_props_sanity() {
    use strum::IntoEnumIterator;

    for col in DBCol::iter() {
        // Check that rc and write_once are mutually exclusive.
        assert!((col.is_rc() as u32) + (col.is_insert_only() as u32) <= 1, "{col}")
    }
}