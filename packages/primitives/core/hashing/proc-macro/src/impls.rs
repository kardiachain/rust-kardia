use quote::quote;
use syn::parse::{Parse, ParseStream};

use proc_macro::TokenStream;

pub(super) struct InputBytes(pub Vec<u8>);

pub(super) struct MultipleInputBytes(pub Vec<Vec<u8>>);

impl MultipleInputBytes {
	pub(super) fn concatenated(mut self) -> Vec<u8> {
		if self.0.is_empty() {
			Vec::new()
		} else {
			let mut result = core::mem::take(&mut self.0[0]);
			for other in self.0[1..].iter_mut() {
				result.append(other);
			}
			result
		}
	}
}

impl Parse for InputBytes {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		match syn::ExprArray::parse(input) {
			Ok(array) => {
				let mut bytes = Vec::<u8>::new();
				for expr in array.elems.iter() {
					match expr {
						syn::Expr::Lit(lit) => match &lit.lit {
							syn::Lit::Int(b) => bytes.push(b.base10_parse()?),
							syn::Lit::Byte(b) => bytes.push(b.value()),
							_ =>
								return Err(syn::Error::new(
									input.span(),
									"Expected array of u8 elements.".to_string(),
								)),
						},
						_ =>
							return Err(syn::Error::new(
								input.span(),
								"Expected array of u8 elements.".to_string(),
							)),
					}
				}
				return Ok(InputBytes(bytes))
			},
			Err(_e) => (),
		}
		// use rust names as a vec of their utf8 bytecode.
		match syn::Ident::parse(input) {
			Ok(ident) => return Ok(InputBytes(ident.to_string().as_bytes().to_vec())),
			Err(_e) => (),
		}
		Ok(InputBytes(syn::LitByteStr::parse(input)?.value()))
	}
}

impl Parse for MultipleInputBytes {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let elts =
			syn::punctuated::Punctuated::<InputBytes, syn::token::Comma>::parse_terminated(input)?;
		Ok(MultipleInputBytes(elts.into_iter().map(|elt| elt.0).collect()))
	}
}

pub(super) fn twox_64(bytes: Vec<u8>) -> TokenStream {
	bytes_to_array(kp_core_hashing::twox_64(bytes.as_slice()))
}

pub(super) fn twox_128(bytes: Vec<u8>) -> TokenStream {
	bytes_to_array(kp_core_hashing::twox_128(bytes.as_slice()))
}

pub(super) fn blake2b_512(bytes: Vec<u8>) -> TokenStream {
	bytes_to_array(kp_core_hashing::blake2_512(bytes.as_slice()))
}

pub(super) fn blake2b_256(bytes: Vec<u8>) -> TokenStream {
	bytes_to_array(kp_core_hashing::blake2_256(bytes.as_slice()))
}

pub(super) fn blake2b_64(bytes: Vec<u8>) -> TokenStream {
	bytes_to_array(kp_core_hashing::blake2_64(bytes.as_slice()))
}

pub(super) fn keccak_256(bytes: Vec<u8>) -> TokenStream {
	bytes_to_array(kp_core_hashing::keccak_256(bytes.as_slice()))
}

pub(super) fn keccak_512(bytes: Vec<u8>) -> TokenStream {
	bytes_to_array(kp_core_hashing::keccak_512(bytes.as_slice()))
}

pub(super) fn sha2_256(bytes: Vec<u8>) -> TokenStream {
	bytes_to_array(kp_core_hashing::sha2_256(bytes.as_slice()))
}

fn bytes_to_array(bytes: impl IntoIterator<Item = u8>) -> TokenStream {
	let bytes = bytes.into_iter();

	quote!(
		[ #( #bytes ),* ]
	)
	.into()
}