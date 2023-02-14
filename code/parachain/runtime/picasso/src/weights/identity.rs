
//! Autogenerated weights for `identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-14, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `657e6acf5e95`, CPU: `Intel(R) Xeon(R) CPU @ 3.10GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/7as5b27dws6pfhhpjrs68qfvfx2ldcli-composable/bin/composable
// benchmark
// pallet
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=10
// --output=code/parachain/runtime/picasso/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> identity::WeightInfo for WeightInfo<T> {
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 7]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Minimum execution time: 29_131 nanoseconds.
		Weight::from_ref_time(30_089_657)
			// Standard Error: 29_897
			.saturating_add(Weight::from_ref_time(674_422).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 8]`.
	/// The range of component `x` is `[0, 32]`.
	fn set_identity(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 45_120 nanoseconds.
		Weight::from_ref_time(58_837_566)
			// Standard Error: 65_895
			.saturating_add(Weight::from_ref_time(468_350).saturating_mul(r.into()))
			// Standard Error: 15_111
			.saturating_add(Weight::from_ref_time(844_830).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:1 w:1)
	/// The range of component `s` is `[0, 32]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Minimum execution time: 18_391 nanoseconds.
		Weight::from_ref_time(45_996_921)
			// Standard Error: 37_945
			.saturating_add(Weight::from_ref_time(4_997_935).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:1)
	/// The range of component `p` is `[0, 32]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Minimum execution time: 18_517 nanoseconds.
		Weight::from_ref_time(45_901_156)
			// Standard Error: 38_022
			.saturating_add(Weight::from_ref_time(2_281_449).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:32)
	/// The range of component `r` is `[1, 8]`.
	/// The range of component `s` is `[0, 32]`.
	/// The range of component `x` is `[0, 32]`.
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Minimum execution time: 71_101 nanoseconds.
		Weight::from_ref_time(61_288_956)
			// Standard Error: 35_839
			.saturating_add(Weight::from_ref_time(298_241).saturating_mul(r.into()))
			// Standard Error: 8_224
			.saturating_add(Weight::from_ref_time(2_007_492).saturating_mul(s.into()))
			// Standard Error: 8_224
			.saturating_add(Weight::from_ref_time(407_699).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 8]`.
	/// The range of component `x` is `[0, 32]`.
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 64_290 nanoseconds.
		Weight::from_ref_time(61_628_165)
			// Standard Error: 31_892
			.saturating_add(Weight::from_ref_time(557_336).saturating_mul(r.into()))
			// Standard Error: 7_313
			.saturating_add(Weight::from_ref_time(797_491).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 8]`.
	/// The range of component `x` is `[0, 32]`.
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 58_484 nanoseconds.
		Weight::from_ref_time(58_937_426)
			// Standard Error: 29_445
			.saturating_add(Weight::from_ref_time(220_519).saturating_mul(r.into()))
			// Standard Error: 6_752
			.saturating_add(Weight::from_ref_time(791_482).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 7]`.
	fn set_fee(r: u32, ) -> Weight {
		// Minimum execution time: 15_896 nanoseconds.
		Weight::from_ref_time(16_233_696)
			// Standard Error: 9_863
			.saturating_add(Weight::from_ref_time(495_851).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 7]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Minimum execution time: 16_145 nanoseconds.
		Weight::from_ref_time(16_609_036)
			// Standard Error: 15_485
			.saturating_add(Weight::from_ref_time(502_859).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 7]`.
	fn set_fields(r: u32, ) -> Weight {
		// Minimum execution time: 16_103 nanoseconds.
		Weight::from_ref_time(16_433_773)
			// Standard Error: 11_627
			.saturating_add(Weight::from_ref_time(477_896).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 7]`.
	/// The range of component `x` is `[0, 32]`.
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 47_628 nanoseconds.
		Weight::from_ref_time(46_216_157)
			// Standard Error: 32_370
			.saturating_add(Weight::from_ref_time(497_788).saturating_mul(r.into()))
			// Standard Error: 6_519
			.saturating_add(Weight::from_ref_time(1_256_247).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Identity SuperOf (r:0 w:32)
	/// The range of component `r` is `[1, 8]`.
	/// The range of component `s` is `[0, 32]`.
	/// The range of component `x` is `[0, 32]`.
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Minimum execution time: 91_875 nanoseconds.
		Weight::from_ref_time(83_559_579)
			// Standard Error: 39_533
			.saturating_add(Weight::from_ref_time(309_928).saturating_mul(r.into()))
			// Standard Error: 9_072
			.saturating_add(Weight::from_ref_time(1_980_433).saturating_mul(s.into()))
			// Standard Error: 9_072
			.saturating_add(Weight::from_ref_time(388_816).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[0, 31]`.
	fn add_sub(s: u32, ) -> Weight {
		// Minimum execution time: 56_571 nanoseconds.
		Weight::from_ref_time(63_289_929)
			// Standard Error: 13_116
			.saturating_add(Weight::from_ref_time(362_320).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	/// The range of component `s` is `[1, 32]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Minimum execution time: 24_089 nanoseconds.
		Weight::from_ref_time(26_066_385)
			// Standard Error: 5_695
			.saturating_add(Weight::from_ref_time(160_952).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[1, 32]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Minimum execution time: 62_589 nanoseconds.
		Weight::from_ref_time(65_873_735)
			// Standard Error: 11_142
			.saturating_add(Weight::from_ref_time(325_533).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[0, 31]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Minimum execution time: 45_076 nanoseconds.
		Weight::from_ref_time(47_036_686)
			// Standard Error: 9_813
			.saturating_add(Weight::from_ref_time(276_341).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
