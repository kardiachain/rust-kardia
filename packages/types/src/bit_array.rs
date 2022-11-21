use std::ops::{BitAnd, BitOr, BitXor, Not};

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

    pub fn len(&self) -> usize {
        self.bv.len()
    }

    pub fn not(&self) -> Self {
        Self {
            bv: self.clone().bv.not(),
        }
    }

    pub fn and(&self, rhs: Self) -> Self {
        Self {
            bv: self.clone().bv.bitand(rhs.clone().bv),
        }
    }

    pub fn or(&self, rhs: Self) -> Self {
        Self {
            bv: self.clone().bv.bitor(rhs.clone().bv),
        }
    }

    pub fn sub(&self, o: Self) -> Self {
        if self.len() > o.len() {
            let mut o_bv = o.clone().bv;
            o_bv.resize(self.len(), false);
            // o_bv.extend_from_bitslice(
            //     BitVec::<u64, Msb0>::repeat(false, self.len() - o.len()).as_bitslice(),
            // );
            return self.and(Self { bv: o_bv }.not());
        }

        self.and(o.not())
    }

    pub fn update(&mut self, src: Self) {
        if self.len() != src.len() {
            self.bv.resize(src.len(), false);
        }
        self.bv.copy_from_bitslice(&src.bv);
    }

    pub fn to_string(&self) -> String {
        self.bv.to_string()
    }

    pub fn pick_random(self) -> Option<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::BitArray;

    #[test]
    fn sub() {
        let mut ba_1 = BitArray::new(4); // 1101
        ba_1.set_index(0, true);
        ba_1.set_index(1, true);
        ba_1.set_index(3, true);

        let mut ba_2 = BitArray::new(3); // 101
        ba_2.set_index(0, true);
        ba_2.set_index(2, true);

        assert_eq!(
            ba_1.clone().sub(ba_2.clone()).to_string(),
            String::from("[0, 1, 0, 1]")
        );
        assert_eq!(
            ba_2.clone().sub(ba_1.clone()).to_string(),
            String::from("[0, 0, 1]")
        );
    }
}
