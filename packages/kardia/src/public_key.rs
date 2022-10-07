//! Public keys used in Tendermint networks

pub use ed25519_consensus::VerificationKey as Ed25519;
#[cfg(feature = "secp256k1")]
pub use k256::ecdsa::VerifyingKey as Secp256k1;

mod pub_key_request;
mod pub_key_response;
use core::{cmp::Ordering, convert::TryFrom, fmt, ops::Deref, str::FromStr};

pub use pub_key_request::PubKeyRequest;
pub use pub_key_response::PubKeyResponse;
use serde::{de, ser, Deserialize, Deserializer, Serialize};
use serde_json::Value;
#[cfg(feature = "secp256k1")]
use signature::Verifier as _;
// use subtle_encoding::{base64, bech32, hex};
// use tendermint_proto::{
//     crypto::{public_key::Sum, PublicKey as RawPublicKey},
//     Protobuf,
// };

use crate::{error::Error, prelude::*};

// Note:On the golang side this is generic in the sense that it could everything that implements
// github.com/tendermint/tendermint/crypto.PubKey
// While this is meant to be used with different key-types, it currently only uses a PubKeyEd25519
// version.
// TODO: make this more generic

// Warning: the custom serialization implemented here does not use TryFrom<RawPublicKey>.
//          it should only be used to read/write the priva_validator_key.json.
//          All changes to the serialization should check both the JSON and protobuf conversions.
// Todo: Merge JSON serialization with #[serde(try_from = "RawPublicKey", into = "RawPublicKey)]
/// Public keys allowed in Tendermint protocols
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "type", content = "value")] // JSON custom serialization for priv_validator_key.json
pub enum PublicKey {
    /// Ed25519 keys
    // #[serde(
    //     rename = "tendermint/PubKeyEd25519",
    //     serialize_with = "serialize_ed25519_base64",
    //     deserialize_with = "deserialize_ed25519_base64"
    // )]
    Ed25519(Ed25519),

    /// Secp256k1 keys
    #[cfg(feature = "secp256k1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
    #[serde(
        rename = "tendermint/PubKeySecp256k1",
        serialize_with = "serialize_secp256k1_base64",
        deserialize_with = "deserialize_secp256k1_base64"
    )]
    Secp256k1(Secp256k1),
}

// Internal thunk type to facilitate deserialization from the raw Protobuf data
// structure's JSON representation.
#[derive(Serialize, Deserialize)]
struct ProtobufPublicKeyWrapper {
    #[serde(rename = "Sum")]
    sum: ProtobufPublicKey,
}

impl From<ProtobufPublicKeyWrapper> for PublicKey {
    fn from(wrapper: ProtobufPublicKeyWrapper) -> Self {
        match wrapper.sum {
            ProtobufPublicKey::Ed25519 { ed25519 } => PublicKey::Ed25519(ed25519),
            #[cfg(feature = "secp256k1")]
            ProtobufPublicKey::Secp256k1 { secp256k1 } => PublicKey::Secp256k1(secp256k1),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "value")] // JSON custom serialization for priv_validator_key.json
enum ProtobufPublicKey {
    #[serde(rename = "tendermint.crypto.PublicKey_Ed25519")]
    Ed25519 {
        // #[serde(
        //     serialize_with = "serialize_ed25519_base64",
        //     deserialize_with = "deserialize_ed25519_base64"
        // )]
        ed25519: Ed25519,
    },

    #[cfg(feature = "secp256k1")]
    #[serde(rename = "tendermint.crypto.PublicKey_Secp256K1")]
    Secp256k1 {
        #[serde(
            serialize_with = "serialize_secp256k1_base64",
            deserialize_with = "deserialize_secp256k1_base64"
        )]
        secp256k1: Secp256k1,
    },
}

/// Custom deserialization for public keys to handle multiple potential JSON
/// formats from Tendermint.
///
/// See <https://github.com/informalsystems/tendermint-rs/issues/1021> for
/// context.
// TODO(thane): Remove this once the serialization in Tendermint has been fixed.
pub fn deserialize_public_key<'de, D>(deserializer: D) -> Result<PublicKey, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Value::deserialize(deserializer)?;
    if v.as_object()
        .map(|obj| obj.contains_key("Sum"))
        .unwrap_or(false)
    {
        serde_json::from_value::<ProtobufPublicKeyWrapper>(v).map(Into::into)
    } else {
        serde_json::from_value::<PublicKey>(v)
    }
    .map_err(serde::de::Error::custom)
}

// impl Protobuf<RawPublicKey> for PublicKey {}

// impl TryFrom<RawPublicKey> for PublicKey {
//     type Error = Error;

//     fn try_from(value: RawPublicKey) -> Result<Self, Self::Error> {
//         let sum = &value
//             .sum
//             .ok_or_else(|| Error::invalid_key("empty sum".to_string()))?;
//         if let Sum::Ed25519(b) = sum {
//             return Self::from_raw_ed25519(b)
//                 .ok_or_else(|| Error::invalid_key("malformed ed25519 key".to_string()));
//         }
//         #[cfg(feature = "secp256k1")]
//         if let Sum::Secp256k1(b) = sum {
//             return Self::from_raw_secp256k1(b)
//                 .ok_or_else(|| Error::invalid_key("malformed key".to_string()));
//         }
//         Err(Error::invalid_key("not an ed25519 key".to_string()))
//     }
// }

// impl From<PublicKey> for RawPublicKey {
//     fn from(value: PublicKey) -> Self {
//         match value {
//             PublicKey::Ed25519(ref pk) => RawPublicKey {
//                 sum: Some(tendermint_proto::crypto::public_key::Sum::Ed25519(
//                     pk.as_bytes().to_vec(),
//                 )),
//             },
//             #[cfg(feature = "secp256k1")]
//             PublicKey::Secp256k1(ref pk) => RawPublicKey {
//                 sum: Some(tendermint_proto::crypto::public_key::Sum::Secp256k1(
//                     pk.to_bytes().to_vec(),
//                 )),
//             },
//         }
//     }
// }

impl PublicKey {
    /// From raw secp256k1 public key bytes
    #[cfg(feature = "secp256k1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
    pub fn from_raw_secp256k1(bytes: &[u8]) -> Option<PublicKey> {
        Secp256k1::from_sec1_bytes(bytes)
            .ok()
            .map(PublicKey::Secp256k1)
    }

    /// From raw Ed25519 public key bytes
    pub fn from_raw_ed25519(bytes: &[u8]) -> Option<PublicKey> {
        Ed25519::try_from(bytes).map(PublicKey::Ed25519).ok()
    }

    /// Get Ed25519 public key
    pub fn ed25519(self) -> Option<Ed25519> {
        #[allow(unreachable_patterns)]
        match self {
            PublicKey::Ed25519(pk) => Some(pk),
            _ => None,
        }
    }

    /// Get Secp256k1 public key
    #[cfg(feature = "secp256k1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
    pub fn secp256k1(self) -> Option<Secp256k1> {
        match self {
            PublicKey::Secp256k1(pk) => Some(pk),
            _ => None,
        }
    }

    /// Verify the given [`Signature`] using this public key
    // pub fn verify(&self, msg: &[u8], signature: &Signature) -> Result<(), Error> {
    //     match self {
    //         PublicKey::Ed25519(pk) => {
    //             match ed25519_consensus::Signature::try_from(signature.as_bytes()) {
    //                 Ok(sig) => pk.verify(&sig, msg).map_err(|_| {
    //                     Error::signature_invalid(
    //                         "Ed25519 signature verification failed".to_string(),
    //                     )
    //                 }),
    //                 Err(_) => Err(Error::signature_invalid(
    //                     "Could not parse Ed25519 signature".to_string(),
    //                 )),
    //             }
    //         },
    //         #[cfg(feature = "secp256k1")]
    //         PublicKey::Secp256k1(pk) => {
    //             match k256::ecdsa::Signature::try_from(signature.as_bytes()) {
    //                 Ok(sig) => pk.verify(msg, &sig).map_err(|_| {
    //                     Error::signature_invalid(
    //                         "Secp256k1 signature verification failed".to_string(),
    //                     )
    //                 }),
    //                 Err(e) => Err(Error::signature_invalid(format!(
    //                     "invalid Secp256k1 signature: {}",
    //                     e
    //                 ))),
    //             }
    //         },
    //     }
    // }

    /// Serialize this key as a byte vector.
    pub fn to_bytes(self) -> Vec<u8> {
        match self {
            PublicKey::Ed25519(pk) => pk.as_bytes().to_vec(),
            #[cfg(feature = "secp256k1")]
            PublicKey::Secp256k1(pk) => pk.to_bytes().to_vec(),
        }
    }

    // Serialize this key as Bech32 with the given human readable prefix
    // pub fn to_bech32(self, hrp: &str) -> String {
    //     let backward_compatible_amino_prefixed_pubkey = match self {
    //         PublicKey::Ed25519(ref pk) => {
    //             let mut key_bytes = vec![0x16, 0x24, 0xDE, 0x64, 0x20];
    //             key_bytes.extend(pk.as_bytes());
    //             key_bytes
    //         },
    //         #[cfg(feature = "secp256k1")]
    //         PublicKey::Secp256k1(ref pk) => {
    //             let mut key_bytes = vec![0xEB, 0x5A, 0xE9, 0x87, 0x21];
    //             key_bytes.extend(pk.to_bytes());
    //             key_bytes
    //         },
    //     };
    //     bech32::encode(hrp, backward_compatible_amino_prefixed_pubkey)
    // }

    // /// Serialize this key as hexadecimal
    // pub fn to_hex(self) -> String {
    //     String::from_utf8(hex::encode_upper(self.to_bytes())).unwrap()
    // }
}

impl From<Ed25519> for PublicKey {
    fn from(pk: Ed25519) -> PublicKey {
        PublicKey::Ed25519(pk)
    }
}

#[cfg(feature = "secp256k1")]
impl From<Secp256k1> for PublicKey {
    fn from(pk: Secp256k1) -> PublicKey {
        PublicKey::Secp256k1(pk)
    }
}

impl PartialOrd for PublicKey {
    fn partial_cmp(&self, other: &PublicKey) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PublicKey {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            PublicKey::Ed25519(a) => match other {
                PublicKey::Ed25519(b) => a.as_bytes().cmp(b.as_bytes()),
                #[cfg(feature = "secp256k1")]
                PublicKey::Secp256k1(_) => Ordering::Less,
            },
            #[cfg(feature = "secp256k1")]
            PublicKey::Secp256k1(a) => match other {
                PublicKey::Ed25519(_) => Ordering::Greater,
                #[cfg(feature = "secp256k1")]
                PublicKey::Secp256k1(b) => a.cmp(b),
            },
        }
    }
}

/// Public key roles used in Tendermint networks
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum TendermintKey {
    /// User signing keys used for interacting with accounts in the state machine
    AccountKey(PublicKey),

    /// Validator signing keys used for authenticating consensus protocol messages
    ConsensusKey(PublicKey),
}

impl TendermintKey {
    /// Create a new account key from a [`PublicKey`]
    pub fn new_account_key(public_key: PublicKey) -> Result<TendermintKey, Error> {
        match public_key {
            PublicKey::Ed25519(_) => Ok(TendermintKey::AccountKey(public_key)),
            #[cfg(feature = "secp256k1")]
            PublicKey::Secp256k1(_) => Ok(TendermintKey::AccountKey(public_key)),
        }
    }

    /// Create a new consensus key from a [`PublicKey`]
    pub fn new_consensus_key(public_key: PublicKey) -> Result<TendermintKey, Error> {
        #[allow(unreachable_patterns)]
        match public_key {
            PublicKey::Ed25519(_) => Ok(TendermintKey::AccountKey(public_key)),
            _ => Err(Error::invalid_key(
                "only ed25519 consensus keys are supported".to_string(),
            )),
        }
    }

    /// Get the [`PublicKey`] value for this [`TendermintKey`]
    pub fn public_key(&self) -> &PublicKey {
        match self {
            TendermintKey::AccountKey(key) => key,
            TendermintKey::ConsensusKey(key) => key,
        }
    }
}

// TODO(tarcieri): deprecate/remove this in favor of `TendermintKey::public_key`
impl Deref for TendermintKey {
    type Target = PublicKey;

    fn deref(&self) -> &PublicKey {
        self.public_key()
    }
}

/// Public key algorithms
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Algorithm {
    /// ed25519
    Ed25519,

    /// secp256k1
    Secp256k1,
}

impl Algorithm {
    /// Get the string label for this algorithm
    pub fn as_str(&self) -> &str {
        match self {
            Algorithm::Ed25519 => "ed25519",
            Algorithm::Secp256k1 => "secp256k1",
        }
    }
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Algorithm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "ed25519" => Ok(Algorithm::Ed25519),
            "secp256k1" => Ok(Algorithm::Secp256k1),
            _ => Err(Error::parse(format!("invalid algorithm: {}", s))),
        }
    }
}

impl Serialize for Algorithm {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Algorithm {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use de::Error;
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}

/// Serialize the bytes of an Ed25519 public key as Base64. Used for serializing JSON
// fn serialize_ed25519_base64<S>(pk: &Ed25519, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: ser::Serializer,
// {
//     String::from_utf8(base64::encode(pk.as_bytes()))
//         .unwrap()
//         .serialize(serializer)
// }

/// Serialize the bytes of a secp256k1 ECDSA public key as Base64. Used for serializing JSON
#[cfg(feature = "secp256k1")]
fn serialize_secp256k1_base64<S>(pk: &Secp256k1, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    String::from_utf8(base64::encode(pk.to_bytes()))
        .unwrap()
        .serialize(serializer)
}

// fn deserialize_ed25519_base64<'de, D>(deserializer: D) -> Result<Ed25519, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     use de::Error;
//     let encoded = String::deserialize(deserializer)?;
//     let bytes = base64::decode(&encoded).map_err(D::Error::custom)?;
//     Ed25519::try_from(&bytes[..]).map_err(|_| D::Error::custom("invalid Ed25519 key"))
// }

#[cfg(feature = "secp256k1")]
fn deserialize_secp256k1_base64<'de, D>(deserializer: D) -> Result<Secp256k1, D::Error>
where
    D: Deserializer<'de>,
{
    use de::Error;
    let encoded = String::deserialize(deserializer)?;
    let bytes = base64::decode(&encoded).map_err(D::Error::custom)?;
    Secp256k1::from_sec1_bytes(&bytes).map_err(|_| D::Error::custom("invalid secp256k1 key"))
}