//! Facade of currency implementation. Useful while migrating from old to new currency system.

use frame_support::{
	defensive, ensure,
	traits::{Defensive, InspectLockableCurrency, LockableCurrency, Currency},
};
use sp_staking::{StakingAccount, StakingInterface};

use crate::{
	BalanceOf, Bonded, Config, Error, Ledger, Pallet, Payee, RewardDestination, StakingLedger,
	VirtualStakers, STAKING_ID,
};

/// Balance that is staked and at stake.
pub fn staked<T: Config>(who: &T::AccountId) -> BalanceOf<T> {
	T::Currency::balance_locked(crate::STAKING_ID, who)
}

/// Existential deposit for the chain.
pub fn existential_deposit<T: Config>() -> BalanceOf<T> {
    T::Currency::minimum_balance()
}

pub fn burn<T: Config>(amount: BalanceOf<T>) {
	T::Currency::burn(amount);
}

pub fn total_issuance<T: Config>() -> BalanceOf<T> {
	T::Currency::total_issuance()
}

pub fn set_balance<T: Config>(who: &T::AccountId, value: BalanceOf<T>) {
	T::Currency::make_free_balance_be(who, value);
}