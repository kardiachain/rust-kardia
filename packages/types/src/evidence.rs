use std::fmt::Debug;

use mockall::automock;

use crate::vote::Vote;

#[automock]
pub trait EvidencePool: Debug + Sync + Send + 'static {}


#[derive(Clone, PartialEq, prost::Message)]
pub struct DuplicateVoteEvidence {
    #[prost(message, optional, tag="1")]
    pub vote_a: Option<Vote>,
    #[prost(message, optional, tag="2")]
    pub vote_b: Option<Vote>,
    #[prost(int64, tag="3")]
    pub total_voting_power: i64,
    #[prost(int64, tag="4")]
    pub validator_power: i64,
    #[prost(message, optional, tag="5")]
    pub timestamp: Option<prost_types::Timestamp>,
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

#[derive(Clone, PartialEq, prost::Message)]
pub struct EvidenceData {
    #[prost(message, repeated, tag="1")]
    pub evidence: Vec<Evidence>,
}