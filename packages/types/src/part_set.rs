use crate::{bit_array::BitArray, crypto::Proof};

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

    pub fn get_part(&self, index: usize) -> Part {
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
        self.parts[part.clone().index as usize] = part.clone();
        self.count += 1;
        if let Some(pba) = self.parts_bit_array.as_mut() {
            pba.set_index(part.index as usize, true);
        } else {
            return Err("parts bit array isn't initialized".to_owned());
        }
        Ok(())
    }

    pub fn new_part_set_from_header(header: PartSetHeader) -> Self {
        Self {
            total: header.total,
            hash: header.hash,
            parts: Vec::with_capacity(header.total as usize),
            parts_bit_array: Some(BitArray::new_bit_array(header.total as usize)),
            count: 0,
        }
    }

    pub fn is_completed(&self) -> bool {
        self.count == self.total
    }
}