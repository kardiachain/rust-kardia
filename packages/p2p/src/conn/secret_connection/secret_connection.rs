use std::{error::Error, net::TcpStream};

use secp256k1::PublicKey;
use chacha20poly1305::ChaCha20Poly1305;
use x25519_dalek::{EphemeralSecret, PublicKey as EphemeralPublic};
use crate::conn::{secret_connection::kdf::Kdf};

/// Size of the MAC tag
pub const TAG_SIZE: usize = 16;

/// Maximum size of a message
pub const DATA_MAX_SIZE: usize = 1024;

/// 4 + 1024 == 1028 total frame size
const DATA_LEN_SIZE: usize = 4;
const TOTAL_FRAME_SIZE: usize = DATA_MAX_SIZE + DATA_LEN_SIZE;

/// Handshake is a process of establishing the `SecretConnection` between two peers.
pub struct Handshake<S> {
    state: S,
}

/// Handshake states

/// `AwaitingEphKey` means we're waiting for the remote ephemeral pubkey.
pub struct AwaitingEphKey {
    local_privkey: ed25519_consensus::SigningKey,
    local_eph_privkey: Option<EphemeralSecret>,
}

/// `AwaitingAuthSig` means we're waiting for the remote authenticated signature.
pub struct AwaitingAuthSig {
    sc_mac: [u8; 32],
    kdf: Kdf,
    recv_cipher: ChaCha20Poly1305,
    send_cipher: ChaCha20Poly1305,
    local_signature: ed25519_consensus::Signature,
}

#[allow(clippy::use_self)]
impl Handshake<AwaitingEphKey> {
    /// Initial a handshake.
    #[must_use]
    pub fn new(
        local_privkey: ed25519_consensus::SigningKey
    ) -> (Self, EphemeralPublic) {
        todo!()
    }

    /// Performs a Diffie-Hellman key agreement and creates a local signature.
    /// Transitions Handshake into `AwaitingAuthSig` state.
    ///
    /// # Errors
    ///
    /// * if protocol order was violated, e.g. handshake missing
    /// * if challenge signing fails
    pub fn got_key(
        &mut self, 
        remote_eph_pubkey: EphemeralPublic,
    ) -> Result<Handshake<AwaitingAuthSig>, Box<dyn Error>> {
        todo!()
    }
}

impl Handshake<AwaitingAuthSig> {
    /// Returns a verified pubkey of the remote peer.
    ///
    /// # Errors
    ///
    /// * if signature scheme isn't supported
    pub fn got_signature(
        &mut self,
        // TODO: change auth_sig_msg to proto AuthSigMessage
        auth_sig_msg: String,
    ) {
        todo!()
    }
}


/// =========================

// SecretConnection implements net.Conn.
// It is an implementation of the STS protocol.
//
// Consumers of the SecretConnection are responsible for authenticating
// the remote peer's pubkey against known information, like a nodeID.
// Otherwise they are vulnerable to MITM.
impl SecretConnection {
    /// Returns the remote pubkey. Panics if there's no key.
    pub fn remote_pubkey(&self) -> secp256k1::PublicKey {
        self.remote_pubkey.expect("remote_pubkey uninitialized")
    }

    pub fn write(&mut self, data: &[u8]) -> Result<i32, Box<dyn Error>> {
        todo!()
    }


}


pub struct SecretConnection {
    remote_pubkey: Option<PublicKey>,
}

/// Performs a handshake and returns a new `SecretConnection`.
///
/// # Errors
///
/// * if sharing of the pubkey fails
/// * if sharing of the signature fails
/// * if receiving the signature fails
pub fn make_secret_connection() {
    todo!()
}

