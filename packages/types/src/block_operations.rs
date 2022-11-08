use std::{fmt::Debug, sync::Arc};

use ethereum_types::Address;
use mockall::automock;

use crate::{
    block::{Block, BlockMeta},
    commit::Commit,
    consensus::state::LatestBlockState,
    part_set::{Part, PartSet},
};

#[automock]
pub trait BlockOperations: Debug + Sync + Send + 'static {
    fn base(&self) -> u64;
    fn height(&self) -> u64;
    fn load_block_meta(&self, height: u64) -> Option<BlockMeta>;
    fn load_block_part(&self, height: u64, index: usize) -> Option<Part>;
    fn load_block_commit(&self, height: u64) -> Option<Commit>;
    fn load_seen_commit(&self, height: u64) -> Option<Commit>;
    fn create_proposal_block(
        &self,
        height: u64,
        state: Arc<Box<dyn LatestBlockState>>,
        proposer_address: Address,
        commit: Option<Commit>,
    ) -> (Block, PartSet);
    fn load_commit(&self, height: u64) -> Option<Commit>;
    fn save_block(&self, block: Block, block_parts: PartSet, seen_commit: Commit);
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
            return self.load_seen_commit(height);
        } else {
            return self.load_block_commit(height);
        }
    }

    fn save_block(&self, block: Block, block_parts: PartSet, seen_commit: Commit) {
        todo!()
    }

    fn create_proposal_block(
        &self,
        height: u64,
        state: Arc<Box<dyn LatestBlockState>>,
        proposer_address: Address,
        commit: Option<Commit>,
    ) -> (Block,PartSet) {
        todo!()
    }
}
