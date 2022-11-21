use std::{cmp::Ordering, fmt::Debug};

use ethereum_types::Address;

pub trait ValidatorSetTrait: Debug + Sync + Send + 'static {
    fn get_proposer(&self) -> Option<Validator>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidatorSet {
    pub validators: Vec<Validator>,
    pub proposer: Option<Validator>,
    pub total_voting_power: u64,
}

impl ValidatorSet {
    pub fn get_proposer(&mut self) -> Option<Validator> {
        if self.clone().validators.len() == 0 {
            return None;
        }
        if let Some(proposer) = self.clone().proposer {
            Some(proposer)
        } else {
            let proposer = self.clone().find_proposer();
            self.proposer = proposer.clone();
            proposer.clone()
        }
    }

    fn find_proposer(&self) -> Option<Validator> {
        let mut proposer: Option<Validator> = None;

        for val in self.clone().validators {
            if proposer.is_none() {
                proposer = Some(val);
            } else if let Some(p) = proposer {
                proposer = Some(p.compare_proposer_priority(val));
            }
        }

        return proposer;
    }

    pub fn get_by_address(&self, address: Address) -> Option<(usize, Validator)> {
        for (i, v) in self.validators.iter().enumerate() {
            if address.eq(&v.address) {
                return Some((i, v.clone()));
            }
        }

        return None;
    }

    pub fn size(&self) -> usize {
        return self.validators.len();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Validator {
    pub address: Address,
    pub voting_power: u64,
    pub proposer_priority: u64,
}

impl Validator {
    pub fn compare_proposer_priority(&self, other: Validator) -> Self {
        if self.proposer_priority > other.proposer_priority {
            return self.clone();
        }

        if self.proposer_priority < other.proposer_priority {
            return other;
        }

        match self.address.cmp(&other.address) {
            Ordering::Less => self.clone(),
            Ordering::Greater => other,
            _ => panic!("cannot compare identical validators"),
        }
    }
}
