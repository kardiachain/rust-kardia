use core::fmt::{self, Display, Debug};
use std::str::FromStr;

use serde::{Serialize, Deserialize};
use subtle::{self, ConstantTimeEq};
use subtle_encoding::hex;

use crate::error::Error;

/// Length of a Node ID in bytes
pub const LENGTH: usize = 20;

/// Node IDs
#[allow(clippy::derive_hash_xor_eq)]
#[derive(Copy, Clone, Eq, Hash, PartialOrd, Ord, PartialEq, Serialize, Deserialize)]
pub struct Id([u8; LENGTH]);

impl Id {
    /// Create a new Node ID from raw bytes
    pub fn new(bytes: [u8; LENGTH]) -> Id {
        Id(bytes)
    }

    /// Borrow the node ID as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsRef<[u8]> for Id {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl ConstantTimeEq for Id {
    fn ct_eq(&self, other: &Id) -> subtle::Choice {
        self.as_bytes().ct_eq(other.as_bytes())
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "node::Id({})", self)
    }
}

/// Decode Node ID from hex
impl FromStr for Id {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Accept either upper or lower case hex
        let bytes = hex::decode_upper(s)
            .or_else(|_| hex::decode(s))
            .map_err(Error::subtle_encoding)?;

        if bytes.len() != LENGTH {
            return Err(Error::parse("invalid length".to_string()));
        }

        let mut result_bytes = [0u8; LENGTH];
        result_bytes.copy_from_slice(&bytes);
        Ok(Id(result_bytes))
    }
}
