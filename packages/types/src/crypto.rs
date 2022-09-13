#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Proof {
    #[prost(uint64, tag = "1")]
    pub total: u64,
    #[prost(uint64, tag = "2")]
    pub index: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub leaf_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub aunts: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}

impl From<kai_proto::crypto::Proof> for Proof {
    fn from(m: kai_proto::crypto::Proof) -> Self {
        Self {
            total: m.total,
            index: m.index,
            leaf_hash: m.leaf_hash,
            aunts: m.aunts,
        }
    }
}

impl Into<kai_proto::crypto::Proof> for Proof {
    fn into(self) -> kai_proto::crypto::Proof {
        kai_proto::crypto::Proof {
            total: self.total,
            index: self.index,
            leaf_hash: self.leaf_hash,
            aunts: self.aunts,
        }
    }
}
