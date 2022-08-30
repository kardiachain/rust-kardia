#[derive(Debug, Clone, PartialEq)]
pub struct ValidatorSet {
    pub validators: Vec<Validator>,
    pub proposer: Option<Validator>,
    pub total_voting_power: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Validator {
    pub address: Vec<u8>,
    pub voting_power: i64,
    pub proposer_priority: i64,
}
