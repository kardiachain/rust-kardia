use std::fmt::{self, Display, Debug};

/// Length of a Node ID in bytes
pub const LENGTH: usize = 20;

/// Node IDs
#[derive(Copy, Clone, Hash)]
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

// impl From<Ed25519> for Id {
//     fn from(pk: Ed25519) -> Id {
//         let digest = Sha256::digest(pk.as_bytes());
//         let mut bytes = [0u8; LENGTH];
//         bytes.copy_from_slice(&digest[..LENGTH]);
//         Id(bytes)
//     }
// }