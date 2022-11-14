//! Macros to derive runtime debug implementation.
//!
//! This custom derive implements a `core::fmt::Debug` trait,
//! but in case the `std` feature is enabled the implementation
//! will actually print out the structure as regular `derive(Debug)`
//! would do. If `std` is disabled the implementation will be empty.
//!
//! This behaviour is useful to prevent bloating the runtime WASM
//! blob from unneeded code.
//!
//! ```rust
//! #[derive(kp_debug_derive::RuntimeDebug)]
//! struct MyStruct;
//!
//! assert_eq!(format!("{:?}", MyStruct), "MyStruct");
//! ```

mod impls;

use proc_macro::TokenStream;

#[proc_macro_derive(RuntimeDebug)]
pub fn debug_derive(input: TokenStream) -> TokenStream {
	impls::debug_derive(syn::parse_macro_input!(input))
}