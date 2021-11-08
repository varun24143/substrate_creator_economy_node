//! The Yodariver Runtime.
//!
//! Primary features of this runtime include:
//! * Ethereum compatibility
//! * Yodariver tokenomics

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

#[cfg(feature = "std")]
include!("/wasm_binary.rs");

use frame_support::{
	construct_runtime, parameter_types,
	traits::{Contains, Everything, Get, Imbalance, InstanceFilter, OnUnbalanced},
	weights::{
		constants::{RocksDbWeight, WEIGHT_PER_SECOND},
		DispatchClass, GetDispatchInfo, IdentityFee, Weight,
	},
	PalletId,
};
use frame_system::{EnsureOneOf, EnsureRoot, EnsureSigned};
pub use yoda_core_primitives::{
	AccountId, AccountIndex, Address, Balance, BlockNumber, DigestItem, Hash, Header, Index,
	Signature,
};



