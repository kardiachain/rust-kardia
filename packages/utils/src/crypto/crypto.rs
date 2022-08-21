// use p256::{self, PublicKey, SecretKey, ecdsa::{self, Error}};
use secp256k1::{SecretKey, PublicKey, Secp256k1};
use std::{fs::File, io::Read};
use crate::common::types::Address;
use hex;
use elliptic_curve;


// keccakState wraps sha3.state. In addition to the usual hash methods, it also supports
// Read to get a variable amount of data from the hash state. Read is faster than Sum
// because it doesn't copy the internal state, but also modifies the internal state.
pub trait KeccakState {
}

pub fn pubkey_to_address(p : PublicKey) -> Address {
    todo!();
}

// LoadECDSA loads a secp256k1 private key from the given file.
pub fn load_ecdsa(file : &str) -> SecretKey {
    let mut buf : Vec<u8> = Vec::new();
    let mut fd = File::open(file).unwrap();
    let _ = fd.read(&mut buf[..64]);
    let key = hex::decode(buf).unwrap();
    
    to_ecdsa(key).unwrap()
}

// ToECDSA creates a private key with the given D value. The strict parameter
// controls whether the key's length should be enforced at the curve size or
// it can also accept legacy encodings (0 prefixes).
pub fn to_ecdsa(d: Vec<u8>) -> Result<SecretKey, Box<dyn std::error::Error>> {
    // let priv_key : secp256k1::SecretKey;
    
    // let secp = Secp256k1::new();
    let priv_key = SecretKey::from_slice(&d).expect("32 bytes, within curve order");
    Ok(priv_key)
}