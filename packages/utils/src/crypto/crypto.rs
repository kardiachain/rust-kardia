// use p256::{self, PublicKey, SecretKey, ecdsa::{self, Error}};
use secp256k1::{rand::rngs::OsRng, SecretKey, Secp256k1};
use std::{fs::File, io::Read};
use hex;
use std::error::Error;


// keccakState wraps sha3.state. In addition to the usual hash methods, it also supports
// Read to get a variable amount of data from the hash state. Read is faster than Sum
// because it doesn't copy the internal state, but also modifies the internal state.
pub trait KeccakState {
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
    let priv_key = SecretKey::from_slice(&d).expect("32 bytes, within curve order");
    Ok(priv_key)
}

// HexToECDSA parses a secp256k1 private key.
pub fn hex_to_ecdsa(hexkey: String) -> Result<secp256k1::SecretKey, Box<dyn Error>> {
    let b = hex::decode(hexkey).expect("invalid hex string");
    to_ecdsa(b)
}

pub fn generate_key() -> secp256k1::SecretKey {
    let secp = Secp256k1::new();
    let (secret_key, _) = secp.generate_keypair(&mut OsRng);
    
    secret_key
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_generate_key_ok() {
        let sec = generate_key();
        println!("{}", sec.display_secret())
    }
}

