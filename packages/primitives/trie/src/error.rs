use kp_std::{boxed::Box, vec::Vec};

/// Error type used for trie related errors.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum Error<H> {
	#[cfg_attr(feature = "std", error("Bad format"))]
	BadFormat,
	#[cfg_attr(feature = "std", error("Decoding failed: {0}"))]
	Decode(#[cfg_attr(feature = "std", source)] codec::Error),
	#[cfg_attr(
		feature = "std",
		error("Recorded key ({0:x?}) access with value as found={1}, but could not confirm with trie.")
	)]
	InvalidRecording(Vec<u8>, bool),
	#[cfg_attr(feature = "std", error("Trie error: {0:?}"))]
	TrieError(Box<trie_db::TrieError<H, Self>>),
}

impl<H> From<codec::Error> for Error<H> {
	fn from(x: codec::Error) -> Self {
		Error::Decode(x)
	}
}

impl<H> From<Box<trie_db::TrieError<H, Self>>> for Error<H> {
	fn from(x: Box<trie_db::TrieError<H, Self>>) -> Self {
		Error::TrieError(x)
	}
}