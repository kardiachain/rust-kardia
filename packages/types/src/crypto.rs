pub struct Proof {
    pub total: u64,
    pub index: u64,
    pub leaf_hash: Vec<u8>,
    pub aunts: Vec<Vec<u8>>,
}