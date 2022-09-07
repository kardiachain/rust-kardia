use std::fmt::Debug;

use crate::{block::BlockMeta, commit::Commit, part_set::Part};

pub trait BlockOperations: Debug + Sync + Send + 'static {
    fn base(&self) -> u64;
    fn height(&self) -> u64;
    fn load_block_meta(&self, height: u64) -> Option<BlockMeta>;
    fn load_block_part(&self, height: u64, index: usize) -> Option<Part>;
    fn load_block_commit(&self, height: u64) -> Option<Commit>;
    fn load_seen_commit(&self, height: u64) -> Option<Commit>;
    fn load_commit(&self, height: u64) -> Option<Commit>;
}

#[derive(Debug, Clone)]
pub struct BlockOperationsImpl {}

impl BlockOperations for BlockOperationsImpl {
    fn base(&self) -> u64 {
        todo!()
    }

    fn height(&self) -> u64 {
        todo!()
    }

    fn load_block_meta(&self, height: u64) -> Option<BlockMeta> {
        todo!()
    }

    fn load_block_part(&self, height: u64, index: usize) -> Option<Part> {
        todo!()
    }

    fn load_block_commit(&self, height: u64) -> Option<Commit> {
        todo!()
    }

    fn load_seen_commit(&self, height: u64) -> Option<Commit> {
        todo!()
    }

    fn load_commit(&self, height: u64) -> Option<Commit> {
        if height == self.height() {
            return self.load_seen_commit(height)
        } else {
            return self.load_block_commit(height)
        }
    }
}
