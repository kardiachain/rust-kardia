use std::convert::TryInto;

use sha3::{Digest, Sha3_256};

pub const HASH_LENGTH: usize = 32;
pub const ADDRESS_LENGTH: usize = 20;

pub type Hash = [u8; HASH_LENGTH];

pub fn hash(b: &[u8]) -> Option<Hash> {
    let mut hasher = Sha3_256::new();
    hasher.update(b);
    Some(hasher.finalize()[..].try_into().expect("should not occur"))
}
