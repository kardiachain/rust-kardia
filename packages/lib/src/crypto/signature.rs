use bytes::{Buf, Bytes};
use ethereum_types::Address;
use secp256k1::{
    ecdsa::{RecoverableSignature, RecoveryId},
    Error, Message as SecpMessage, PublicKey, SecretKey, SECP256K1,
};
use sha3::{Digest, Keccak256};

use super::crypto::pub_to_address;

pub fn sig_to_pub(hash: Bytes, sig: Bytes) -> Result<PublicKey, Error> {
    let _sig = &sig[0..64];
    let _rec = RecoveryId::from_i32(sig[64] as i32)?;
    let signature = RecoverableSignature::from_compact(_sig, _rec)?;

    let message = &SecpMessage::from_slice(hash.to_vec().as_slice())?;
    SECP256K1.recover_ecdsa(message, &signature)
}

pub fn sign(hash: Bytes, sk: SecretKey) -> Result<Bytes, Error> {
    let (rec_id, sig) = SECP256K1
        .sign_ecdsa_recoverable(
            &secp256k1::Message::from_slice(hash.to_vec().as_slice())?,
            &sk,
        )
        .serialize_compact();
    let mut signature = Vec::from(sig);
    // convert to Kardia signature format with 'recovery id' v at the end.
    signature.push(rec_id.to_i32().try_into().unwrap());
    return Ok(Bytes::from(signature));
}

pub fn verify_signature(addr: Address, hash: Bytes, signature: Bytes) -> bool {
    if let Ok(pk) = sig_to_pub(hash, signature) {
        return addr == pub_to_address(pk);
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::sign;

    #[test]
    fn sign_test() {
        let signer_secret_key = secp256k1::SecretKey::new(&mut secp256k1::rand::thread_rng());
        
        let msg: [u8; 32] = [0; 32];
        let rs = sign(Bytes::copy_from_slice(&msg), signer_secret_key);
        assert!(rs.is_ok());
        let _ = rs.unwrap().to_vec();

        let invalid_msg: [u8; 34] = [0; 34];
        let rs = sign(Bytes::copy_from_slice(&invalid_msg), signer_secret_key);
        assert!(matches!(rs.err().unwrap(), secp256k1::Error::InvalidMessage));
    }
}
