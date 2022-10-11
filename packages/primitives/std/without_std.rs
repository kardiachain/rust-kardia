pub extern crate alloc;

pub use alloc::boxed;
pub use alloc::rc;
pub use alloc::sync;
pub use alloc::vec;
pub use core::any;
pub use core::cell;
pub use core::clone;
pub use core::cmp;
pub use core::convert;
pub use core::default;
pub use core::fmt;
pub use core::hash;
pub use core::iter;
pub use core::marker;
pub use core::mem;
pub use core::num;
pub use core::ops;
pub use core::ptr;
pub use core::result;
pub use core::slice;
// Allow interpreting vectors of bytes as strings, but not constructing them.
pub use core::str;
pub use core::time;
// We are trying to avoid certain things here, such as `core::string`
// (if you need `String` you are probably doing something wrong, since
// runtime doesn't require anything human readable).

pub mod collections {
	pub use alloc::collections::btree_map;
	pub use alloc::collections::btree_set;
	pub use alloc::collections::vec_deque;
}

pub mod borrow {
	pub use core::borrow::*;
	pub use alloc::borrow::*;
}

pub mod thread {
	/// Returns if the current thread is panicking.
	///
	/// In wasm this always returns `false`, as we abort on any panic.
	pub fn panicking() -> bool {
		false
	}
}