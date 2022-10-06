use columns::DBCol;
use crate::kvdb::Transaction;
use std::sync::Arc;

use crate::kvdb::Database;

mod columns;
mod config;
mod error;
mod kvdb;
mod utils;

pub type DbHash = primitive_types::H256;

pub struct Store {
    db: Arc<dyn Database<DbHash>>,
}

impl Store {
    pub fn new(db: Arc<dyn Database<DbHash>>) -> Self {
        Store { db: db }
    }

    pub fn get(&self, col: DBCol, key: &[u8]) -> Option<Vec<u8>> {
        self.db.get(col, key)
    }

    pub fn commit(&self, transaction: Transaction<DbHash>) -> error::Result<()> {
        self.db.commit(transaction)
    }

    /// Check if the value exists in the database without retrieving it.
    pub fn contains(&self, col: DBCol, key: &[u8]) -> bool {
        self.db.get(col, key).is_some()
    }
}
