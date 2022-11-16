/// Initialize a key-value collection from array.
///
/// Creates a vector of given pairs and calls `collect` on the iterator from it.
/// Can be used to create a `HashMap`.
#[macro_export]
macro_rules! map {
	($( $name:expr => $value:expr ),* $(,)? ) => (
		vec![ $( ( $name, $value ) ),* ].into_iter().collect()
	);
}


pub mod crypto;
pub mod ecdsa;
pub mod hash;
#[cfg(feature = "std")]
mod hasher;
pub mod hashing;
pub mod hexdisplay;
pub mod uint;
pub mod ed25519;
pub mod traits;
pub mod testing;


#[cfg(feature = "full_crypto")]
pub use hashing::{blake2_128, blake2_256, keccak_256, twox_128, twox_256, twox_64};

#[cfg(feature = "full_crypto")]
pub use crypto::{ByteArray, DeriveJunction, Pair, Public};

#[cfg(feature = "std")]
pub use self::hasher::blake2::Blake2Hasher;
#[cfg(feature = "std")]
pub use self::hasher::keccak::KeccakHasher;

pub use self::{
	hash::{convert_hash, H160, H256, H512},
	uint::{U256, U512},
};

pub use kp_storage as storage;
pub use hash_db::Hasher;
