use bytes::Bytes;
use ethereum_types::Address;
use kai_lib::crypto::{crypto::keccak256, signature::verify_signature};
use prost::Message;

use crate::{
    block::{BlockId, BlockIdFlag},
    canonical_types::create_canonical_vote,
    commit::CommitSig,
    consensus::state::ChainId,
    errors::VoteError,
    types::SignedMsgType,
};

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vote {
    #[prost(enumeration = "SignedMsgType", tag = "1")]
    pub r#type: i32,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    #[prost(uint32, tag = "3")]
    pub round: u32,
    /// zero if vote is nil.
    #[prost(message, optional, tag = "4")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(message, optional, tag = "5")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes = "vec", tag = "6")]
    pub validator_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "7")]
    pub validator_index: u32,
    #[prost(bytes = "vec", tag = "8")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}

impl From<kai_proto::types::Vote> for Vote {
    fn from(m: kai_proto::types::Vote) -> Self {
        Self {
            r#type: match m.r#type {
                1 => SignedMsgType::Prevote,
                2 => SignedMsgType::Precommit,
                32 => SignedMsgType::Proposal,
                _ => SignedMsgType::Unknown,
            }
            .into(),
            height: m.height,
            round: m.round,
            block_id: m.block_id.map(|x| x.into()),
            timestamp: m.timestamp,
            validator_address: m.validator_address,
            validator_index: m.validator_index,
            signature: m.signature,
        }
    }
}

impl Into<kai_proto::types::Vote> for Vote {
    fn into(self) -> kai_proto::types::Vote {
        kai_proto::types::Vote {
            r#type: self.r#type.into(),
            height: self.height,
            round: self.round,
            block_id: self.block_id.map(|x| x.into()),
            timestamp: self.timestamp,
            validator_address: self.validator_address,
            validator_index: self.validator_index,
            signature: self.signature,
        }
    }
}

pub fn is_valid_vote_type(t: SignedMsgType) -> bool {
    match t {
        SignedMsgType::Precommit => true,
        SignedMsgType::Prevote => true,
        _ => false,
    }
}

impl Vote {
    pub fn verify(&self, chain_id: ChainId, validator_address: Address) -> Result<(), VoteError> {
        let psb = self.vote_sign_bytes(chain_id);
        if psb.is_none() {
            return Err(VoteError::CreateVoteSignBytesError);
        }

        if !verify_signature(
            validator_address,
            keccak256(psb.unwrap()),
            Bytes::from(self.clone().signature),
        ) {
            return Err(VoteError::InvalidSignature);
        }

        return Ok(());
    }

    pub fn vote_sign_bytes(&self, chain_id: ChainId) -> Option<Bytes> {
        create_canonical_vote(chain_id, self.clone())
            .map(|cv| Bytes::copy_from_slice(&cv.encode_length_delimited_to_vec()))
    }

    pub fn commit_sig(&self) -> CommitSig {
        let block_id_flag = if self.block_id.is_some_and(|bid| bid.is_completed()) {
            BlockIdFlag::Commit
        } else if self.block_id.is_some_and(|bid| bid.is_zero()) {
            BlockIdFlag::Nil
        } else {
            panic!(
                "Invalid vote {:?} - expected BlockID to be either empty or complete",
                self
            );
        };

        return CommitSig {
            block_id_flag: block_id_flag.into(),
            validator_address: self.validator_address.clone(),
            timestamp: self.timestamp.clone(),
            signature: self.signature.clone(),
        };
    }
}
