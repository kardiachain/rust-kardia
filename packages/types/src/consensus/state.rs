use std::fmt::Debug;

pub trait LatestBlockState: Debug + Sync + Send + 'static {}
