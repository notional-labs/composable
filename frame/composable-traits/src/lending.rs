use crate::vault::Deposit;
use codec::Codec;
use frame_support::{
	pallet_prelude::*,
	sp_runtime::{Permill, Perquintill},
	sp_std::{fmt::Debug, vec::Vec},
};

/// The fixed point number of suggested by substrate precision
/// Must be (1.0.. because applied only to price normalized values
pub type NormalizedCollateralFactor = frame_support::sp_runtime::FixedU128;

#[derive(Encode, Decode, Default)]
pub struct MarketConfigInput<AccountId>
where
	AccountId: core::cmp::Ord,
{
	/// can pause borrow & deposits of assets
	pub manager: AccountId,
	pub reserve_factor: Perquintill,
	pub collateral_factor: NormalizedCollateralFactor,
}

#[derive(Encode, Decode, Default)]
pub struct MarketConfig<VaultId> {
	pub borrow: VaultId,
	pub collateral: VaultId,
	pub reserve_factor: Perquintill,
	pub collateral_factor: NormalizedCollateralFactor,
}

/// Basic lending with no its own wrapper (liquidity) token.
///  User will deposit borrow and collateral assets via `Vault`.
/// `Liquidation` is other trait.
/// Based on Blacksmith (Warp v2) IBSLendingPair.sol and Parallel Finance.
/// Fees will be withdrawing to vault.
/// Lenders with be rewarded via vault.
pub trait Lending {
	type VaultId: Codec;
	type MarketId: Codec;
	/// (deposit VaultId, collateral VaultId) <-> PairId
	type AccountId: core::cmp::Ord + Clone + Codec;
	type Balance;
	type BlockNumber;

	/// creates market for new pair in specified vault. if market exists under specified manager,
	/// updates its parameters `deposit` - asset users want to borrow.
	/// `collateral` - asset users will put as collateral.
	fn create_or_update(
		borrow_asset_vault: Self::VaultId,
		collateral_asset_vault: Self::VaultId,
		config: MarketConfigInput<Self::AccountId>,
	) -> Result<(), DispatchError>;

	/// AccountId of the market instance
	fn account_id(market_id: &Self::MarketId) -> Self::AccountId;

	fn deposit_collateral(
		market_id: &Self::MarketId,
		account_id: &Self::AccountId,
		amount: Self::Balance,
	) -> Result<(), DispatchError>;

	/// get all existing markets for current deposit
	fn get_markets_for_borrow(vault: Self::VaultId) -> Vec<Self::MarketId>;

	fn get_all_markets() -> Vec<(Self::MarketId, MarketConfig<Self::VaultId>)>;

	/// `amount_to_borrow` is the amount of the borrow asset lendings's vault shares the user wants to borrow.
	/// Normalizes amounts for calculations.
	/// Borrows as exact amount as possible with some inaccuracies for oracle price based normalization.
	/// If there is not enough collateral or borrow amounts - fails
	fn borrow(
		market_id: &Self::MarketId,
		debt_owner: &Self::AccountId,
		amount_to_borrow: Self::Balance,
	) -> Result<(), DispatchError>;

	/// `from` repays some of `beneficiary` debts.
	///
	/// - `pair`        : the pair to be repaid.
	/// - `repay_amount`: the amount to be repaid.
	fn repay_borrow(
		market_id: &Self::MarketId,
		from: &Self::AccountId,
		beneficiary: &Self::AccountId,
		repay_amount: Self::Balance,
	) -> Result<(), DispatchError>;

	fn total_borrows(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError>;

	fn accrue_interest(market_id: &Self::MarketId) -> Result<(), DispatchError>;

	fn borrow_balance_current(
		market_id: &Self::MarketId,
		account: &Self::AccountId,
	) -> Result<Self::Balance, DispatchError>;

	fn collateral_of_account(
		market_id: &Self::MarketId,
		account: &Self::AccountId,
	) -> Result<Self::Balance, DispatchError>;

	/// Borrower shouldn't borrow more than his total collateral value
	fn collateral_required(
		market_id: &Self::MarketId,
		borrow_amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;

	/// Returns the borrow limit for an account.
	/// Calculation uses current values for calculations, so can change during call to `borrow`.
	/// Depends on overall collateral put by user into vault.
	/// This borrow limit of specific user, depends only on prices and users collateral, not on state of vault.
	fn get_borrow_limit(
		market_id: &Self::MarketId,
		account: Self::AccountId,
	) -> Result<Self::Balance, DispatchError>;
}
