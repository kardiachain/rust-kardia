//! Macros to calculate constant hash bytes result.
//!
//! Macros from this crate does apply a specific hash function on input.
//! Input can be literal byte array as `b"content"` or array of bytes
//! as `[1, 2, 3]`.
//! Rust identifier can also be use, in this case we use their utf8 string
//! byte representation, for instance if the ident is `MyStruct`, then
//! `b"MyStruct"` will be hashed.
//! If multiple arguments comma separated are passed, they are concatenated
//! then hashed.
//!
//! Examples:
//!
//! ```rust
//! assert_eq!(
//! 	kp_core_hashing_proc_macro::blake2b_256!(b"test"),
//! 	kp_core_hashing::blake2_256(b"test"),
//! );
//! assert_eq!(
//! 	kp_core_hashing_proc_macro::blake2b_256!([1u8]),
//! 	kp_core_hashing::blake2_256(&[1u8]),
//! );
//! assert_eq!(
//! 	kp_core_hashing_proc_macro::blake2b_256!([1, 2, 3]),
//! 	kp_core_hashing::blake2_256(&[1, 2, 3]),
//! );
//! assert_eq!(
//! 	kp_core_hashing_proc_macro::blake2b_256!(identifier),
//! 	kp_core_hashing::blake2_256(b"identifier"),
//! );
//! assert_eq!(
//! 	kp_core_hashing_proc_macro::blake2b_256!(identifier, b"/string"),
//! 	kp_core_hashing::blake2_256(b"identifier/string"),
//! );
//! ```

mod impls;

use impls::MultipleInputBytes;
use proc_macro::TokenStream;

/// Process a Blake2 64-bit hash of bytes parameter outputs a `[u8; 8]`.
/// Multiple inputs are concatenated before hashing.
/// Input can be identifier (name of identifier as bytes is used), byte string or
/// array of bytes.
#[proc_macro]
pub fn blake2b_64(input: TokenStream) -> TokenStream {
	impls::blake2b_64(syn::parse_macro_input!(input as MultipleInputBytes).concatenated())
}

/// Apply a Blake2 256-bit hash of bytes parameter, outputs a `[u8; 32]`.
/// Multiple inputs are concatenated before hashing.
/// Input can be identifier (name of identifier as bytes is used), byte string or
/// array of bytes.
#[proc_macro]
pub fn blake2b_256(input: TokenStream) -> TokenStream {
	impls::blake2b_256(syn::parse_macro_input!(input as MultipleInputBytes).concatenated())
}

/// Apply a Blake2 512-bit hash of bytes parameter, outputs a `[u8; 64]`.
/// Multiple inputs are concatenated before hashing.
/// Input can be identifier (name of identifier as bytes is used), byte string or
/// array of bytes.
#[proc_macro]
pub fn blake2b_512(input: TokenStream) -> TokenStream {
	impls::blake2b_512(syn::parse_macro_input!(input as MultipleInputBytes).concatenated())
}

/// Apply a XX 64-bit hash on its bytes parameter, outputs a `[u8; 8]`.
/// Multiple inputs are concatenated before hashing.
/// Input can be identifier (name of identifier as bytes is used), byte string or
/// array of bytes.
#[proc_macro]
pub fn twox_64(input: TokenStream) -> TokenStream {
	impls::twox_64(syn::parse_macro_input!(input as MultipleInputBytes).concatenated())
}

/// Apply a XX 128-bit hash on its bytes parameter, outputs a `[u8; 16]`.
/// Multiple inputs are concatenated before hashing.
/// Input can be identifier (name of identifier as bytes is used), byte string or
/// array of bytes.
#[proc_macro]
pub fn twox_128(input: TokenStream) -> TokenStream {
	impls::twox_128(syn::parse_macro_input!(input as MultipleInputBytes).concatenated())
}

/// Apply a keccak 256-bit hash on its bytes parameter, outputs a `[u8; 32]`.
/// Multiple inputs are concatenated before hashing.
/// Input can be identifier (name of identifier as bytes is used), byte string or
/// array of bytes.
#[proc_macro]
pub fn keccak_256(input: TokenStream) -> TokenStream {
	impls::keccak_256(syn::parse_macro_input!(input as MultipleInputBytes).concatenated())
}

/// Apply a keccak 512-bit hash on its bytes parameter, outputs a `[u8; 64]`.
/// Multiple inputs are concatenated before hashing.
/// Input can be identifier (name of identifier as bytes is used), byte string or
/// array of bytes.
#[proc_macro]
pub fn keccak_512(input: TokenStream) -> TokenStream {
	impls::keccak_512(syn::parse_macro_input!(input as MultipleInputBytes).concatenated())
}

/// Apply a sha2 256-bit hash on its bytes parameter, outputs a `[u8; 32]`.
/// Multiple inputs are concatenated before hashing.
/// Input can be identifier (name of identifier as bytes is used), byte string or
/// array of bytes.
#[proc_macro]
pub fn sha2_256(input: TokenStream) -> TokenStream {
	impls::sha2_256(syn::parse_macro_input!(input as MultipleInputBytes).concatenated())
}