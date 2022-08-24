use bitvec::prelude::*;
// pub type BitArray = BitArr!(for 64, in u64, Msb0);

#[derive(Debug, Clone)]
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

// TODO: implement fns of bit_array.go

impl BitArray {
    pub fn set_index(mut self, i: usize , v :bool) -> bool {
        if i >= self.bits.try_into().unwrap() {
            return false
        }

        if v {
            self.elems[i/64] |= 1u64 << (i%64);
        } else {
		    self.elems[i/64] &= !(1u64 << (i%64));
        }

        return true
    }

    pub fn sub(self, bp: Self) -> Option<Self> {
        todo!()
    }

    pub fn pick_random(self) -> Option<usize> {
        todo!()
    }
}