use std::fmt::Debug;

use crate::block::BlockMeta;

pub trait BlockOperations: Debug + Sync + Send + 'static {
    fn base(&self) -> u64;
    fn height(&self) -> u64;
    fn load_block_meta(&self, height: u64) -> Option<BlockMeta>;
}
