#![feature(is_some_with)]
#![feature(extend_one)]

pub mod bit_array;
pub mod block;
pub mod block_operations;
pub mod canonical_types;
pub mod commit;
pub mod common;
pub mod consensus;
pub mod crypto;
pub mod errors;
pub mod evidence;
pub mod misc;
pub mod part_set;
pub mod peer;
pub mod priv_validator;
pub mod proposal;
pub mod round;
pub mod timestamp;
pub mod types;
pub mod validator_set;
pub mod vote;
pub mod vote_set;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
