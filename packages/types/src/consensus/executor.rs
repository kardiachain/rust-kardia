use std::fmt::Debug;

pub trait BlockExecutor: Debug + Sync + Send + 'static {}
