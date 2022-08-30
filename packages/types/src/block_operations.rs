use std::fmt::Debug;

use crate::{block::BlockMeta, part_set::Part, commit::Commit};

pub trait BlockOperations: Debug + Sync + Send + 'static {
    fn base(&self) -> u64;
    fn height(&self) -> u64;
    fn load_block_meta(&self, height: u64) -> Option<BlockMeta>;
    fn load_block_part(&self, height: u64, index: usize) -> Option<Part>;
    fn load_block_commit(&self, height: u64) -> Option<Commit>;
}
