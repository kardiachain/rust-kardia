use std::fmt::Debug;

use mockall::automock;

#[automock]
pub trait LatestBlockState: Debug + Sync + Send + 'static {
    fn get_last_block_height(&self) -> u64;
}
