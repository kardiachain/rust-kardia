use std::fmt::Debug;

use crate::{consensus::state::ChainId, proposal::Proposal};

pub trait PrivValidator: Debug + Sync + Send + 'static {
    // TODO: reference in priv_validator.go, 97
    fn sign_proposal(&self, chain_id: ChainId, proposal: &mut Proposal) -> Result<(), String>;
}
