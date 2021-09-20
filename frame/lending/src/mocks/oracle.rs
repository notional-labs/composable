pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::Codec;
	use composable_traits::{oracle::Oracle, vault::Vault};
	use frame_support::{pallet_prelude::*, PalletId};
	use sp_runtime::{helpers_128bit::multiply_by_rational, ArithmeticError};
	use sp_std::fmt::Debug;

	use crate::mocks::{Balance, MockCurrencyId};

	pub const PALLET_ID: PalletId = PalletId(*b"mck_orac");

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type VaultId: Clone + Codec + Debug + PartialEq + Default + Parameter;
		type Vault: Vault<AssetId = MockCurrencyId, VaultId = Self::VaultId, Balance = Balance>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	impl<T: Config> Pallet<T> {
		fn get_price(of: &MockCurrencyId) -> Result<(Balance, ()), DispatchError> {
			<Self as Oracle>::get_price(of)
		}
	}

	impl<T: Config> Oracle for Pallet<T> {
		type AssetId = MockCurrencyId;
		type Balance = Balance;
		type Timestamp = ();

		fn get_price(
			of: &Self::AssetId,
		) -> Result<(Self::Balance, Self::Timestamp), DispatchError> {
			let usd_mul = |k| Self::get_price(&MockCurrencyId::USDT).map(|(x, y)| (x * k, y));
			match of {
				/*
					Ideally we would have all the static currency quoted against USD cents on chain.
					So that we would be able to derive LP tokens price.
				*/
				MockCurrencyId::USDT => Ok((100, ())),
				MockCurrencyId::PICA => usd_mul(10),
				MockCurrencyId::BTC => usd_mul(50000),
				MockCurrencyId::ETH => usd_mul(3000),
				MockCurrencyId::LTC => usd_mul(200),
				/*
					If we want users to be able to consider LP tokens as currency,
					the oracle should know the whole currency system in order to
					recursively resolve the price of an LP token generated by an
					arbitrary level of vaults.

					The base asset represented by the level 0 (out of LpToken case)
					should have a price defined.

					One exception can occur if the LP token hasn't been generated by a vault.
				*/
				&x @ MockCurrencyId::LpToken(_) => {
					let vault = T::Vault::token_vault(x)?;
					let base_asset = T::Vault::asset_id(&vault)?;
					let lp_amount = 1000;
					let base_amount = T::Vault::to_underlying_value(&vault, lp_amount)?;
					let (p, t) = Self::get_price(&base_asset)?;
					Ok((
						multiply_by_rational(p, base_amount, lp_amount)
							.map_err(|_| DispatchError::Arithmetic(ArithmeticError::Overflow))?,
						t,
					))
				},
			}
		}
	}
}
