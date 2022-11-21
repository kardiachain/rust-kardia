#![feature(is_some_with)]
#![feature(async_closure)]

mod reactor;
mod state;
mod types;
mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
