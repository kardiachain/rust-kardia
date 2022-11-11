#![no_std]
extern crate alloc;

use core::{
    convert::{TryFrom, TryInto},
    fmt::Display,
};

use alloc::vec::Vec;
use bytes::{Buf, BufMut};
pub use error::Error;
use prost::{encoding::encoded_len_varint, Message};

pub mod blockchain {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.blockchain.rs"));
}

pub mod consensus {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.consensus.rs"));
}

pub mod crypto {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.crypto.rs"));
}

pub mod evidence {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.evidence.rs"));
}

pub mod p2p {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.p2p.rs"));
}

pub mod state {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.state.rs"));
}

pub mod types {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.types.rs"));
}

pub mod error;
pub mod prelude;

#[cfg(test)]
mod tests {
    use crate::types;
    use prost::Message;

    #[test]
    fn decode_goproto_commit_sig() {
        let b_commit_sig: [u8; 63] = [
            18, 42, 48, 120, 50, 97, 56, 70, 100, 57, 57, 99, 49, 57, 50, 55, 49, 70, 52, 70, 48,
            52, 66, 49, 66, 55, 98, 57, 99, 52, 102, 55, 99, 70, 50, 54, 52, 98, 54, 50, 54, 101,
            68, 66, 26, 6, 8, 210, 205, 155, 158, 5, 34, 9, 115, 105, 103, 110, 97, 116, 117, 114,
            101,
        ];
        let decoded_commit_sig = types::CommitSig::decode(b_commit_sig.as_slice()).unwrap();

        assert_eq!(decoded_commit_sig.block_id_flag, 0);
        assert_eq!(
            decoded_commit_sig.validator_address,
            b"0x2a8Fd99c19271F4F04B1B7b9c4f7cF264b626eDB"
        );
        assert_eq!(decoded_commit_sig.timestamp.clone().unwrap().seconds, 1405544146);
        assert_eq!(decoded_commit_sig.timestamp.unwrap().nanos, 0);
        assert_eq!(decoded_commit_sig.signature, b"signature");
    }
}

pub trait Protobuf<T: Message + From<Self> + Default>
where
    Self: Sized + Clone + TryFrom<T>,
    <Self as TryFrom<T>>::Error: Display,
{
    /// Encode into a buffer in Protobuf format.
    ///
    /// Uses [`prost::Message::encode`] after converting into its counterpart
    /// Protobuf data structure.
    ///
    /// [`prost::Message::encode`]: https://docs.rs/prost/*/prost/trait.Message.html#method.encode
    fn encode<B: BufMut>(&self, buf: &mut B) -> Result<(), Error> {
        T::from(self.clone())
            .encode(buf)
            .map_err(Error::encode_message)
    }

    /// Encode with a length-delimiter to a buffer in Protobuf format.
    ///
    /// An error will be returned if the buffer does not have sufficient capacity.
    ///
    /// Uses [`prost::Message::encode_length_delimited`] after converting into
    /// its counterpart Protobuf data structure.
    ///
    /// [`prost::Message::encode_length_delimited`]: https://docs.rs/prost/*/prost/trait.Message.html#method.encode_length_delimited
    fn encode_length_delimited<B: BufMut>(&self, buf: &mut B) -> Result<(), Error> {
        T::from(self.clone())
            .encode_length_delimited(buf)
            .map_err(Error::encode_message)
    }

    /// Constructor that attempts to decode an instance from a buffer.
    ///
    /// The entire buffer will be consumed.
    ///
    /// Similar to [`prost::Message::decode`] but with additional validation
    /// prior to constructing the destination type.
    ///
    /// [`prost::Message::decode`]: https://docs.rs/prost/*/prost/trait.Message.html#method.decode
    fn decode<B: Buf>(buf: B) -> Result<Self, Error> {
        let raw = T::decode(buf).map_err(Error::decode_message)?;

        Self::try_from(raw).map_err(Error::try_from::<T, Self, _>)
    }

    /// Constructor that attempts to decode a length-delimited instance from
    /// the buffer.
    ///
    /// The entire buffer will be consumed.
    ///
    /// Similar to [`prost::Message::decode_length_delimited`] but with
    /// additional validation prior to constructing the destination type.
    ///
    /// [`prost::Message::decode_length_delimited`]: https://docs.rs/prost/*/prost/trait.Message.html#method.decode_length_delimited
    fn decode_length_delimited<B: Buf>(buf: B) -> Result<Self, Error> {
        let raw = T::decode_length_delimited(buf).map_err(Error::decode_message)?;

        Self::try_from(raw).map_err(Error::try_from::<T, Self, _>)
    }

    /// Returns the encoded length of the message without a length delimiter.
    ///
    /// Uses [`prost::Message::encoded_len`] after converting to its
    /// counterpart Protobuf data structure.
    ///
    /// [`prost::Message::encoded_len`]: https://docs.rs/prost/*/prost/trait.Message.html#method.encoded_len
    fn encoded_len(&self) -> usize {
        T::from(self.clone()).encoded_len()
    }

    /// Encodes into a Protobuf-encoded `Vec<u8>`.
    fn encode_vec(&self) -> Vec<u8> {
        let mut wire = Vec::with_capacity(self.encoded_len());
        self.encode(&mut wire)
            .map(|_| wire)
            .expect("buffer size too small")
    }

    /// Constructor that attempts to decode a Protobuf-encoded instance from a
    /// `Vec<u8>` (or equivalent).
    fn decode_vec(v: &[u8]) -> Result<Self, Error> {
        Self::decode(v)
    }

    /// Encode with a length-delimiter to a `Vec<u8>` Protobuf-encoded message.
    fn encode_length_delimited_vec(&self) -> Result<Vec<u8>, Error> {
        let len = self.encoded_len();
        let lenu64 = len.try_into().map_err(Error::parse_length)?;
        let mut wire = Vec::with_capacity(len + encoded_len_varint(lenu64));
        self.encode_length_delimited(&mut wire).map(|_| wire)
    }

    /// Constructor that attempts to decode a Protobuf-encoded instance with a
    /// length-delimiter from a `Vec<u8>` or equivalent.
    fn decode_length_delimited_vec(v: &[u8]) -> Result<Self, Error> {
        Self::decode_length_delimited(v)
    }
}
