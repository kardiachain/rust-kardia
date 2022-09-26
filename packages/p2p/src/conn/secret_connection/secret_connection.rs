// use std::{error::Error, net::TcpStream};

use p256::elliptic_curve::subtle::ConstantTimeEq;
use secp256k1::PublicKey;
use chacha20poly1305::{
    ChaCha20Poly1305, aead::NewAead,
};
use x25519_dalek::{EphemeralSecret, PublicKey as EphemeralPublic};
use crate::conn::{secret_connection::{kdf::Kdf, error::Error, protocol::Version}};
use merlin::Transcript;

use rand_core::OsRng;

/// Size of the MAC tag
pub const TAG_SIZE: usize = 16;

/// Maximum size of a message
pub const DATA_MAX_SIZE: usize = 1024;

/// 4 + 1024 == 1028 total frame size
const DATA_LEN_SIZE: usize = 4;
const TOTAL_FRAME_SIZE: usize = DATA_MAX_SIZE + DATA_LEN_SIZE;

/// Handshake is a process of establishing the `SecretConnection` between two peers.
pub struct Handshake<S> {
    protocol_version: Version,
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
        local_privkey: ed25519_consensus::SigningKey,
        protocol_version: Version,
    ) -> (Self, EphemeralPublic) {
        // Generate an ephemeral key for perfect forward secrecy.
        let local_eph_privkey = EphemeralSecret::new(&mut OsRng);
        let local_eph_pubkey = EphemeralPublic::from(&local_eph_privkey);

        (
            Self {
                protocol_version,
                state: AwaitingEphKey {
                    local_privkey,
                    local_eph_privkey: Some(local_eph_privkey),
                },
            },
            local_eph_pubkey,
        )
    }

    /// Performs a Diffie-Hellman key agreement and creates a local signature.
    /// Transitions Handshake into `AwaitingAuthSig` state.
    ///
    /// # Errors
    ///use merlin::Transcript;
    /// * if protocol order was violated, e.g. handshake missing
    /// * if challenge signing fails
    pub fn got_key(
        &mut self, 
        remote_eph_pubkey: EphemeralPublic,
    ) -> Result<Handshake<AwaitingAuthSig>, Error> {
        let local_eph_privkey = match self.state.local_eph_privkey.take() {
            Some(key) => key,
            None => return Err(Error::missing_secret()),
        };
        let local_eph_pubkey = EphemeralPublic::from(&local_eph_privkey);

        // Compute common shared secret.
        let shared_secret = EphemeralSecret::diffie_hellman(local_eph_privkey, &remote_eph_pubkey);

        let mut transcript = Transcript::new(b"KARDIA_SECRET_CONNECTION_TRANSCRIPT_HASH");

        // Reject all-zero outputs from X25519 (i.e. from low-order points)
        //
        // See the following for information on potential attacks this check
        // aids in mitigating:
        //
        // - https://github.com/tendermint/kms/issues/142
        // - https://eprint.iacr.org/2019/526.pdf
        if shared_secret.as_bytes().ct_eq(&[0x00; 32]).unwrap_u8() == 1 {
            return  Err(Error::low_order_key());
        }

        // Sort by lexical order.
        let local_eph_pubkey_bytes = *local_eph_pubkey.as_bytes();
        let (low_eph_pubkey_bytes, high_eph_pubkey_bytes) =
            sort32(local_eph_pubkey_bytes, *remote_eph_pubkey.as_bytes());

        transcript.append_message(b"EPHEMERAL_LOWER_PUBLIC_KEY", &low_eph_pubkey_bytes);
        transcript.append_message(b"EPHEMERAL_UPPER_PUBLIC_KEY", &high_eph_pubkey_bytes);
        transcript.append_message(b"DH_SECRET", shared_secret.as_bytes());

        // Check if the local ephemeral public key was the least, lexicographically sorted.
        let loc_is_least = local_eph_pubkey_bytes == low_eph_pubkey_bytes;

        let kdf = Kdf::derive_secrets_and_challenge(shared_secret.as_bytes(), loc_is_least);

        let mut sc_mac: [u8; 32] = [0; 32];

        transcript.challenge_bytes(b"SECRET_CONNECTION_MAC", &mut sc_mac);

        // Sign the challenge bytes for authentication.
        let local_signature = if self.protocol_version.has_transcript() {
            self.state.local_privkey.sign(&sc_mac)
        } else {
            self.state.local_privkey.sign(&kdf.challenge)
        };

        Ok(Handshake {
            protocol_version: self.protocol_version,
            state: AwaitingAuthSig {
                sc_mac,
                recv_cipher: ChaCha20Poly1305::new(&kdf.recv_secret.into()),
                send_cipher: ChaCha20Poly1305::new(&kdf.send_secret.into()),
                kdf,
                local_signature,
            },
        })
    }
}

impl Handshake<AwaitingAuthSig> {
    /// Returns a verified pubkey of the remote peer.
    ///
    /// # Errors
    ///
    /// * if signature scheme isn't supported
    pub fn got_signature (
        &mut self,
        // TODO: change auth_sig_msg to proto AuthSigMessage.
        auth_sig_msg: String,
    ) {
        todo!()
    }
}

/// Return is of the form lo, hi
#[must_use]
pub fn sort32(first: [u8; 32], second: [u8; 32]) -> ([u8; 32], [u8; 32]) {
    if second > first {
        (first, second)
    } else {
        (second, first)
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

    pub fn write(&mut self, data: &[u8]) -> Result<i32, Error> {
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

