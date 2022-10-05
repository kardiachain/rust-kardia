use bytes::{Buf, Bytes};
use ethereum_types::Address;
use secp256k1::{
    ecdsa::{RecoverableSignature, RecoveryId},
    Error, Message as SecpMessage, PublicKey, SECP256K1,
};
use sha3::{Digest, Keccak256};

pub fn pub_to_address(pk: PublicKey) -> Address {
    let address_slice = &Keccak256::digest(&pk.serialize_uncompressed()[1..])[12..];
    Address::from_slice(address_slice)
}
