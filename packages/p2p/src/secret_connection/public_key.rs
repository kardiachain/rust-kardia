//! Secret Connection peer public keys

use std::{
    convert::TryFrom,
    fmt::{self, Display},
};

use k256::ecdsa;
use sha2::{digest::Digest, Sha256};
use kai_core::{error::Error, node};

/// Secret Connection peer public keys (signing, presently Secp256k1-only)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PublicKey {
    /// Secp256k1 Secret Connection Keys
    Secp256k1(k256::ecdsa::VerifyingKey)
}

impl PublicKey {

    /// From raw Secp259k1 public key bytes
    ///
    /// # Errors
    ///
    /// * if the bytes given are invalid
    pub fn from_raw_secp256k1(bytes: &[u8]) -> Result<Self, Error> {
        k256::ecdsa::VerifyingKey::from_sec1_bytes(bytes)
            .map(Self::Secp256k1)
            .map_err(|_| Error::signature())
    }

    /// Get Secp256k1 public key
    #[must_use]
    pub const fn secp256k1(self) -> Option<k256::ecdsa::VerifyingKey> {
        match self {
            Self::Secp256k1(pk) => Some(pk),
        }
    }

    /// Get the remote Peer ID
    #[must_use]
    pub fn peer_id(self) -> node::Id {
        match self {
            Self::Secp256k1(pk) => {
                let digest = Sha256::digest(pk.to_bytes().as_slice());
                let mut bytes = [0_u8; 20];
                bytes.copy_from_slice(&digest[..20]);
                node::Id::new(bytes)
            },
        }
    }
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.peer_id())
    }
}

impl From<&k256::ecdsa::SigningKey> for PublicKey {
    fn from(sk: &k256::ecdsa::SigningKey) -> Self {
        Self::Secp256k1(ecdsa::VerifyingKey::from(sk))
    }
}

impl From<k256::ecdsa::VerifyingKey> for PublicKey {
    fn from(pk: k256::ecdsa::VerifyingKey) -> Self {
        Self::Secp256k1(pk)
    }
}
