use std::fmt::Debug;

use mockall::automock;

#[automock]
pub trait EvidencePool: Debug + Sync + Send + 'static {}
