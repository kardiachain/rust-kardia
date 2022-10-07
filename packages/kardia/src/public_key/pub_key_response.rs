use super::PublicKey;

/// PubKeyResponse
#[derive(Clone, PartialEq, Debug)]
// Todo: either pub_key OR error is present
pub struct PubKeyResponse {
    /// Public key
    pub pub_key: Option<PublicKey>,

    // Error
    // pub error: Option<RemoteSignerError>,
}