

/// Key Derivation Function for `SecretConnection` (HKDF)
pub struct Kdf {
    /// Receiver's secret
    pub recv_secret: [u8; 32],

    /// Sender's secret
    pub send_secret: [u8; 32],

    /// Challenge to be signed by peer
    pub challenge: [u8; 32],
}

impl Kdf {
    /// Returns recv secret, send secret, challenge as 32 byte arrays
    #[must_use]
    pub fn derive_secrets_and_challenge(shared_secret: &[u8; 32], loc_is_lo: bool) -> Self {
        todo!()
    }
}