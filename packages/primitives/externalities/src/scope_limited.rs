//! Stores the externalities in an `environmental` value to make it scope limited available.

use crate::Externalities;

environmental::environmental!(ext: trait Externalities);

/// Set the given externalities while executing the given closure. To get access to the
/// externalities while executing the given closure [`with_externalities`] grants access to them.
/// The externalities are only set for the same thread this function was called from.
pub fn set_and_run_with_externalities<F, R>(ext: &mut dyn Externalities, f: F) -> R
where
	F: FnOnce() -> R,
{
	ext::using(ext, f)
}

/// Execute the given closure with the currently set externalities.
///
/// Returns `None` if no externalities are set or `Some(_)` with the result of the closure.
pub fn with_externalities<F: FnOnce(&mut dyn Externalities) -> R, R>(f: F) -> Option<R> {
	ext::with(f)
}