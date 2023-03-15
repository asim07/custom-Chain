#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{inherent::Vec, pallet_prelude::*, sp_runtime};
	use frame_system::pallet_prelude::*;
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_assets::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn get_asset_id)]
	pub type AssetId<T: Config> = StorageValue<_, T::AssetIdParameter, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored {
			something: u32,
			who: T::AccountId,
		},
		SomethingHappened {
			done: T::AssetIdParameter,
			user: T::AccountId,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_token(
			origin: OriginFor<T>,
			name: Vec<u8>,
			symbol: Vec<u8>,
			id: T::AssetIdParameter,
			decimals: u8,
			initial_supply: T::Balance,
		) -> DispatchResult {
			let owner = ensure_signed(origin.clone())?;

			let token_owner =
				<T::Lookup as sp_runtime::traits::StaticLookup>::unlookup(owner.clone());

			pallet_assets::Pallet::<T>::create(
				origin.clone(),
				id.clone(),
				token_owner.clone(),
				initial_supply,
			);

			pallet_assets::Pallet::<T>::set_metadata(origin, id.clone(), name, symbol, decimals);
			Self::deposit_event(Event::<T>::SomethingHappened {
				done: id.clone(),
				user: owner.clone(),
			});
			Ok(())
		}
	}
}
