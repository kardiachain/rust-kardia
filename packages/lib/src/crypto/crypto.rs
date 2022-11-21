use bytes::Bytes;
use ethereum_types::Address;
use secp256k1::PublicKey;
use sha3::{Digest, Keccak256};

pub fn pub_to_address(pk: PublicKey) -> Address {
    let address_slice = &Keccak256::digest(&pk.serialize_uncompressed()[1..])[12..];
    Address::from_slice(address_slice)
}

pub fn keccak256(b: Bytes) -> Bytes {
    Bytes::copy_from_slice(&Keccak256::digest(b).as_slice())
}
