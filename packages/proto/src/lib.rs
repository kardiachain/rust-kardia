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
    use crate::consensus::{message::Sum, Message as ConsensusMessage};
    use crate::types;
    use prost::Message;

    /**
     Test bytes is constructed from
     ```
    commitSig := CommitSig{
         BlockIdFlag:      BlockIDFlagUnknown,
         ValidatorAddress: []byte("0x2a8Fd99c19271F4F04B1B7b9c4f7cF264b626eDB"),
         Timestamp:        time.Unix(1405544146, 0),
         Signature:        []byte("signature"),
     }
     ```
      */
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
        assert_eq!(
            decoded_commit_sig.timestamp.clone().unwrap().seconds,
            1405544146
        );
        assert_eq!(decoded_commit_sig.timestamp.unwrap().nanos, 0);
        assert_eq!(decoded_commit_sig.signature, b"signature");
    }

    #[test]
    /**
    Test bytes is constructed from
    ```
    msg := Message_NewRoundStep{
        &NewRoundStep{
            Height: 1,
            Round: 1,
            Step: 1,
            SecondsSinceStartTime: 1,
            LastCommitRound: 1,
        },
    }
    ```
     */
    fn decode_msg_new_round_step() {
        let msg_new_round_step: [u8; 12] = [10, 10, 8, 1, 16, 1, 24, 1, 32, 1, 40, 1];
        let rs = ConsensusMessage::decode(msg_new_round_step.as_slice());

        assert!(rs.is_ok());
        let decoded_msg = rs.unwrap();
        assert!(decoded_msg.sum.is_some());

        match decoded_msg.sum.unwrap() {
            Sum::NewRoundStep(nrs) => {
                assert_eq!(nrs.height, 1);
                assert_eq!(nrs.round, 1);
                assert_eq!(nrs.step, 1);
                assert_eq!(nrs.seconds_since_start_time, 1);
                assert_eq!(nrs.last_commit_round, 1);
            },
            _ => panic!("invalid message type")
        };
    }
}
