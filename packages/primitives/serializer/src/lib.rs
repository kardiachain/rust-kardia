//!
//! The idea is that we can later change the implementation
//! to something more compact, but for now we're using JSON.

#![warn(missing_docs)]

pub use serde_json::{from_reader, from_slice, from_str, Error, Result};

const PROOF: &str = "Serializers are infallible; qed";

/// Serialize the given data structure as a pretty-printed String of JSON.
pub fn to_string_pretty<T: serde::Serialize + ?Sized>(value: &T) -> String {
	serde_json::to_string_pretty(value).expect(PROOF)
}

/// Serialize the given data structure as a JSON byte vector.
pub fn encode<T: serde::Serialize + ?Sized>(value: &T) -> Vec<u8> {
	serde_json::to_vec(value).expect(PROOF)
}

/// Serialize the given data structure as JSON into the IO stream.
pub fn to_writer<W: ::std::io::Write, T: serde::Serialize + ?Sized>(
	writer: W,
	value: &T,
) -> Result<()> {
	serde_json::to_writer(writer, value)
}