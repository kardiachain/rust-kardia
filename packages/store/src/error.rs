/// The error type for database operations.
#[derive(Debug)]
pub struct DatabaseError(pub Box<dyn std::error::Error + Send + Sync + 'static>);

impl std::fmt::Display for DatabaseError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl std::error::Error for DatabaseError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		None
	}
}

/// A specialized `Result` type for database operations.
pub type Result<T> = std::result::Result<T, DatabaseError>;