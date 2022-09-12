#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSet {
    #[prost(message, repeated, tag="1")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    #[prost(message, optional, tag="2")]
    pub proposer: ::core::option::Option<Validator>,
    #[prost(int64, tag="3")]
    pub total_voting_power: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validator {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub voting_power: i64,
    #[prost(int64, tag="4")]
    pub proposer_priority: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleValidator {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub voting_power: i64,
}
/// PartsetHeader
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartSetHeader {
    #[prost(uint32, tag="1")]
    pub total: u32,
    #[prost(bytes="vec", tag="2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Part {
    #[prost(uint32, tag="1")]
    pub index: u32,
    #[prost(bytes="vec", tag="2")]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub proof: ::core::option::Option<super::cryptox::Proof>,
}
/// BlockID
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockId {
    #[prost(bytes="vec", tag="1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub part_set_header: ::core::option::Option<PartSetHeader>,
}
// --------------------------------

// Header defines the structure of a Kardiachain block header.
// --------------------------------

/// Header defines the structure of a block header.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    /// basic block info
    #[prost(string, tag="2")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub height: u64,
    #[prost(uint64, tag="15")]
    pub gas_limit: u64,
    #[prost(message, optional, tag="4")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// prev block info
    #[prost(message, optional, tag="5")]
    pub last_block_id: ::core::option::Option<BlockId>,
    /// hashes of block data
    ///
    /// commit from validators from the last block
    #[prost(bytes="vec", tag="6")]
    pub last_commit_hash: ::prost::alloc::vec::Vec<u8>,
    /// transactions
    #[prost(bytes="vec", tag="7")]
    pub data_hash: ::prost::alloc::vec::Vec<u8>,
    /// hashes from the app output from the prev block
    ///
    /// validators for the current block
    #[prost(bytes="vec", tag="8")]
    pub validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// validators for the next block
    #[prost(bytes="vec", tag="9")]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// consensus params for current block
    #[prost(bytes="vec", tag="10")]
    pub consensus_hash: ::prost::alloc::vec::Vec<u8>,
    /// state after txs from the previous block
    #[prost(bytes="vec", tag="11")]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
    /// consensus info
    ///
    /// evidence included in the block
    #[prost(bytes="vec", tag="13")]
    pub evidence_hash: ::prost::alloc::vec::Vec<u8>,
    /// original proposer of the block
    #[prost(bytes="vec", tag="14")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="16")]
    pub num_txs: u64,
}
/// Vote represents a prevote, precommit, or commit vote from validators for
/// consensus.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vote {
    #[prost(enumeration="SignedMsgType", tag="1")]
    pub r#type: i32,
    #[prost(uint64, tag="2")]
    pub height: u64,
    #[prost(uint32, tag="3")]
    pub round: u32,
    /// zero if vote is nil.
    #[prost(message, optional, tag="4")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(message, optional, tag="5")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes="vec", tag="6")]
    pub validator_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="7")]
    pub validator_index: u32,
    #[prost(bytes="vec", tag="8")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// Commit contains the evidence that a block was committed by a set of validators.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commit {
    #[prost(uint64, tag="1")]
    pub height: u64,
    #[prost(uint32, tag="2")]
    pub round: u32,
    #[prost(message, optional, tag="3")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(message, repeated, tag="4")]
    pub signatures: ::prost::alloc::vec::Vec<CommitSig>,
}
/// CommitSig is a part of the Vote included in a Commit.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitSig {
    #[prost(enumeration="BlockIdFlag", tag="1")]
    pub block_id_flag: i32,
    #[prost(bytes="vec", tag="2")]
    pub validator_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes="vec", tag="4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Proposal {
    #[prost(enumeration="SignedMsgType", tag="1")]
    pub r#type: i32,
    #[prost(uint64, tag="2")]
    pub height: u64,
    #[prost(uint32, tag="3")]
    pub round: u32,
    #[prost(uint32, tag="4")]
    pub pol_round: u32,
    #[prost(message, optional, tag="5")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(message, optional, tag="6")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes="vec", tag="7")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedHeader {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag="2")]
    pub commit: ::core::option::Option<Commit>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockMeta {
    #[prost(message, optional, tag="1")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(message, optional, tag="3")]
    pub header: ::core::option::Option<Header>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BitArray {
    #[prost(int64, tag="1")]
    pub bits: i64,
    #[prost(uint64, repeated, tag="2")]
    pub elems: ::prost::alloc::vec::Vec<u64>,
}
/// BlockIdFlag indicates which BlcokID the signature is for
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlockIdFlag {
    Unknown = 0,
    Absent = 1,
    Commit = 2,
    Nil = 3,
}
impl BlockIdFlag {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BlockIdFlag::Unknown => "BLOCK_ID_FLAG_UNKNOWN",
            BlockIdFlag::Absent => "BLOCK_ID_FLAG_ABSENT",
            BlockIdFlag::Commit => "BLOCK_ID_FLAG_COMMIT",
            BlockIdFlag::Nil => "BLOCK_ID_FLAG_NIL",
        }
    }
}
/// SignedMsgType is a type of signed message in the consensus.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SignedMsgType {
    Unknown = 0,
    /// Votes
    Prevote = 1,
    Precommit = 2,
    /// Proposals
    Proposal = 32,
}
impl SignedMsgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SignedMsgType::Unknown => "SIGNED_MSG_TYPE_UNKNOWN",
            SignedMsgType::Prevote => "SIGNED_MSG_TYPE_PREVOTE",
            SignedMsgType::Precommit => "SIGNED_MSG_TYPE_PRECOMMIT",
            SignedMsgType::Proposal => "SIGNED_MSG_TYPE_PROPOSAL",
        }
    }
}
/// DuplicateVoteEvidence contains evidence a validator signed two conflicting
/// votes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DuplicateVoteEvidence {
    #[prost(message, optional, tag="1")]
    pub vote_a: ::core::option::Option<Vote>,
    #[prost(message, optional, tag="2")]
    pub vote_b: ::core::option::Option<Vote>,
    #[prost(int64, tag="3")]
    pub total_voting_power: i64,
    #[prost(int64, tag="4")]
    pub validator_power: i64,
    #[prost(message, optional, tag="5")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Evidence {
    #[prost(oneof="evidence::Sum", tags="1")]
    pub sum: ::core::option::Option<evidence::Sum>,
}
/// Nested message and enum types in `Evidence`.
pub mod evidence {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Sum {
        #[prost(message, tag="1")]
        DuplicateVoteEvidence(super::DuplicateVoteEvidence),
    }
}
/// EvidenceData contains any evidence of malicious wrong-doing by validators
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvidenceData {
    #[prost(message, repeated, tag="1")]
    pub evidence: ::prost::alloc::vec::Vec<Evidence>,
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
/// Data contains the set of transactions included in the block
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    /// Txs that will be applied by state @ block.Height+1.
    /// NOTE: not all txs here are valid.  We're just agreeing on the order first.
    /// This means that block.AppHash does not include these txs.
    #[prost(bytes="vec", repeated, tag="1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// ConsensusParams contains consensus critical parameters that determine the
/// validity of blocks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusParams {
    #[prost(message, optional, tag="1")]
    pub block: ::core::option::Option<BlockParams>,
    #[prost(message, optional, tag="2")]
    pub evidence: ::core::option::Option<EvidenceParams>,
    #[prost(message, optional, tag="3")]
    pub validator: ::core::option::Option<ValidatorParams>,
}
/// BlockParams contains limits on the block size.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockParams {
    /// Max block size, in bytes.
    /// Note: must be greater than 0
    #[prost(int64, tag="1")]
    pub max_bytes: i64,
    /// Max gas per block.
    /// Note: must be greater or equal to -1
    #[prost(uint64, tag="2")]
    pub max_gas: u64,
    /// Minimum time increment between consecutive blocks (in milliseconds) If the
    /// block header timestamp is ahead of the system clock, decrease this value.
    ///
    /// Not exposed to the application.
    #[prost(int64, tag="3")]
    pub time_iota_ms: i64,
}
/// EvidenceParams determine how we handle evidence of malfeasance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvidenceParams {
    /// Max age of evidence, in blocks.
    ///
    /// The basic formula for calculating this is: MaxAgeDuration / {average block
    /// time}.
    #[prost(int64, tag="1")]
    pub max_age_num_blocks: i64,
    /// Max age of evidence, in time.
    ///
    /// It should correspond with an app's "unbonding period" or other similar
    /// mechanism for handling [Nothing-At-Stake
    /// attacks](<https://github.com/ethereum/wiki/wiki/Proof-of-Stake-FAQ#what-is-the-nothing-at-stake-problem-and-how-can-it-be-fixed>).
    #[prost(message, optional, tag="2")]
    pub max_age_duration: ::core::option::Option<::prost_types::Duration>,
    /// This sets the maximum size of total evidence in bytes that can be committed in a single block.
    /// and should fall comfortably under the max block bytes.
    /// Default is 1048576 or 1MB
    #[prost(int64, tag="3")]
    pub max_bytes: i64,
}
/// ValidatorParams restrict the public key types validators can use.
/// NOTE: uses ABCI pubkey naming, not Amino names.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorParams {
    #[prost(string, repeated, tag="1")]
    pub pub_key_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDataRoundState {
    #[prost(uint64, tag="1")]
    pub height: u64,
    #[prost(uint32, tag="2")]
    pub round: u32,
    #[prost(string, tag="3")]
    pub step: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanonicalBlockId {
    #[prost(bytes="vec", tag="1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub part_set_header: ::core::option::Option<CanonicalPartSetHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanonicalPartSetHeader {
    #[prost(uint32, tag="1")]
    pub total: u32,
    #[prost(bytes="vec", tag="2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanonicalProposal {
    /// type alias for byte
    #[prost(enumeration="SignedMsgType", tag="1")]
    pub r#type: i32,
    /// canonicalization requires fixed size encoding here
    #[prost(uint64, tag="2")]
    pub height: u64,
    /// canonicalization requires fixed size encoding here
    #[prost(uint32, tag="3")]
    pub round: u32,
    #[prost(uint32, tag="4")]
    pub pol_round: u32,
    #[prost(message, optional, tag="5")]
    pub block_id: ::core::option::Option<CanonicalBlockId>,
    #[prost(message, optional, tag="6")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="7")]
    pub chain_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanonicalVote {
    /// type alias for byte
    #[prost(enumeration="SignedMsgType", tag="1")]
    pub r#type: i32,
    /// canonicalization requires fixed size encoding here
    #[prost(uint64, tag="2")]
    pub height: u64,
    /// canonicalization requires fixed size encoding here
    #[prost(uint32, tag="3")]
    pub round: u32,
    #[prost(message, optional, tag="4")]
    pub block_id: ::core::option::Option<CanonicalBlockId>,
    #[prost(message, optional, tag="5")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="6")]
    pub chain_id: ::prost::alloc::string::String,
}
