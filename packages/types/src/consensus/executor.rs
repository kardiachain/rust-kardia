use std::fmt::Debug;

use mockall::automock;

#[automock]
pub trait BlockExecutor: Debug + Sync + Send + 'static {}
