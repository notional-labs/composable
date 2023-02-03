
//! Autogenerated weights for `cosmwasm`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-03, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `8934575597cc`, CPU: `Intel(R) Xeon(R) CPU @ 3.10GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/386hzkyz77l1m76kfsnqr70svvd104hq-composable/bin/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=10
// --output=code/parachain/runtime/dali/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `cosmwasm`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> cosmwasm::WeightInfo for WeightInfo<T> {
	// Storage: Cosmwasm CodeHashToId (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Cosmwasm CurrentCodeId (r:1 w:1)
	// Storage: Cosmwasm PristineCode (r:0 w:1)
	// Storage: Cosmwasm InstrumentedCode (r:0 w:1)
	// Storage: Cosmwasm CodeIdToInfo (r:0 w:1)
	/// The range of component `n` is `[1, 514288]`.
	fn upload(n: u32, ) -> Weight {
		// Minimum execution time: 529_026 nanoseconds.
		Weight::from_ref_time(462_685_947 as u64)
			// Standard Error: 118
			.saturating_add(Weight::from_ref_time(107_118 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Cosmwasm CodeIdToInfo (r:1 w:1)
	// Storage: Cosmwasm ContractToInfo (r:1 w:1)
	// Storage: Cosmwasm CurrentNonce (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Cosmwasm InstrumentedCode (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Tokens Accounts (r:2 w:2)
	/// The range of component `n` is `[0, 23]`.
	fn instantiate(n: u32, ) -> Weight {
		// Minimum execution time: 375_736 nanoseconds.
		Weight::from_ref_time(410_393_105 as u64)
			// Standard Error: 92_582
			.saturating_add(Weight::from_ref_time(35_494_745 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(n as u64)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Cosmwasm ContractToInfo (r:1 w:0)
	// Storage: Cosmwasm CodeIdToInfo (r:1 w:1)
	// Storage: Cosmwasm InstrumentedCode (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Tokens Accounts (r:2 w:2)
	/// The range of component `n` is `[0, 23]`.
	fn execute(n: u32, ) -> Weight {
		// Minimum execution time: 346_100 nanoseconds.
		Weight::from_ref_time(372_057_803 as u64)
			// Standard Error: 75_705
			.saturating_add(Weight::from_ref_time(35_583_308 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(n as u64)))
	}
	// Storage: Cosmwasm ContractToInfo (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Cosmwasm CodeIdToInfo (r:2 w:2)
	// Storage: Cosmwasm InstrumentedCode (r:2 w:1)
	// Storage: Cosmwasm PristineCode (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Cosmwasm CodeHashToId (r:0 w:1)
	fn migrate() -> Weight {
		// Minimum execution time: 667_988 nanoseconds.
		Weight::from_ref_time(675_706_000 as u64)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Cosmwasm ContractToInfo (r:1 w:1)
	// Storage: Cosmwasm CodeIdToInfo (r:1 w:1)
	// Storage: Cosmwasm InstrumentedCode (r:1 w:0)
	fn update_admin() -> Weight {
		// Minimum execution time: 324_454 nanoseconds.
		Weight::from_ref_time(328_825_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: unknown [0xe9a804b2e527fd3601d2ffc0bb023cd668656c6c6f20776f726c64] (r:1 w:0)
	fn db_read() -> Weight {
		// Minimum execution time: 21_142 nanoseconds.
		Weight::from_ref_time(22_127_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: unknown [0xe9a804b2e527fd3601d2ffc0bb023cd668656c6c6f20776f726c64] (r:1 w:0)
	fn db_read_other_contract() -> Weight {
		// Minimum execution time: 21_025 nanoseconds.
		Weight::from_ref_time(21_953_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: unknown [0x46fb7408d4f285228f4af516ea25851b68656c6c6f] (r:1 w:1)
	fn db_write() -> Weight {
		// Minimum execution time: 22_118 nanoseconds.
		Weight::from_ref_time(22_704_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn db_scan() -> Weight {
		// Minimum execution time: 5_288 nanoseconds.
		Weight::from_ref_time(5_587_000 as u64)
	}
	// Storage: unknown [0x] (r:1 w:0)
	fn db_next() -> Weight {
		// Minimum execution time: 45_888 nanoseconds.
		Weight::from_ref_time(47_088_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: unknown [0x46fb7408d4f285228f4af516ea25851b68656c6c6f] (r:0 w:1)
	fn db_remove() -> Weight {
		// Minimum execution time: 9_776 nanoseconds.
		Weight::from_ref_time(9_960_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Tokens Accounts (r:1 w:0)
	fn balance() -> Weight {
		// Minimum execution time: 7_289 nanoseconds.
		Weight::from_ref_time(7_668_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: Tokens Accounts (r:2 w:2)
	/// The range of component `n` is `[0, 23]`.
	fn transfer(n: u32, ) -> Weight {
		// Minimum execution time: 362 nanoseconds.
		Weight::from_ref_time(26_053_805 as u64)
			// Standard Error: 72_605
			.saturating_add(Weight::from_ref_time(32_087_142 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(n as u64)))
	}
	// Storage: Cosmwasm ContractToInfo (r:1 w:1)
	fn set_contract_meta() -> Weight {
		// Minimum execution time: 17_932 nanoseconds.
		Weight::from_ref_time(18_760_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn running_contract_meta() -> Weight {
		// Minimum execution time: 5_211 nanoseconds.
		Weight::from_ref_time(5_465_000 as u64)
	}
	// Storage: Cosmwasm ContractToInfo (r:1 w:0)
	fn contract_meta() -> Weight {
		// Minimum execution time: 14_680 nanoseconds.
		Weight::from_ref_time(15_489_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	fn addr_validate() -> Weight {
		// Minimum execution time: 3_408 nanoseconds.
		Weight::from_ref_time(3_491_000 as u64)
	}
	fn addr_canonicalize() -> Weight {
		// Minimum execution time: 3_422 nanoseconds.
		Weight::from_ref_time(3_469_000 as u64)
	}
	fn addr_humanize() -> Weight {
		// Minimum execution time: 283 nanoseconds.
		Weight::from_ref_time(320_000 as u64)
	}
	fn secp256k1_recover_pubkey() -> Weight {
		// Minimum execution time: 39_646 nanoseconds.
		Weight::from_ref_time(40_005_000 as u64)
	}
	fn secp256k1_verify() -> Weight {
		// Minimum execution time: 39_711 nanoseconds.
		Weight::from_ref_time(40_288_000 as u64)
	}
	fn ed25519_verify() -> Weight {
		// Minimum execution time: 56_867 nanoseconds.
		Weight::from_ref_time(57_247_000 as u64)
	}
	fn ed25519_batch_verify() -> Weight {
		// Minimum execution time: 135_886 nanoseconds.
		Weight::from_ref_time(141_057_000 as u64)
	}
	// Storage: Cosmwasm CodeIdToInfo (r:1 w:1)
	// Storage: Cosmwasm ContractToInfo (r:1 w:1)
	// Storage: Cosmwasm CurrentNonce (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Tokens Accounts (r:2 w:2)
	/// The range of component `n` is `[0, 23]`.
	fn continue_instantiate(n: u32, ) -> Weight {
		// Minimum execution time: 317_594 nanoseconds.
		Weight::from_ref_time(352_396_429 as u64)
			// Standard Error: 100_232
			.saturating_add(Weight::from_ref_time(33_207_693 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(n as u64)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Cosmwasm ContractToInfo (r:1 w:0)
	/// The range of component `n` is `[0, 23]`.
	fn continue_execute(n: u32, ) -> Weight {
		// Minimum execution time: 281_259 nanoseconds.
		Weight::from_ref_time(290_334_878 as u64)
			// Standard Error: 31_256
			.saturating_add(Weight::from_ref_time(1_413_583 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
	}
	// Storage: Cosmwasm ContractToInfo (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	fn continue_migrate() -> Weight {
		// Minimum execution time: 280_212 nanoseconds.
		Weight::from_ref_time(288_490_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Cosmwasm ContractToInfo (r:1 w:0)
	fn continue_query() -> Weight {
		// Minimum execution time: 264_936 nanoseconds.
		Weight::from_ref_time(269_519_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Cosmwasm ContractToInfo (r:1 w:0)
	fn continue_reply() -> Weight {
		// Minimum execution time: 278_869 nanoseconds.
		Weight::from_ref_time(285_797_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
	}
	// Storage: Cosmwasm CodeIdToInfo (r:1 w:0)
	fn query_info() -> Weight {
		// Minimum execution time: 28_601 nanoseconds.
		Weight::from_ref_time(29_263_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Cosmwasm ContractToInfo (r:1 w:0)
	// Storage: unknown [0x46fb7408d4f285228f4af516ea25851b68656c6c6f] (r:1 w:0)
	fn query_raw() -> Weight {
		// Minimum execution time: 33_325 nanoseconds.
		Weight::from_ref_time(34_727_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Const(r: u32, ) -> Weight {
		// Minimum execution time: 4_007 nanoseconds.
		Weight::from_ref_time(4_414_217 as u64)
			// Standard Error: 1_274
			.saturating_add(Weight::from_ref_time(996_364 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Const(r: u32, ) -> Weight {
		// Minimum execution time: 4_133 nanoseconds.
		Weight::from_ref_time(4_533_340 as u64)
			// Standard Error: 1_285
			.saturating_add(Weight::from_ref_time(996_725 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Load(r: u32, ) -> Weight {
		// Minimum execution time: 3_883 nanoseconds.
		Weight::from_ref_time(4_558_706 as u64)
			// Standard Error: 2_808
			.saturating_add(Weight::from_ref_time(1_908_098 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Load(r: u32, ) -> Weight {
		// Minimum execution time: 4_007 nanoseconds.
		Weight::from_ref_time(4_593_740 as u64)
			// Standard Error: 3_159
			.saturating_add(Weight::from_ref_time(1_899_371 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Store(r: u32, ) -> Weight {
		// Minimum execution time: 4_117 nanoseconds.
		Weight::from_ref_time(4_495_609 as u64)
			// Standard Error: 4_298
			.saturating_add(Weight::from_ref_time(3_096_029 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Store(r: u32, ) -> Weight {
		// Minimum execution time: 4_016 nanoseconds.
		Weight::from_ref_time(4_475_602 as u64)
			// Standard Error: 4_359
			.saturating_add(Weight::from_ref_time(3_113_968 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Eq(r: u32, ) -> Weight {
		// Minimum execution time: 4_050 nanoseconds.
		Weight::from_ref_time(4_744_808 as u64)
			// Standard Error: 3_389
			.saturating_add(Weight::from_ref_time(2_366_357 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Eqz(r: u32, ) -> Weight {
		// Minimum execution time: 4_112 nanoseconds.
		Weight::from_ref_time(4_793_354 as u64)
			// Standard Error: 1_717
			.saturating_add(Weight::from_ref_time(1_753_745 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Ne(r: u32, ) -> Weight {
		// Minimum execution time: 4_110 nanoseconds.
		Weight::from_ref_time(4_887_038 as u64)
			// Standard Error: 3_190
			.saturating_add(Weight::from_ref_time(2_361_030 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64LtS(r: u32, ) -> Weight {
		// Minimum execution time: 4_124 nanoseconds.
		Weight::from_ref_time(4_786_036 as u64)
			// Standard Error: 2_634
			.saturating_add(Weight::from_ref_time(2_360_668 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64GtS(r: u32, ) -> Weight {
		// Minimum execution time: 4_235 nanoseconds.
		Weight::from_ref_time(4_960_172 as u64)
			// Standard Error: 4_069
			.saturating_add(Weight::from_ref_time(2_363_567 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64LeS(r: u32, ) -> Weight {
		// Minimum execution time: 4_063 nanoseconds.
		Weight::from_ref_time(4_828_206 as u64)
			// Standard Error: 2_095
			.saturating_add(Weight::from_ref_time(2_355_652 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64GeS(r: u32, ) -> Weight {
		// Minimum execution time: 4_135 nanoseconds.
		Weight::from_ref_time(4_842_322 as u64)
			// Standard Error: 2_808
			.saturating_add(Weight::from_ref_time(2_358_592 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Clz(r: u32, ) -> Weight {
		// Minimum execution time: 4_015 nanoseconds.
		Weight::from_ref_time(4_607_758 as u64)
			// Standard Error: 1_647
			.saturating_add(Weight::from_ref_time(1_754_578 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Ctz(r: u32, ) -> Weight {
		// Minimum execution time: 4_253 nanoseconds.
		Weight::from_ref_time(4_782_522 as u64)
			// Standard Error: 1_770
			.saturating_add(Weight::from_ref_time(1_748_182 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Popcnt(r: u32, ) -> Weight {
		// Minimum execution time: 4_131 nanoseconds.
		Weight::from_ref_time(4_721_645 as u64)
			// Standard Error: 1_853
			.saturating_add(Weight::from_ref_time(1_748_587 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Add(r: u32, ) -> Weight {
		// Minimum execution time: 4_055 nanoseconds.
		Weight::from_ref_time(4_795_649 as u64)
			// Standard Error: 3_862
			.saturating_add(Weight::from_ref_time(2_398_854 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Sub(r: u32, ) -> Weight {
		// Minimum execution time: 4_292 nanoseconds.
		Weight::from_ref_time(4_928_215 as u64)
			// Standard Error: 4_724
			.saturating_add(Weight::from_ref_time(2_394_012 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Mul(r: u32, ) -> Weight {
		// Minimum execution time: 4_192 nanoseconds.
		Weight::from_ref_time(4_802_248 as u64)
			// Standard Error: 3_914
			.saturating_add(Weight::from_ref_time(2_396_113 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64DivS(r: u32, ) -> Weight {
		// Minimum execution time: 4_190 nanoseconds.
		Weight::from_ref_time(4_808_331 as u64)
			// Standard Error: 3_728
			.saturating_add(Weight::from_ref_time(3_081_612 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64DivU(r: u32, ) -> Weight {
		// Minimum execution time: 4_128 nanoseconds.
		Weight::from_ref_time(4_848_517 as u64)
			// Standard Error: 4_511
			.saturating_add(Weight::from_ref_time(2_649_801 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64RemS(r: u32, ) -> Weight {
		// Minimum execution time: 4_093 nanoseconds.
		Weight::from_ref_time(4_702_447 as u64)
			// Standard Error: 3_545
			.saturating_add(Weight::from_ref_time(3_003_120 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64And(r: u32, ) -> Weight {
		// Minimum execution time: 4_186 nanoseconds.
		Weight::from_ref_time(5_051_274 as u64)
			// Standard Error: 1_841
			.saturating_add(Weight::from_ref_time(2_382_193 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Or(r: u32, ) -> Weight {
		// Minimum execution time: 4_115 nanoseconds.
		Weight::from_ref_time(5_009_194 as u64)
			// Standard Error: 3_748
			.saturating_add(Weight::from_ref_time(2_394_279 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Xor(r: u32, ) -> Weight {
		// Minimum execution time: 4_240 nanoseconds.
		Weight::from_ref_time(4_889_571 as u64)
			// Standard Error: 2_957
			.saturating_add(Weight::from_ref_time(2_391_316 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Shl(r: u32, ) -> Weight {
		// Minimum execution time: 3_989 nanoseconds.
		Weight::from_ref_time(4_763_349 as u64)
			// Standard Error: 2_773
			.saturating_add(Weight::from_ref_time(2_358_590 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64ShrS(r: u32, ) -> Weight {
		// Minimum execution time: 4_066 nanoseconds.
		Weight::from_ref_time(5_019_974 as u64)
			// Standard Error: 3_104
			.saturating_add(Weight::from_ref_time(2_349_547 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Rotl(r: u32, ) -> Weight {
		// Minimum execution time: 4_193 nanoseconds.
		Weight::from_ref_time(4_794_048 as u64)
			// Standard Error: 2_890
			.saturating_add(Weight::from_ref_time(2_356_641 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64Rotr(r: u32, ) -> Weight {
		// Minimum execution time: 4_207 nanoseconds.
		Weight::from_ref_time(4_695_795 as u64)
			// Standard Error: 5_976
			.saturating_add(Weight::from_ref_time(2_369_553 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I64ExtendSI32(r: u32, ) -> Weight {
		// Minimum execution time: 4_300 nanoseconds.
		Weight::from_ref_time(4_747_139 as u64)
			// Standard Error: 2_258
			.saturating_add(Weight::from_ref_time(1_751_845 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_I32WrapI64(r: u32, ) -> Weight {
		// Minimum execution time: 4_235 nanoseconds.
		Weight::from_ref_time(4_797_559 as u64)
			// Standard Error: 3_300
			.saturating_add(Weight::from_ref_time(1_739_486 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Eq(r: u32, ) -> Weight {
		// Minimum execution time: 4_165 nanoseconds.
		Weight::from_ref_time(5_018_704 as u64)
			// Standard Error: 3_070
			.saturating_add(Weight::from_ref_time(2_369_905 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Ne(r: u32, ) -> Weight {
		// Minimum execution time: 4_047 nanoseconds.
		Weight::from_ref_time(4_830_355 as u64)
			// Standard Error: 3_539
			.saturating_add(Weight::from_ref_time(2_376_775 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Lt(r: u32, ) -> Weight {
		// Minimum execution time: 3_969 nanoseconds.
		Weight::from_ref_time(4_888_917 as u64)
			// Standard Error: 3_136
			.saturating_add(Weight::from_ref_time(2_353_749 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Gt(r: u32, ) -> Weight {
		// Minimum execution time: 4_156 nanoseconds.
		Weight::from_ref_time(4_886_076 as u64)
			// Standard Error: 3_949
			.saturating_add(Weight::from_ref_time(2_357_245 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Le(r: u32, ) -> Weight {
		// Minimum execution time: 4_213 nanoseconds.
		Weight::from_ref_time(4_975_073 as u64)
			// Standard Error: 3_973
			.saturating_add(Weight::from_ref_time(2_403_255 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Ge(r: u32, ) -> Weight {
		// Minimum execution time: 4_244 nanoseconds.
		Weight::from_ref_time(4_877_847 as u64)
			// Standard Error: 4_169
			.saturating_add(Weight::from_ref_time(2_404_081 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Abs(r: u32, ) -> Weight {
		// Minimum execution time: 3_997 nanoseconds.
		Weight::from_ref_time(4_593_334 as u64)
			// Standard Error: 2_590
			.saturating_add(Weight::from_ref_time(1_755_647 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Neg(r: u32, ) -> Weight {
		// Minimum execution time: 4_094 nanoseconds.
		Weight::from_ref_time(4_916_014 as u64)
			// Standard Error: 2_467
			.saturating_add(Weight::from_ref_time(1_731_796 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Ceil(r: u32, ) -> Weight {
		// Minimum execution time: 4_141 nanoseconds.
		Weight::from_ref_time(4_803_479 as u64)
			// Standard Error: 3_110
			.saturating_add(Weight::from_ref_time(1_721_337 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Floor(r: u32, ) -> Weight {
		// Minimum execution time: 4_047 nanoseconds.
		Weight::from_ref_time(4_649_606 as u64)
			// Standard Error: 2_063
			.saturating_add(Weight::from_ref_time(1_723_984 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Trunc(r: u32, ) -> Weight {
		// Minimum execution time: 4_060 nanoseconds.
		Weight::from_ref_time(4_530_565 as u64)
			// Standard Error: 4_320
			.saturating_add(Weight::from_ref_time(1_729_904 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Nearest(r: u32, ) -> Weight {
		// Minimum execution time: 4_058 nanoseconds.
		Weight::from_ref_time(4_919_002 as u64)
			// Standard Error: 2_849
			.saturating_add(Weight::from_ref_time(1_860_823 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Sqrt(r: u32, ) -> Weight {
		// Minimum execution time: 4_154 nanoseconds.
		Weight::from_ref_time(4_685_548 as u64)
			// Standard Error: 3_630
			.saturating_add(Weight::from_ref_time(1_796_987 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Add(r: u32, ) -> Weight {
		// Minimum execution time: 4_200 nanoseconds.
		Weight::from_ref_time(4_675_518 as u64)
			// Standard Error: 2_967
			.saturating_add(Weight::from_ref_time(2_360_411 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Sub(r: u32, ) -> Weight {
		// Minimum execution time: 4_062 nanoseconds.
		Weight::from_ref_time(4_951_447 as u64)
			// Standard Error: 5_053
			.saturating_add(Weight::from_ref_time(2_357_263 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Mul(r: u32, ) -> Weight {
		// Minimum execution time: 4_087 nanoseconds.
		Weight::from_ref_time(4_610_411 as u64)
			// Standard Error: 3_912
			.saturating_add(Weight::from_ref_time(2_369_437 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Div(r: u32, ) -> Weight {
		// Minimum execution time: 4_196 nanoseconds.
		Weight::from_ref_time(5_079_532 as u64)
			// Standard Error: 3_126
			.saturating_add(Weight::from_ref_time(2_351_264 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Min(r: u32, ) -> Weight {
		// Minimum execution time: 4_140 nanoseconds.
		Weight::from_ref_time(4_929_348 as u64)
			// Standard Error: 4_109
			.saturating_add(Weight::from_ref_time(2_412_959 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Max(r: u32, ) -> Weight {
		// Minimum execution time: 3_977 nanoseconds.
		Weight::from_ref_time(4_809_859 as u64)
			// Standard Error: 4_541
			.saturating_add(Weight::from_ref_time(2_422_526 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_F64Copysign(r: u32, ) -> Weight {
		// Minimum execution time: 4_023 nanoseconds.
		Weight::from_ref_time(4_851_431 as u64)
			// Standard Error: 3_465
			.saturating_add(Weight::from_ref_time(2_357_419 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_Select(r: u32, ) -> Weight {
		// Minimum execution time: 4_134 nanoseconds.
		Weight::from_ref_time(4_762_410 as u64)
			// Standard Error: 4_898
			.saturating_add(Weight::from_ref_time(3_087_541 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_If(r: u32, ) -> Weight {
		// Minimum execution time: 3_992 nanoseconds.
		Weight::from_ref_time(4_487_372 as u64)
			// Standard Error: 2_443
			.saturating_add(Weight::from_ref_time(1_391_246 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_Else(r: u32, ) -> Weight {
		// Minimum execution time: 4_033 nanoseconds.
		Weight::from_ref_time(4_478_544 as u64)
			// Standard Error: 2_560
			.saturating_add(Weight::from_ref_time(2_127_921 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_GetLocal(r: u32, ) -> Weight {
		// Minimum execution time: 4_016 nanoseconds.
		Weight::from_ref_time(4_515_659 as u64)
			// Standard Error: 1_078
			.saturating_add(Weight::from_ref_time(1_064_888 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_SetLocal(r: u32, ) -> Weight {
		// Minimum execution time: 4_133 nanoseconds.
		Weight::from_ref_time(4_649_093 as u64)
			// Standard Error: 1_257
			.saturating_add(Weight::from_ref_time(1_300_174 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_TeeLocal(r: u32, ) -> Weight {
		// Minimum execution time: 4_123 nanoseconds.
		Weight::from_ref_time(4_450_963 as u64)
			// Standard Error: 657
			.saturating_add(Weight::from_ref_time(222 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_GetGlobal(_r: u32, ) -> Weight {
		// Minimum execution time: 4_050 nanoseconds.
		Weight::from_ref_time(4_486_631 as u64)
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_SetGlobal(r: u32, ) -> Weight {
		// Minimum execution time: 4_090 nanoseconds.
		Weight::from_ref_time(4_424_686 as u64)
			// Standard Error: 549
			.saturating_add(Weight::from_ref_time(69 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_CurrentMemory(r: u32, ) -> Weight {
		// Minimum execution time: 4_114 nanoseconds.
		Weight::from_ref_time(4_564_172 as u64)
			// Standard Error: 2_630
			.saturating_add(Weight::from_ref_time(1_369_536 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 3]`.
	fn instruction_GrowMemory(r: u32, ) -> Weight {
		// Minimum execution time: 3_957 nanoseconds.
		Weight::from_ref_time(4_177_000 as u64)
			// Standard Error: 18_857_030
			.saturating_add(Weight::from_ref_time(2_727_389_409 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_Br(r: u32, ) -> Weight {
		// Minimum execution time: 4_191 nanoseconds.
		Weight::from_ref_time(4_461_002 as u64)
			// Standard Error: 1_860
			.saturating_add(Weight::from_ref_time(720_617 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_BrIf(r: u32, ) -> Weight {
		// Minimum execution time: 3_904 nanoseconds.
		Weight::from_ref_time(4_347_463 as u64)
			// Standard Error: 1_312
			.saturating_add(Weight::from_ref_time(1_373_569 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_BrTable(r: u32, ) -> Weight {
		// Minimum execution time: 4_051 nanoseconds.
		Weight::from_ref_time(4_548_881 as u64)
			// Standard Error: 1_696
			.saturating_add(Weight::from_ref_time(1_872_800 as u64).saturating_mul(r as u64))
	}
	/// The range of component `s` is `[1, 50]`.
	fn instruction_BrTable_per_elem(s: u32, ) -> Weight {
		// Minimum execution time: 6_071 nanoseconds.
		Weight::from_ref_time(6_427_488 as u64)
			// Standard Error: 956
			.saturating_add(Weight::from_ref_time(8_496 as u64).saturating_mul(s as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_Call(r: u32, ) -> Weight {
		// Minimum execution time: 4_175 nanoseconds.
		Weight::from_ref_time(4_544_950 as u64)
			// Standard Error: 7_274
			.saturating_add(Weight::from_ref_time(9_518_543 as u64).saturating_mul(r as u64))
	}
	/// The range of component `r` is `[0, 50]`.
	fn instruction_CallIndirect(r: u32, ) -> Weight {
		// Minimum execution time: 4_431 nanoseconds.
		Weight::from_ref_time(4_569_649 as u64)
			// Standard Error: 15_247
			.saturating_add(Weight::from_ref_time(11_304_508 as u64).saturating_mul(r as u64))
	}
}
