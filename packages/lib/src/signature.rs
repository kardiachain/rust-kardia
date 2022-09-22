use bytes::{Buf, Bytes};
use secp256k1::{
    ecdsa::{RecoverableSignature, RecoveryId},
    Error, Message as SecpMessage, PublicKey, SECP256K1,
};
use sha3::{Digest, Keccak256};

pub fn sig_to_pub(hash: Bytes, sig: Bytes) -> Result<PublicKey, Error> {
    let _sig = &sig[0..64];
    let _rec = RecoveryId::from_i32(sig[64] as i32)?;
    let signature = RecoverableSignature::from_compact(_sig, _rec)?;

    let message = &SecpMessage::from_slice(hash.chunk())?;
    SECP256K1.recover_ecdsa(message, &signature)
}
