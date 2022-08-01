pub mod blockchain {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.blockchain.rs"));
}

pub mod consensus {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.consensus.rs"));
}

pub mod crypto {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.crypto.rs"));
}

pub mod evidence {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.evidence.rs"));
}

pub mod p2p {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.p2p.rs"));
}

pub mod state {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.state.rs"));
}

pub mod types {
    include!(concat!(env!("OUT_DIR"), "/kardiachain.types.rs"));
}

#[cfg(test)]
mod tests {
    use crate::types;
    use prost::Message;

    #[test]
    fn decode_goproto_commit_sig() {
        let b_commit_sig: [u8; 63] = [
            18, 42, 48, 120, 50, 97, 56, 70, 100, 57, 57, 99, 49, 57, 50, 55, 49, 70, 52, 70, 48,
            52, 66, 49, 66, 55, 98, 57, 99, 52, 102, 55, 99, 70, 50, 54, 52, 98, 54, 50, 54, 101,
            68, 66, 26, 6, 8, 210, 205, 155, 158, 5, 34, 9, 115, 105, 103, 110, 97, 116, 117, 114,
            101,
        ];
        let decoded_commit_sig = types::CommitSig::decode(b_commit_sig.as_slice()).unwrap();

        assert_eq!(decoded_commit_sig.block_id_flag, 0);
        assert_eq!(
            decoded_commit_sig.validator_address,
            b"0x2a8Fd99c19271F4F04B1B7b9c4f7cF264b626eDB"
        );
        assert_eq!(decoded_commit_sig.timestamp.clone().unwrap().seconds, 1405544146);
        assert_eq!(decoded_commit_sig.timestamp.unwrap().nanos, 0);
        assert_eq!(decoded_commit_sig.signature, b"signature");
    }
}
