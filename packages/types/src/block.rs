use crate::{
    commit::Commit,
    evidence::EvidenceData,
    misc::Data,
    part_set::{PartSet, PartSetHeader},
};
use prost::Message;

#[derive(Clone, ::prost::Message)]
pub struct BlockId {
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub part_set_header: ::core::option::Option<PartSetHeader>,
}

impl BlockId {
    pub fn is_zero(&self) -> bool {
        self.hash.len() == 0 && self.part_set_header.is_none()
    }

    pub fn new_zero_block_id() -> BlockId {
        BlockId {
            hash: vec![],
            part_set_header: None,
        }
    }

    pub fn key(&self) -> String {
        let mut key = self.hash.clone();
        key.append(
            &mut self
                .part_set_header
                .clone()
                .map_or_else(|| vec![], |psh| psh.hash),
        );
        String::from_utf8(key).unwrap()
    }

    pub fn is_completed(&self) -> bool {
        return self.hash.len() > 0 && self.part_set_header.is_some_and(|psh| !psh.is_zero());
    }
}

impl PartialEq for BlockId {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash && self.part_set_header == other.part_set_header
    }
}

impl From<kai_proto::types::BlockId> for BlockId {
    fn from(m: kai_proto::types::BlockId) -> Self {
        Self {
            hash: m.hash,
            part_set_header: m.part_set_header.map(|psh| psh.into()),
        }
    }
}

impl Into<kai_proto::types::BlockId> for BlockId {
    fn into(self) -> kai_proto::types::BlockId {
        kai_proto::types::BlockId {
            hash: self.hash,
            part_set_header: self.part_set_header.map(|psh| psh.into()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlockIdFlag {
    Unknown = 0,
    Absent = 1,
    Commit = 2,
    Nil = 3,
}

#[derive(Debug, Clone)]
pub struct BlockMeta {
    pub block_id: BlockId,
    pub header: Header,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    /// basic block info
    #[prost(string, tag = "2")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub height: u64,
    #[prost(uint64, tag = "15")]
    pub gas_limit: u64,
    #[prost(message, optional, tag = "4")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// prev block info
    #[prost(message, optional, tag = "5")]
    pub last_block_id: ::core::option::Option<BlockId>,
    /// hashes of block data
    ///
    /// commit from validators from the last block
    #[prost(bytes = "vec", tag = "6")]
    pub last_commit_hash: ::prost::alloc::vec::Vec<u8>,
    /// transactions
    #[prost(bytes = "vec", tag = "7")]
    pub data_hash: ::prost::alloc::vec::Vec<u8>,
    /// hashes from the app output from the prev block
    ///
    /// validators for the current block
    #[prost(bytes = "vec", tag = "8")]
    pub validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// validators for the next block
    #[prost(bytes = "vec", tag = "9")]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// consensus params for current block
    #[prost(bytes = "vec", tag = "10")]
    pub consensus_hash: ::prost::alloc::vec::Vec<u8>,
    /// state after txs from the previous block
    #[prost(bytes = "vec", tag = "11")]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
    /// consensus info
    ///
    /// evidence included in the block
    #[prost(bytes = "vec", tag = "13")]
    pub evidence_hash: ::prost::alloc::vec::Vec<u8>,
    /// original proposer of the block
    #[prost(bytes = "vec", tag = "14")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "16")]
    pub num_txs: u64,
}

impl Header {
    pub fn hash(&self) -> Option<crate::common::hash::Hash> {
        let binding = ::prost::Message::encode_to_vec(self);
        let pbh = binding.as_slice();
        crate::common::hash::hash(pbh)
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<Data>,
    #[prost(message, optional, tag = "3")]
    pub evidence: ::core::option::Option<EvidenceData>,
    #[prost(message, optional, tag = "4")]
    pub last_commit: ::core::option::Option<Commit>,
}

impl Block {
    pub fn hash(&self) -> Option<crate::common::hash::Hash> {
        self.header.clone().and_then(|h| h.hash())
    }

    pub fn make_part_set(&self, part_size: u32) -> PartSet {
        PartSet::new_from_block(self)
    }
}
