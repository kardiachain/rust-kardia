use crate::bit_array::BitArray;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Part {
    #[prost(uint32, tag = "1")]
    pub index: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub proof: ::core::option::Option<super::crypto::Proof>,
}

impl From<kai_proto::types::Part> for Part {
    fn from(m: kai_proto::types::Part) -> Self {
        Self {
            index: m.index,
            bytes: m.bytes,
            proof: m.proof.map(|p| p.into()),
        }
    }
}

impl Into<kai_proto::types::Part> for Part {
    fn into(self) -> kai_proto::types::Part {
        kai_proto::types::Part {
            index: self.index,
            bytes: self.bytes,
            proof: self.proof.map(|p| p.into()),
        }
    }
}

#[derive(Clone, ::prost::Message)]
pub struct PartSetHeader {
    #[prost(uint32, tag = "1")]
    pub total: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
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

impl PartSetHeader {
    pub fn is_zero(&self) -> bool {
        return self.total == 0 && self.hash.len() == 0;
    }
}

pub const BLOCK_PART_SIZE_BYTES: u32 = 65536;

#[derive(Debug, Clone)]
pub struct PartSet {
    pub total: u32,
    pub hash: Vec<u8>,

    pub parts: Vec<Option<Part>>,
    pub parts_bit_array: BitArray,
    pub count: u32,
}

impl PartSet {
    pub fn new(data: Vec<u8>, part_size: u32) -> Self {
        todo!()
    }

    pub fn header(&self) -> PartSetHeader {
        PartSetHeader {
            total: self.total,
            hash: self.hash.clone(),
        }
    }

    pub fn has_header(&self, header: PartSetHeader) -> bool {
        return self.header().eq(&header);
    }

    pub fn get_part(&self, index: usize) -> Option<Part> {
        self.parts[index].clone()
    }

    pub fn add_part(&mut self, part: Part) -> Result<(), String> {
        if part.clone().index >= self.total {
            return Err("unexpected index".to_owned());
        }

        // check hash proof
        if !part
            .clone()
            .proof
            .expect("part invalid: proof does not existed")
            .verify(self.hash.clone())
        {
            return Err("invalid proof".to_owned());
        }

        // add part
        self.parts
            .insert(part.clone().index as usize, Some(part.clone()));
        self.parts_bit_array
            .set_index(part.clone().index as usize, true);
        self.count += 1;
        Ok(())
    }

    pub fn new_part_set_from_header(header: PartSetHeader) -> Self {
        Self {
            total: header.total,
            hash: header.hash,
            parts: vec![None; header.total as usize],
            parts_bit_array: BitArray::new(header.total as usize),
            count: 0,
        }
    }

    pub fn is_completed(&self) -> bool {
        self.count == self.total
    }
}
