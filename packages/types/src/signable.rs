use bytes::Bytes;
use ethereum_types::Address;
use kai_lib::{crypto::pub_to_address, signature::sig_to_pub};

pub fn verify_signature(addr: Address, hash: Bytes, signature: Bytes) -> bool {
    if let Ok(pk) = sig_to_pub(hash, signature) {
        return addr == pub_to_address(pk);
    } else {
        false
    }
}
