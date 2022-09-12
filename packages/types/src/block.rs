use crate::{part_set::PartSetHeader, misc::Data, evidence::EvidenceData, commit::Commit};

#[derive(Default, Debug, Clone)]
pub struct BlockId {
    pub hash: Vec<u8>,
    pub parts_header: Option<PartSetHeader>,
}

impl From<kai_proto::types::BlockId> for BlockId {
    fn from(m: kai_proto::types::BlockId) -> Self {
        Self {
            hash: m.hash,
            parts_header: m.part_set_header.map(|psh| psh.into()),
        }
    }
}

impl Into<kai_proto::types::BlockId> for BlockId {
    fn into(self) -> kai_proto::types::BlockId {
        kai_proto::types::BlockId {
            hash: self.hash,
            part_set_header: self.parts_header.map(|psh| psh.into()),
        }
    }
}

impl PartialEq for BlockId {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash && self.parts_header == other.parts_header
    }
}

#[derive(Debug, Clone)]
pub struct BlockMeta {
    pub block_id: BlockId,
    pub header: Header,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub chain_id: String,
    pub height: u64,
    pub gas_limit: u64,
    pub time: Option<prost_types::Timestamp>,
    pub last_block_id: Option<BlockId>,
    pub last_commit_hash: Vec<u8>,
    pub data_hash: Vec<u8>,
    pub validators_hash: Vec<u8>,
    pub next_validators_hash: Vec<u8>,
    pub consensus_hash: Vec<u8>,
    pub app_hash: Vec<u8>,
    pub evidence_hash: Vec<u8>,
    pub proposer_address: Vec<u8>,
    pub num_txs: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag="2")]
    pub data: ::core::option::Option<Data>,
    #[prost(message, optional, tag="3")]
    pub evidence: ::core::option::Option<EvidenceData>,
    #[prost(message, optional, tag="4")]
    pub last_commit: ::core::option::Option<Commit>,
}