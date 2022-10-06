
use kvdb::{KeyValueDB, DBTransaction};

use crate::{columns::DBCol, error};

#[derive(Debug)]
pub enum DatabaseType {
    Full
}

/// Wrap RocksDb database into a trait object that implements `sp_database::Database`
pub fn as_database<D, H>(db: D) -> std::sync::Arc<dyn Database<H>>
where
	D: KeyValueDB + 'static,
	H: Clone + AsRef<[u8]>,
{
	std::sync::Arc::new(DbAdapter(db))
}

/// An alteration to the database.
#[derive(Clone)]
pub enum Change<H> {
	Set(DBCol, Vec<u8>, Vec<u8>),
	Remove(DBCol, Vec<u8>),
	Store(DBCol, H, Vec<u8>),
	Reference(DBCol, H),
	Release(DBCol, H),
}

/// A series of changes to the database that can be committed atomically. They do not take effect
/// until passed into `Database::commit`.
#[derive(Default, Clone)]
pub struct Transaction<H>(pub Vec<Change<H>>);

impl<H> Transaction<H> {
	/// Create a new transaction to be prepared and committed atomically.
	pub fn new() -> Self {
		Transaction(Vec::new())
	}
	/// Set the value of `key` in `col` to `value`, replacing anything that is there currently.
	pub fn set(&mut self, col: DBCol, key: &[u8], value: &[u8]) {
		self.0.push(Change::Set(col, key.to_vec(), value.to_vec()))
	}
	/// Set the value of `key` in `col` to `value`, replacing anything that is there currently.
	pub fn set_from_vec(&mut self, col: DBCol, key: &[u8], value: Vec<u8>) {
		self.0.push(Change::Set(col, key.to_vec(), value))
	}
	/// Remove the value of `key` in `col`.
	pub fn remove(&mut self, col: DBCol, key: &[u8]) {
		self.0.push(Change::Remove(col, key.to_vec()))
	}
	/// Store the `preimage` of `hash` into the database, so that it may be looked up later with
	/// `Database::get`. This may be called multiple times, but subsequent
	/// calls will ignore `preimage` and simply increase the number of references on `hash`.
	pub fn store(&mut self, col: DBCol, hash: H, preimage: Vec<u8>) {
		self.0.push(Change::Store(col, hash, preimage))
	}
	/// Increase the number of references for `hash` in the database.
	pub fn reference(&mut self, col: DBCol, hash: H) {
		self.0.push(Change::Reference(col, hash))
	}
	/// Release the preimage of `hash` from the database. An equal number of these to the number of
	/// corresponding `store`s must have been given before it is legal for `Database::get` to
	/// be unable to provide the preimage.
	pub fn release(&mut self, col: DBCol, hash: H) {
		self.0.push(Change::Release(col, hash))
	}
}

pub trait Database<H: Clone + AsRef<[u8]>>: Send + Sync {
	/// Commit the `transaction` to the database atomically. Any further calls to `get` or `lookup`
	/// will reflect the new state.
	fn commit(&self, transaction: Transaction<H>) -> error::Result<()>;

	/// Retrieve the value previously stored against `key` or `None` if
	/// `key` is not currently in the database.
	fn get(&self, col: DBCol, key: &[u8]) -> Option<Vec<u8>>;

	/// Check if the value exists in the database without retrieving it.
	fn contains(&self, col: DBCol, key: &[u8]) -> bool {
		self.get(col, key).is_some()
	}

	/// Check value size in the database possibly without retrieving it.
	fn value_size(&self, col: DBCol, key: &[u8]) -> Option<usize> {
		self.get(col, key).map(|v| v.len())
	}

	/// Call `f` with the value previously stored against `key`.
	///
	/// This may be faster than `get` since it doesn't allocate.
	/// Use `with_get` helper function if you need `f` to return a value from `f`
	fn with_get(&self, col: DBCol, key: &[u8], f: &mut dyn FnMut(&[u8])) {
		self.get(col, key).map(|v| f(&v));
	}

	/// Check if database supports internal ref counting for state data.
	///
	/// For backwards compatibility returns `false` by default.
	fn supports_ref_counting(&self) -> bool {
		false
	}

	/// Remove a possible path-prefix from the key.
	///
	/// Not all database implementations use a prefix for keys, so this function may be a noop.
	fn sanitize_key(&self, _key: &mut Vec<u8>) {}
}

impl<H> std::fmt::Debug for dyn Database<H> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Database")
	}
}

/// Call `f` with the value previously stored against `key` and return the result, or `None` if
/// `key` is not currently in the database.
///
/// This may be faster than `get` since it doesn't allocate.
pub fn with_get<R, H: Clone + AsRef<[u8]>>(
	db: &dyn Database<H>,
	col: DBCol,
	key: &[u8],
	mut f: impl FnMut(&[u8]) -> R,
) -> Option<R> {
	let mut result: Option<R> = None;
	let mut adapter = |k: &_| {
		result = Some(f(k));
	};
	db.with_get(col, key, &mut adapter);
	result
}


pub struct DbAdapter<D: KeyValueDB + 'static>(D);

fn handle_err<T>(result: std::io::Result<T>) -> T {
	match result {
		Ok(r) => r,
		Err(e) => {
			panic!("Critical database error: {:?}", e);
		},
	}
}

impl<D: KeyValueDB> DbAdapter<D> {
	// Returns counter key and counter value if it exists.
	fn read_counter(&self, col: DBCol, key: &[u8]) -> error::Result<(Vec<u8>, Option<u32>)> {
		// Add a key suffix for the counter
		let mut counter_key = key.to_vec();
		counter_key.push(0);
		Ok(match self.0.get(col as u32, &counter_key).map_err(|e| error::DatabaseError(Box::new(e)))? {
			Some(data) => {
				let mut counter_data = [0; 4];
				if data.len() != 4 {
					return Err(error::DatabaseError(Box::new(std::io::Error::new(
						std::io::ErrorKind::Other,
						format!("Unexpected counter len {}", data.len()),
					))))
				}
				counter_data.copy_from_slice(&data);
				let counter = u32::from_le_bytes(counter_data);
				(counter_key, Some(counter))
			},
			None => (counter_key, None),
		})
	}
}

impl<D: KeyValueDB, H: Clone + AsRef<[u8]>> Database<H> for DbAdapter<D> {
	fn commit(&self, transaction: Transaction<H>) -> error::Result<()> {
		let mut tx = DBTransaction::new();
		for change in transaction.0.into_iter() {
			match change {
				Change::Set(col, key, value) => tx.put_vec(col as u32, &key, value),
				Change::Remove(col, key) => tx.delete(col as u32, &key),
				Change::Store(col, key, value) => match self.read_counter(col, key.as_ref())? {
					(counter_key, Some(mut counter)) => {
						counter += 1;
						tx.put(col as u32, &counter_key, &counter.to_le_bytes());
					},
					(counter_key, None) => {
						let d = 1u32.to_le_bytes();
						tx.put(col as u32, &counter_key, &d);
						tx.put_vec(col as u32, key.as_ref(), value);
					},
				},
				Change::Reference(col, key) => {
					if let (counter_key, Some(mut counter)) =
						self.read_counter(col, key.as_ref())?
					{
						counter += 1;
						tx.put(col as u32, &counter_key, &counter.to_le_bytes());
					}
				},
				Change::Release(col, key) => {
					if let (counter_key, Some(mut counter)) =
						self.read_counter(col, key.as_ref())?
					{
						counter -= 1;
						if counter == 0 {
							tx.delete(col as u32, &counter_key);
							tx.delete(col as u32, key.as_ref());
						} else {
							tx.put(col as u32, &counter_key, &counter.to_le_bytes());
						}
					}
				},
			}
		}
		self.0.write(tx).map_err(|e| error::DatabaseError(Box::new(e)))
	}

	fn get(&self, col: DBCol, key: &[u8]) -> Option<Vec<u8>> {
		handle_err(self.0.get(col as u32, key))
	}

	fn contains(&self, col: DBCol, key: &[u8]) -> bool {
		handle_err(self.0.has_key(col as u32, key))
	}
}