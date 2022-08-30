pub mod bit_array;
pub mod block;
pub mod block_operations;
pub mod commit;
pub mod consensus;
pub mod crypto;
pub mod evidence;
pub mod misc;
pub mod part_set;
pub mod peer;
pub mod proposal;
pub mod round;
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
