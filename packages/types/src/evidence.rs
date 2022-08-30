use std::fmt::Debug;

pub trait EvidencePool: Debug + Sync + Send + 'static {}
