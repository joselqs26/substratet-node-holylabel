#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		#[pallet::constant]
		type MaximumOwned: Get<u32>;
	}

	pub enum AccountRol {
		Factory,
		Distributor,
		Decanter,
		Consumer,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct AccountType<T: Config> {
		pub rol_id: [u8; 1],
		pub owner: T::AccountId,
	}

	#[pallet::storage]
	pub(super) type OwnerOfAccountType<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		BoundedVec<[u8; 1], T::MaximumOwned>,
		ValueQuery
	>;

	#[pallet::error]
	pub enum Error<T> {
		MaximumCollectiblesOwned,
		RolNotFound,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new collectible was successfully created
		AccountCreated {
			owner: T::AccountId,
			id_rol: [u8; 1],
		},
	}

	// Function to mint a collectible
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn mint(origin: OriginFor<T>, id_rol: [u8; 1]) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;

			let u8_value: u8 = id_rol[0];
			let index: usize = u8_value as usize;
			let account_rol: Result<AccountRol, Error<T>> = match index {
				0 => Ok(AccountRol::Factory),
				1 => Ok(AccountRol::Distributor),
				2 => Ok(AccountRol::Decanter),
				3 => Ok(AccountRol::Consumer),
				_ => Err(Error::<T>::RolNotFound),
			};

			if let Ok(account_rol) = account_rol {
				// El valor se obtuvo correctamente, puedes continuar con él
			} else {
				// Manejar el error aquí si ocurre
				// Puedes imprimir un mensaje de error o realizar otra acción apropiada
				return Err(Error::<T>::RolNotFound.into());
			}

			let account_type = AccountType::<T> { rol_id: id_rol, owner: owner.clone() };

			OwnerOfAccountType::<T>
				::try_append(owner.clone(), account_type.rol_id)
				.map_err(|_| Error::<T>::MaximumCollectiblesOwned)?;

			Self::deposit_event(Event::AccountCreated {
				owner: owner.clone(),
				id_rol,
			});

			// Returns the unique_id of the new collectible if this succeeds
			Ok(Pays::No.into())
		}

		#[pallet::weight(0)]
		pub fn link_account_rol(origin: OriginFor<T>, id_rol: [u8; 1]) -> DispatchResult {
			// Write new collectible to storage by calling helper function

			let _ = Self::mint(origin, id_rol);

			Ok(())
		}
	}
}
