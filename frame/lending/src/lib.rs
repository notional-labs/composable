//!

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
	bad_style,
	bare_trait_objects,
	const_err,
	improper_ctypes,
	non_shorthand_field_patterns,
	no_mangle_generic_items,
	overflowing_literals,
	path_statements,
	patterns_in_fns_without_body,
	private_in_public,
	unconditional_recursion,
	unused_allocation,
	unused_comparisons,
	unused_parens,
	while_true,
	trivial_casts,
	trivial_numeric_casts,
	unused_extern_crates
)]

mod rate_model;

#[frame_support::pallet]
pub mod pallet {

	use codec::{Codec, EncodeLike, FullCodec};
	use composable_traits::{
		lending::{Lending, LendingConfigInput},
		oracle::Oracle,
		vault::{Deposit, Vault, VaultConfig},
	};
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungibles::{Inspect, Mutate, Transfer},
			tokens::{fungibles::MutateHold, DepositConsequence},
		},
		PalletId,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor, Config as SystemConfig};
	use num_traits::{Bounded, SaturatingSub};
	use sp_runtime::{
		helpers_128bit::multiply_by_rational,
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, Convert,
			Hash, Zero,
		},
		Permill, Perquintill,
	};
	use sp_std::fmt::Debug;

	#[derive(Default, Copy, Clone, Encode, Decode)]
	#[repr(transparent)]
	pub struct MarketIndex(u32);

	pub const PALLET_ID: PalletId = PalletId(*b"Lending!");

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Oracle: Oracle<AssetId = Self::AssetId>;
		type VaultId: Clone + Codec + Debug + PartialEq + Default + Parameter;
		type Vault: Vault<VaultId = Self::VaultId, AssetId = Self::AssetId>;
		type AssetId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default;
		type Balance: Default
			+ Parameter
			+ Codec
			+ Copy
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ SaturatingSub
			+ AtLeast32BitUnsigned
			+ Zero
			+ From<u64>;
		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>;

		type Convert: Convert<Self::Balance, u128> + Convert<u128, Self::Balance>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::error]
	pub enum Error<T> {
		Overflow,
		/// vault provided does not exist
		VaultNotFound,
		/// Only assets for which we can track price are supported
		AssetWithoutPrice,
		/// The market could not be found
		MarketDoesNotExist,
		CollateralDepositFailed,
	}

	#[derive(Encode, Decode, Default)]
	pub struct MarketConfig<VaultId> {
		pub borrow_asset_vault: VaultId,
		pub collateral_asset_vault: VaultId,
		pub reserve_factor: Permill,
		pub collateral_factor: Permill,
	}

	/// Lending instances counter
	#[pallet::storage]
	#[pallet::getter(fn lending_count)]
	pub type LendingCount<T: Config> = StorageValue<_, MarketIndex, ValueQuery>;

	/// Indexed lending instances
	#[pallet::storage]
	#[pallet::getter(fn pairs)]
	pub type Markets<T: Config> =
		StorageMap<_, Twox64Concat, MarketIndex, MarketConfig<T::VaultId>, ValueQuery>;

	/// (Market, Account) -> Collateral
	#[pallet::storage]
	#[pallet::getter(fn account_collateral)]
	pub type AccountCollateral<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		MarketIndex,
		Blake2_128Concat,
		T::AccountId,
		T::Balance,
		ValueQuery,
	>;

	impl<T: Config> Lending for Pallet<T> {
		type VaultId = <T::Vault as Vault>::VaultId;

		type AccountId = T::AccountId;

		type MarketId = MarketIndex;

		type Balance = T::Balance;

		type BlockNumber = T::BlockNumber;

		fn create_or_update(
			borrow_asset_vault: <T::Vault as Vault>::VaultId,
			collateral_asset_vault: <T::Vault as Vault>::VaultId,
			config_input: LendingConfigInput<Self::AccountId>,
		) -> Result<(), DispatchError> {
			let collateral_asset = T::Vault::asset_id(&collateral_asset_vault)?;
			let borrow_asset = T::Vault::asset_id(&borrow_asset_vault)?;

			<T::Oracle as Oracle>::get_price(collateral_asset)
				.map_err(|_| Error::<T>::AssetWithoutPrice)?;
			<T::Oracle as Oracle>::get_price(borrow_asset)
				.map_err(|_| Error::<T>::AssetWithoutPrice)?;

			LendingCount::<T>::try_mutate(|MarketIndex(previous_lending_index)| {
				let lending_index = {
					*previous_lending_index += 1;
					MarketIndex(*previous_lending_index)
				};

				let config = MarketConfig {
					borrow_asset_vault,
					collateral_asset_vault,
					reserve_factor: config_input.reserve_factor,
					collateral_factor: config_input.collateral_factor,
				};

				Markets::<T>::insert(lending_index, config);

				Ok(())
			})
		}

		fn account_id(market_id: &Self::MarketId) -> Self::AccountId {
			PALLET_ID.into_sub_account(market_id)
		}

		fn get_pair_in_vault(vault: Self::VaultId) -> Result<Vec<Self::MarketId>, DispatchError> {
			todo!()
		}

		fn get_pairs_all() -> Result<Vec<Self::MarketId>, DispatchError> {
			todo!()
		}

		fn borrow(
			market_id: &Self::MarketId,
			debt_owner: &Self::AccountId,
			amount_to_borrow: Self::Balance,
		) -> Result<(), DispatchError> {
			todo!()
		}

		fn repay_borrow(
			market_id: &Self::MarketId,
			from: &Self::AccountId,
			beneficiary: &Self::AccountId,
			repay_amount: Self::Balance,
		) -> Result<(), DispatchError> {
			todo!()
		}

		fn total_borrows(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn accrue_interest(market_id: &Self::MarketId) -> Result<(), DispatchError> {
			todo!()
		}

		fn borrow_balance_current(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn collateral_of_account(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn collateral_required(
			market_id: &Self::MarketId,
			borrow_amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn get_borrow_limit(
			market_id: &Self::MarketId,
			account: Self::AccountId,
		) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn deposit_collateral(
			market_id: &Self::MarketId,
			from: &Self::AccountId,
			amount: Self::Balance,
		) -> Result<(), DispatchError> {
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let collateral_lp_id = T::Vault::lp_asset_id(&market.collateral_asset_vault)?;
			T::Currency::transfer(
				collateral_lp_id,
				from,
				&Self::account_id(market_id),
				amount,
				true,
			)
			.map_err(|_| Error::<T>::CollateralDepositFailed)?;
			AccountCollateral::<T>::try_mutate(market_id, from, |collateral_balance| {
				let new_collateral_balance =
					T::Convert::convert(*collateral_balance) + T::Convert::convert(amount);
				*collateral_balance = T::Convert::convert(new_collateral_balance);
				Ok(())
			})
		}
	}
}
