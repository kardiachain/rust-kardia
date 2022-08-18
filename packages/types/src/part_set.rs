use crate::crypto::Proof;

pub struct Part {
    pub index: u32,
    pub bytes: Vec<u8>,
    pub proof: Option<Proof>,
}

pub struct PartSetHeader {
    pub total: u32,
    pub hash: Vec<u8>,
}