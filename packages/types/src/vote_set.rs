use std::fmt::Debug;

pub trait VoteSetReader: Debug + Sync + Send + 'static {}

#[derive(Debug, Clone, PartialEq)]
pub struct VoteSet {}

impl VoteSetReader for VoteSet {}
