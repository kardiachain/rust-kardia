use bitvec::{prelude::Msb0, vec::BitVec};

/// BitArray is a wrapper struct that extends BitVec.
#[derive(Debug, Clone, PartialEq)]
pub struct BitArray {
    bv: BitVec<u64, Msb0>,
}

impl From<kai_proto::types::BitArray> for BitArray {
    fn from(m: kai_proto::types::BitArray) -> Self {
        let mut bv: BitVec<u64, Msb0> = BitVec::with_capacity(m.bits as usize);
        bv.copy_from_bitslice(BitVec::from_vec(m.elems).as_bitslice());
        Self { bv: bv }
    }
}

impl Into<kai_proto::types::BitArray> for BitArray {
    fn into(self: Self) -> kai_proto::types::BitArray {
        kai_proto::types::BitArray {
            bits: self.bv.capacity() as i64,
            elems: self.bv.into_vec(),
        }
    }
}

impl BitArray {
    pub fn new(bits: usize) -> Self {
        Self {
            bv: BitVec::<u64, Msb0>::repeat(false, bits),
        }
    }

    pub fn set_index(&mut self, i: usize, v: bool) -> bool {
        if i >= self.bv.capacity() {
            return false;
        }

        self.bv.set(i, v);
        return true;
    }

    pub fn get_index(&self, i: usize) -> Result<bool, String> {
        if let Some(v) = self.bv.get(i).as_deref() {
            Ok(*v)
        } else {
            Err("index out of bound".to_owned())
        }
    }

    pub fn not(self) -> Self {
        todo!()
    }

    pub fn sub(self, o: Self) -> Self {
        todo!()
    }

    pub fn pick_random(self) -> Option<usize> {
        todo!()
    }
}
