use crate::{bit_array::BitArray, block::Block};
use ethereum_types::H256;
use kp_merkle::proof::{proof_from_byte_vectors, Proof};
use prost::Message;
use std::cmp;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Part {
    #[prost(uint32, tag = "1")]
    pub index: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub proof: ::core::option::Option<Proof>,
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
    pub fn new_from_block(block: &Block) -> Self {
        let bz = Block::encode_to_vec(block);
        Self::new(bz, BLOCK_PART_SIZE_BYTES)
    }

    pub fn new(data: Vec<u8>, part_size: u32) -> Self {
        // divide data into 4kb parts.
        let total = (data.len() as u32 + part_size - 1) / part_size;
        let mut parts: Vec<Option<Part>> = Vec::new();
        let mut parts_bytes: Vec<Vec<u8>> = Vec::new();
        let mut parts_bit_array: BitArray = BitArray::new(total as usize);
        for i in 0..(total as usize) {
            let part = Part {
                index: i as u32,
                bytes: data[i * (part_size as usize)
                    ..cmp::min(data.len(), (i + 1) * (part_size as usize))]
                    .to_vec(),
                proof: None, // will be updated below
            };
            parts.push(Some(part.clone()));
            parts_bytes.push(part.bytes.clone());
            parts_bit_array.set_index(i, true);
        }
        // Compute merkle proofs
        let (root, proofs) = proof_from_byte_vectors(parts_bytes);
        for i in 0..(total as usize) {
            if let Some(part) = parts.get_mut(i) {
                let part = part.as_mut();
                part.unwrap().proof = proofs.get(i).cloned();
            }
        }
        return PartSet {
            total: total,
            hash: root.as_bytes().to_vec(),
            parts: parts,
            parts_bit_array,
            count: total,
        };
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

        if part.clone().proof.is_none() {
            return Err("proof is none".to_owned());
        }

        // check hash proof
        if let Err(e) = part.clone().proof.unwrap().verify(
            H256::from_slice(self.hash.clone().as_slice()),
            part.bytes.as_slice(),
        ) {
            return Err(format!("invalid proof: {:?}", e));
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

    pub fn recover_block(&self) -> Result<Block, &str> {
        if self.is_completed() {
            let bz: Vec<u8> = self
                .parts
                .iter()
                .map(|part| part.clone().unwrap().bytes)
                .flatten()
                .collect();

            if let Ok(block) = Block::decode(bz.as_slice()) {
                Ok(block)
            } else {
                Err("block decode error")
            }
        } else {
            Err("cannot recover block on incompleted part set")
        }
    }
}
