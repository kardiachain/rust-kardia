use crate::{bit_array::BitArray, crypto::Proof};

#[derive(Debug, Clone)]
pub struct Part {
    pub index: u32,
    pub bytes: Vec<u8>,
    pub proof: Option<Proof>,
}

#[derive(Debug, Clone)]
pub struct PartSetHeader {
    pub total: u32,
    pub hash: Vec<u8>,
}

impl PartialEq for PartSetHeader {
    fn eq(&self, other: &Self) -> bool {
        self.total == other.total && self.hash == other.hash
    }
}

impl Eq for PartSetHeader {}

impl From<kai_proto::types::PartSetHeader> for PartSetHeader {
    fn from(m: kai_proto::types::PartSetHeader) -> Self {
        Self {
            total: m.total,
            hash: m.hash,
        }
    }
}

impl Into<kai_proto::types::PartSetHeader> for PartSetHeader {
    fn into(self) -> kai_proto::types::PartSetHeader {
        kai_proto::types::PartSetHeader {
            total: self.total,
            hash: self.hash,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PartSet {
    pub total: u32,
    pub hash: Vec<u8>,

    pub parts: Vec<Part>,
    pub parts_bit_array: Option<BitArray>,
    pub count: u32,
}

impl PartSet {
    pub fn header(&self) -> PartSetHeader {
        PartSetHeader {
            total: self.total,
            hash: self.hash.clone(),
        }
    }

    pub fn has_header(&self, header: PartSetHeader) -> bool {
        return self.header().eq(&header);
    }
}
