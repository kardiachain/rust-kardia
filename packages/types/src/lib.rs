
pub mod block;
pub mod proposal;
pub mod bit_array;
pub mod vote;
pub mod crypto;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
