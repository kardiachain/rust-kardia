use std::fmt::Debug;

use mockall::automock;

use crate::{
    common::{address::Address},
    consensus::state::ChainId,
    proposal::Proposal,
};

#[automock]
pub trait PrivValidator: Debug + Sync + Send + 'static {
    // TODO: reference in priv_validator.go, 97
    fn sign_proposal(&self, chain_id: ChainId, proposal: &mut Proposal) -> Result<(), String>;
    fn get_address(&self) -> Option<Address>;
}
