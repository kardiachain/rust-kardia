pub struct PartSetHeader {
    pub total: u32,
    pub hash: Vec<u8>,
}

impl From<kai_proto::types::PartSetHeader> for PartSetHeader {
    fn from(m: kai_proto::types::PartSetHeader) -> Self {
        Self {
            total: m.total,
            hash: m.hash,
        }
    }
}

pub struct BlockId {
    pub hash: Vec<u8>,
    pub part_set_header: Option<PartSetHeader>,
}

impl From<kai_proto::types::BlockId> for BlockId {
    fn from(m: kai_proto::types::BlockId) -> Self {
        Self {
            hash: m.hash,
            part_set_header: m.part_set_header.map(|psh| psh.into()),
        }
    }
}
