use std::fmt::Debug;

use crate::{bit_array::BitArray, block::BlockId};

pub trait VoteSetReader: Debug + Sync + Send + 'static {}

#[derive(Debug, Clone, PartialEq)]
pub struct VoteSet {}

impl VoteSet {
    pub fn bit_array(&self) -> Option<BitArray> {
        todo!()
    }

    pub fn bit_array_by_block_id(&self, block_id: BlockId) -> Option<BitArray> {
        todo!()
    }
}

impl VoteSetReader for VoteSet {}
