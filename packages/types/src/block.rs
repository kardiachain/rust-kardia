use crate::part_set::PartSetHeader;

#[derive(Debug, Clone)]
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

impl Into<kai_proto::types::BlockId> for BlockId {
    fn into(self) -> kai_proto::types::BlockId {
        kai_proto::types::BlockId {
            hash: self.hash,
            part_set_header: self.part_set_header.map(|psh| psh.into()),
        }
    }
}
