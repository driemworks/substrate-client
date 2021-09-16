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
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	/// map a nickname to an account
	pub type Nicks<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn storage_quad_map)]
	/// num, accountid, english word, target language -> target language translation
	pub type StorageQuadMap<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, Vec<u8>>,
			NMapKey<Blake2_128Concat, Vec<u8>>,
		),
		Vec<u8>,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored(Vec<u8>, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_entry_to_directory(
			origin: OriginFor<T>,
		    name: Vec<u8>)
		-> DispatchResult {
			let who = ensure_signed(origin)?;
			<Nicks<T>>::insert(who.clone(), name.clone());
			Self::deposit_event(Event::SomethingStored(name.clone(), who.clone()));
			Ok(())
		}
		/// creates an entry in the map to map an english word or number into a translation in a given target languge 
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_entry(
			origin: OriginFor<T>,
		    some_number: u32,
		    english_name: Vec<u8>,
		    target_language: Vec<u8>,
		    target_language_name: Vec<u8>)
		-> DispatchResult {
			let who = ensure_signed(origin)?;
			// insert all keys and the value
			// , some_number, who.clone()
			StorageQuadMap::<T>::insert(
				(english_name, target_language),
				target_language_name.clone(),
			);
			Self::deposit_event(Event::SomethingStored(target_language_name.clone(), who.clone()));
			Ok(())
		}
	}
}
