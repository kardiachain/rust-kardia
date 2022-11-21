use std::fmt::Debug;

use mockall::automock;

#[automock]
pub trait LatestBlockState: Debug + Sync + Send + 'static {
    fn get_chain_id(&self) -> ChainId;
    fn get_initial_height(&self) -> u64;
    fn get_last_block_height(&self) -> u64;
}

pub type ChainId = String;
