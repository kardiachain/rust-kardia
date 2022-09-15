use std::{cmp::Ordering, fmt::Debug};

use crate::common::address::Address;

pub trait ValidatorSetTrait: Debug + Sync + Send + 'static {
    fn get_proposer(&self) -> Option<Validator>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidatorSet {
    pub validators: Vec<Validator>,
    pub proposer: Option<Validator>,
    pub total_voting_power: i64,
}

impl ValidatorSet {
    pub fn get_proposer(&mut self) -> Option<Validator> {
        if self.clone().validators.len() == 0 {
            return None;
        }
        if let Some(proposer) = self.proposer.clone() {
            Some(proposer)
        } else {
            self.proposer = self.find_proposer();
            self.proposer.clone()
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
                return Some((i, v.clone()))
            }
        }

        return None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Validator {
    pub address: Address,
    pub voting_power: i64,
    pub proposer_priority: i64,
}

impl Validator {
    pub fn compare_proposer_priority(&self, other: Validator) -> Self {
        if self.proposer_priority > other.proposer_priority {
            return self.clone();
        }

        if self.proposer_priority < other.proposer_priority {
            return other;
        }

        match self.address.to_vec().cmp(&other.address.to_vec()) {
            Ordering::Less => self.clone(),
            Ordering::Greater => other,
            _ => panic!("cannot compare identical validators"),
        }
    }
}
