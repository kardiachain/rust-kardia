use std::{path::Path, sync::Arc, fmt, io};

use strum::IntoEnumIterator;

use crate::{kvdb::{DatabaseType, as_database, Database}, columns::DBCol, error, DbHash};


pub fn open_kvdb_rocksdb(path: &Path, create: bool, db_type: DatabaseType, cache_size: usize) -> OpenDbResult{
    let num_columns = DBCol::iter().count() as u32;
    // and now open database assuming that it has the latest version
	let mut db_config = kvdb_rocksdb::DatabaseConfig::with_columns(num_columns);
	db_config.create_if_missing = create;

    let mut memory_budget = std::collections::HashMap::new();
    match db_type {
        DatabaseType::Full => {
            let state_col_budget = (cache_size as f64 * 0.9) as usize;
			let other_col_budget = (cache_size - state_col_budget) / (num_columns as usize - 1);
            for i in 0..num_columns {
				if i == DBCol::State as u32 {
					memory_budget.insert(i, state_col_budget);
				} else {
					memory_budget.insert(i, other_col_budget);
				}
			}
        }
    }
    db_config.memory_budget = memory_budget;
    let db = kvdb_rocksdb::Database::open(&db_config, path)?;
    
    Ok(as_database(db))
}


#[derive(Debug)]
pub enum OpenDbError {
	// constructed only when rocksdb and paritydb are disabled
	#[allow(dead_code)]
	NotEnabled(&'static str),
	DoesNotExist,
	Internal(String),
	DatabaseError(error::DatabaseError),
	UnexpectedDbType {
		expected: DatabaseType,
		found: Vec<u8>,
	},
}

type OpenDbResult = Result<Arc<dyn Database<DbHash>>, OpenDbError>;

impl fmt::Display for OpenDbError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			OpenDbError::Internal(e) => write!(f, "{}", e),
			OpenDbError::DoesNotExist => write!(f, "Database does not exist at given location"),
			OpenDbError::NotEnabled(feat) => {
				write!(f, "`{}` feature not enabled, database can not be opened", feat)
			},
			OpenDbError::DatabaseError(db_error) => {
				write!(f, "Database Error: {}", db_error)
			},
			OpenDbError::UnexpectedDbType { expected, found } => {
				write!(
					f,
					"Unexpected DB-Type. Expected: {:?}, Found: {:?}",
					expected.as_str().as_bytes(),
					found
				)
			},
		}
	}
}

impl From<parity_db::Error> for OpenDbError {
	fn from(err: parity_db::Error) -> Self {
		if matches!(err, parity_db::Error::DatabaseNotFound) {
			OpenDbError::DoesNotExist
		} else {
			OpenDbError::Internal(err.to_string())
		}
	}
}

impl From<io::Error> for OpenDbError {
	fn from(err: io::Error) -> Self {
		if err.to_string().contains("create_if_missing is false") {
			OpenDbError::DoesNotExist
		} else {
			OpenDbError::Internal(err.to_string())
		}
	}
}


impl DatabaseType {
	/// Returns str representation of the type.
	pub fn as_str(&self) -> &'static str {
		match *self {
			DatabaseType::Full => "full",
		}
	}
}
