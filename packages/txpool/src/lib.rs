mod types;

use kp_core::hash::H256;
use kp_types::transaction::transaction::Transaction;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::ops::Bound;
use types::{PoolIterator, PoolKey, TransactionGroup};

// Tx Pool: keeps track of transactions that were not yet accepted into the block chain.
pub struct TxPool {
    transactions: BTreeMap<PoolKey, Vec<Transaction>>,
    know_transactions: HashSet<H256>,
    last_used_key: PoolKey,
}

impl TxPool {
    pub fn new() -> Self {
        Self {
            transactions: BTreeMap::new(),
            know_transactions: HashSet::new(),
            last_used_key: H256::default(),
        }
    }

    /// Insert a signed transaction into the pool that passed validation.
    pub fn add_transaction(&mut self, tx: Transaction) -> bool {
        if !self.know_transactions.insert(tx.hash()) {
            return false;
        }
        self.transactions
            .entry(tx.from)
            .or_insert_with(Vec::new)
            .push(tx);
        true
    }

    pub fn remove_transactions(&mut self, txs: &[Transaction]) {
        let mut grouped_transactions = HashMap::new();
        for tx in txs {
            if self.know_transactions.contains(&tx.hash()) {
                grouped_transactions
                    .entry(tx.from)
                    .or_insert_with(HashSet::new)
                    .insert(tx.hash());
            }
        }
        for (key, hashes) in grouped_transactions {
            let mut remove_entry = false;
            if let Some(v) = self.transactions.get_mut(&key) {
                v.retain(|tx| !hashes.contains(&tx.hash()));
                remove_entry = v.is_empty();
            }
            if remove_entry {
                self.transactions.remove(&key);
            }
            for hash in &hashes {
                self.know_transactions.remove(&hash);
            }
        }
    }

    /// Reintroduce transactions back during the chain reorg
    pub fn reintroduce_transactions(&mut self, transactions: Vec<Transaction>) {
        for tx in transactions {
            self.add_transaction(tx);
        }
    }

    pub fn len(&self) -> usize {
        self.know_transactions.len()
    }
}

/// PoolIterator is a structure to pull transactions from the pool.
/// It implements `PoolIterator` trait that iterates over transaction groups one by one.
/// When the wrapper is dropped the remaining transactions are returned back to the pool.
pub struct PoolIteratorWrapper<'a> {
    /// Mutable reference to the pool, to avoid exposing it while the iterator exists.
    pool: &'a mut TxPool,

    /// Queue of transaction groups. Each group there is sorted by nonce.
    sorted_groups: VecDeque<TransactionGroup>,
}

impl<'a> PoolIteratorWrapper<'a> {
    pub fn new(pool: &'a mut TxPool) -> Self {
        Self {
            pool,
            sorted_groups: Default::default(),
        }
    }
}

/// The iterator works with the following algorithm:
/// On next(), the iterator tries to get a transaction group from the pool, sorts transactions in
/// it, and add it to the back of the sorted groups queue.
/// Remembers the last used key, so it can continue from the next key.
///
/// If the pool is empty, the iterator gets the group from the front of the sorted groups queue.
///
/// If this group is empty (no transactions left inside), then the iterator discards it and
/// updates `unique_transactions` in the pool. Then gets the next one.
///
/// Once a non-empty group is found, this group is pushed to the back of the sorted groups queue
/// and the iterator returns a mutable reference to this group.
///
/// If the sorted groups queue is empty, the iterator returns None.
///
/// When the iterator is dropped, `unique_transactions` in the pool is updated for every group.
/// And all non-empty group from the sorted groups queue are inserted back into the pool.
impl<'a> PoolIterator for PoolIteratorWrapper<'a> {
    fn next(&mut self) -> Option<&mut TransactionGroup> {
        if !self.pool.transactions.is_empty() {
            let key = *self
                .pool
                .transactions
                .range((Bound::Excluded(self.pool.last_used_key), Bound::Unbounded))
                .next()
                .map(|(k, _v)| k)
                .unwrap_or_else(|| {
                    self.pool
                        .transactions
                        .keys()
                        .next()
                        .expect("we've just checked that the map is not empty")
                });
            self.pool.last_used_key = key;
            let mut transactions = self
                .pool
                .transactions
                .remove(&key)
                .expect("just checked existence");
            transactions.sort_by_key(|st| std::cmp::Reverse(st.nonce));
            self.sorted_groups.push_back(TransactionGroup {
                key,
                transactions,
                removed_transaction_hashes: vec![],
            });
            Some(self.sorted_groups.back_mut().expect("just pushed"))
        } else {
            while let Some(sorted_group) = self.sorted_groups.pop_front() {
                if sorted_group.transactions.is_empty() {
                    for hash in sorted_group.removed_transaction_hashes {
                        if self.pool.know_transactions.remove(&hash) {
                            // metrics::TRANSACTION_POOL_TOTAL.dec();
                        }
                    }
                } else {
                    self.sorted_groups.push_back(sorted_group);
                    return Some(self.sorted_groups.back_mut().expect("just pushed"));
                }
            }
            None
        }
    }
}

/// When a pool iterator is dropped, all remaining non empty transaction groups from the sorted
/// groups queue are inserted back into the pool. And removed transactions hashes from groups are
/// removed from the pool's unique_transactions.
impl<'a> Drop for PoolIteratorWrapper<'a> {
    fn drop(&mut self) {
        for group in self.sorted_groups.drain(..) {
            for hash in group.removed_transaction_hashes {
                if self.pool.know_transactions.remove(&hash) {
                    // metrics::TRANSACTION_POOL_TOTAL.dec();
                }
            }
            if !group.transactions.is_empty() {
                self.pool.transactions.insert(group.key, group.transactions);
            }
        }
    }
}
