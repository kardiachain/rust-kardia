use std::fmt::Debug;

use mockall::automock;

#[automock]
pub trait LatestBlockState: Debug + Sync + Send + 'static {}
