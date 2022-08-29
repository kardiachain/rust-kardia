// use p256::{self, PublicKey, SecretKey, ecdsa::{self, Error}};
use secp256k1::{rand::rngs::OsRng, SecretKey, Secp256k1};
use std::{fs::File, io::Read, net::SocketAddrV4};
use hex;
use std::error::Error;
use crate::common::errors::Error::AddrError;
use std::borrow::Cow;

const MISSING_PORT : &str = "missing port in address";
const TO_MANY_COLONS : &str = "too many colons in address";

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
    let priv_key = SecretKey::from_slice(&d)?;
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

// split_host_port splits a network address of the form "host:port",
// "host%zone:port", "[host]:port" or "[host%zone]:port" into host or
// host%zone and port.
//
// A literal IPv6 address in hostport must be enclosed in square
// brackets, as in "[::1]:80", "[::1%lo0]:80".
//
// See func Dial for a description of the hostport parameter, and host
// and port results.
pub fn split_host_port(host_port: &str) -> Result<(&str, &str), Box<dyn Error>> {
    let mut host: &str = "";
    let mut port: &str;
    // The port starts after the last colon.
    match host_port.rfind(":") {
        None => {
            return Err(Box::new(AddrError { err: MISSING_PORT.to_string(), addr: host_port.to_string() }))
        }
        Some(_) => {
            if host_port.starts_with("[") {
                match host_port.find("]") {
                    Some(_) => {
                        let strip_prefix_host_port = host_port.strip_prefix("[").unwrap();
                        (host, port) = strip_prefix_host_port.split_at(strip_prefix_host_port.find("]:").expect("cannot find ']:' in this addr"));
                        port = port.trim_left_matches("]:")
                    },
                    None => return Err(Box::new(AddrError { err: "missing ']' in address".to_string(), addr: host_port.to_string() }))
                }
            } else {
                (host, port) = host_port.split_at(host_port.rfind(":").unwrap());
                port = port.trim_left_matches(":")
            }
        },
    }
    Ok((host, port))
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

    #[test]
    fn test_split_host_port_ok() {
        let hostport = "128.0.0.1:8080";
        assert_eq!(split_host_port(hostport).unwrap(), ("128.0.0.1", "8080"));

        let hostport2 = "[128.0.0.1]:8080";
        assert_eq!(split_host_port(hostport2).unwrap(), ("128.0.0.1", "8080"));
    }
}

