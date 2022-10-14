//! Types that should only be used for testing!

use crate::crypto::KeyTypeId;

/// Key type for generic Ed25519 key.
pub const ED25519: KeyTypeId = KeyTypeId(*b"ed25");
/// Key type for generic Sr 25519 key.
pub const SR25519: KeyTypeId = KeyTypeId(*b"sr25");
/// Key type for generic ECDSA key.
pub const ECDSA: KeyTypeId = KeyTypeId(*b"ecds");

/// A task executor that can be used in tests.
///
/// Internally this just wraps a `ThreadPool` with a pool size of `8`. This
/// should ensure that we have enough threads in tests for spawning blocking futures.
#[cfg(feature = "std")]
#[derive(Clone)]
pub struct TaskExecutor(futures::executor::ThreadPool);

#[cfg(feature = "std")]
impl TaskExecutor {
	/// Create a new instance of `Self`.
	pub fn new() -> Self {
		let mut builder = futures::executor::ThreadPoolBuilder::new();
		Self(builder.pool_size(8).create().expect("Failed to create thread pool"))
	}
}

#[cfg(feature = "std")]
impl Default for TaskExecutor {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(feature = "std")]
impl crate::traits::SpawnNamed for TaskExecutor {
	fn spawn_blocking(
		&self,
		_name: &'static str,
		_group: Option<&'static str>,
		future: futures::future::BoxFuture<'static, ()>,
	) {
		self.0.spawn_ok(future);
	}
	fn spawn(
		&self,
		_name: &'static str,
		_group: Option<&'static str>,
		future: futures::future::BoxFuture<'static, ()>,
	) {
		self.0.spawn_ok(future);
	}
}

#[cfg(feature = "std")]
impl crate::traits::SpawnEssentialNamed for TaskExecutor {
	fn spawn_essential_blocking(
		&self,
		_: &'static str,
		_: Option<&'static str>,
		future: futures::future::BoxFuture<'static, ()>,
	) {
		self.0.spawn_ok(future);
	}
	fn spawn_essential(
		&self,
		_: &'static str,
		_: Option<&'static str>,
		future: futures::future::BoxFuture<'static, ()>,
	) {
		self.0.spawn_ok(future);
	}
}