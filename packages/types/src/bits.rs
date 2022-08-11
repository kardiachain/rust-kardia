pub struct BitArray {
    pub bits: i64,
    pub elems: Vec<u64>,
}

impl From<kai_proto::types::BitArray> for BitArray {
    fn from(m: kai_proto::types::BitArray) -> Self {
        Self {
            bits: m.bits,
            elems: m.elems,
        }
    }
}
