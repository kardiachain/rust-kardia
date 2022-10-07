/// ValidatorsInfo represents the latest validator set, or the last height it changed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorsInfo {
    #[prost(message, optional, tag="1")]
    pub validator_set: ::core::option::Option<super::types::ValidatorSet>,
    #[prost(uint64, tag="2")]
    pub last_height_changed: u64,
}
/// ConsensusParamsInfo represents the latest consensus params, or the last height it changed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusParamsInfo {
    #[prost(message, optional, tag="1")]
    pub consensus_params: ::core::option::Option<super::types::ConsensusParams>,
    #[prost(uint64, tag="2")]
    pub last_height_changed: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct State {
    #[prost(string, tag="2")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="14")]
    pub initial_height: u64,
    #[prost(uint64, tag="3")]
    pub last_block_height: u64,
    #[prost(message, optional, tag="4")]
    pub last_block_id: ::core::option::Option<super::types::BlockId>,
    #[prost(message, optional, tag="5")]
    pub last_block_time: ::core::option::Option<::prost_types::Timestamp>,
    /// LastValidators is used to validate block.LastCommit.
    /// Validators are persisted to the database separately every time they change,
    /// so we can query for historical validator sets.
    /// Note that if s.LastBlockHeight causes a valset change,
    /// we set s.LastHeightValidatorsChanged = s.LastBlockHeight + 1 + 1
    /// Extra +1 due to nextValSet delay.
    #[prost(message, optional, tag="6")]
    pub next_validators: ::core::option::Option<super::types::ValidatorSet>,
    #[prost(message, optional, tag="7")]
    pub validators: ::core::option::Option<super::types::ValidatorSet>,
    #[prost(message, optional, tag="8")]
    pub last_validators: ::core::option::Option<super::types::ValidatorSet>,
    #[prost(uint64, tag="9")]
    pub last_height_validators_changed: u64,
    #[prost(uint64, tag="11")]
    pub last_height_consensus_params_changed: u64,
    #[prost(message, optional, tag="10")]
    pub consensus_params: ::core::option::Option<super::types::ConsensusParams>,
    /// the latest AppHash we've received from calling abci.Commit()
    #[prost(bytes="vec", tag="13")]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Txs {
    #[prost(bytes="vec", repeated, tag="1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PooledTransactions {
    #[prost(bytes="vec", repeated, tag="1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PooledTransactionHashes {
    #[prost(bytes="vec", repeated, tag="1")]
    pub hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPooledTransactions {
    #[prost(bytes="vec", repeated, tag="1")]
    pub hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(oneof="message::Sum", tags="1, 2, 3, 4")]
    pub sum: ::core::option::Option<message::Sum>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Sum {
        #[prost(message, tag="1")]
        Txs(super::Txs),
        #[prost(message, tag="2")]
        PooledTransactionHashes(super::PooledTransactionHashes),
        #[prost(message, tag="3")]
        PooledTransactions(super::PooledTransactions),
        #[prost(message, tag="4")]
        RequestPooledTransactions(super::RequestPooledTransactions),
    }
}
