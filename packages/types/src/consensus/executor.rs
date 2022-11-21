use std::{fmt::Debug, sync::Arc};

use mockall::automock;

use crate::block::Block;

use super::state::LatestBlockState;

#[automock]
pub trait BlockExecutor: Debug + Sync + Send + 'static {
    fn validate_block(
        &self,
        state: Arc<Box<dyn LatestBlockState>>,
        block: Block,
    ) -> Result<(), String>;
}
