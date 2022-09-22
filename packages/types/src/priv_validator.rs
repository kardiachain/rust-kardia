use std::fmt::Debug;

use crate::vote::Vote;
use ethereum_types::Address;
use mockall::automock;

use crate::{consensus::state::ChainId, proposal::Proposal};

#[automock]
pub trait PrivValidator: Debug + Sync + Send + 'static {
    // TODO: reference in priv_validator.go, 97
    fn sign_proposal(&self, chain_id: ChainId, proposal: &mut Proposal) -> Result<(), String>;
    fn sign_vote(&self, chain_id: ChainId, vote: &mut Vote) -> Result<(), String>;
    fn get_address(&self) -> Option<Address>;
}
