#[derive(Debug, Clone)]
pub struct Proof {
    pub total: u64,
    pub index: u64,
    pub leaf_hash: Vec<u8>,
    pub aunts: Vec<Vec<u8>>,
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
