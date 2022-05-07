#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::Currency;
use sp_runtime::traits::StaticLookup;
use scale_info::TypeInfo;
use sp_core::crypto::UncheckedFrom;
use codec::{Encode, HasCompact};
use sp_std::{fmt::Debug, prelude::*};

type BalanceOf<T> = <<T as ContractsConfig>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

pub use pallet::*;
pub use pallet_contracts::{
	Config as ContractsConfig, Pallet as ContractsPallet,
};


#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: ContractsConfig + frame_system::Config {
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> 
		where
		T::AccountId: UncheckedFrom<T::Hash>,
		T::AccountId: AsRef<[u8]>,
		<BalanceOf<T> as HasCompact>::Type: Clone + Eq + PartialEq + Debug + TypeInfo + Encode,
	{
		#[pallet::weight(0)]
		pub fn sudo_call(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] value: BalanceOf<T>,
			#[pallet::compact] gas_limit: Weight,
			storage_deposit_limit: Option<<BalanceOf<T> as codec::HasCompact>::Type>,
			data: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin.clone())?;

			ContractsPallet::<T>::call(
				origin,
				dest,
				value,
				gas_limit,
				storage_deposit_limit,
				data,
			)
		}

	}
}