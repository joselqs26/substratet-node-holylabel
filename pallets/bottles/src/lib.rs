#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    use frame_support::dispatch::Vec;
    use frame_support::sp_runtime::Permill;
    // use std::time::{ SystemTime, UNIX_EPOCH };

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    // Estructura de datos para representar una botella
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct BottleTransaction<T: Config> {
        owner: T::AccountId,
        // transfer_date: u64, // Almacena la marca de tiempo como un u64
        latitude: Permill,          // Posición X (latitud)
        longitude: Permill,         // Posición Y (longitud)
    }

    #[pallet::storage]
    pub(super) type TransactionsCounts<T: Config> =
        StorageMap<
            _,
            Twox64Concat,
            [u8; 16],
            u32,
        >;

    #[pallet::storage]
    pub(super) type BottleTransactions<T: Config> =
        StorageDoubleMap<
            _,
            Twox64Concat,
            [u8; 16],
            Twox64Concat,
            u32,
            BottleTransaction<T>
        >;

    #[pallet::error]
    pub enum Error<T> {
        BottleNotFound,
        BottleAlreadyExists,
        InvalidOwner,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        BottleTransferred(
            [u8; 16], 
            T::AccountId, 
            // u64, 
            Permill, 
            Permill
        )
    }

    // Implementación de funciones de la paleta
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        pub fn mint(
			origin: OriginFor<T>,
			id: [u8; 16],
			latitude: Permill,
			longitude: Permill,
        ) -> DispatchResult {
			let sender = ensure_signed(origin)?;
		    
            // let current_system_time = SystemTime::now();
            // let duration_since_epoch = current_system_time.duration_since(UNIX_EPOCH);
            // let milliseconds_timestamp = match duration_since_epoch {
            //     Ok(duration) => duration.as_millis() as u64,
            //     Err(err) => {
            //         0 
            //     }
            // };

            let transaction = BottleTransaction {
                owner: sender.clone(),
                // transfer_date: milliseconds_timestamp,
                latitude,
                longitude,
            };

            if TransactionsCounts::<T>::contains_key(&id) {
                let count_option = TransactionsCounts::<T>::get(&id);

                let count = match count_option {
                    Some(count_option) => count_option as u32,
                    None => 0 as u32, // Valor por defecto si el Option es None
                };
                
                BottleTransactions::<T>::insert(&id, count, transaction);
                TransactionsCounts::<T>::mutate(&id, |value| {
                    match value {
                        Some(value) => *value + 1,
                        None => 0, // Valor por defecto si el Option es None
                    };
                });
            } else {
                BottleTransactions::<T>::insert(&id, 0 as u32, transaction);
                TransactionsCounts::<T>::set(&id, Some(1 as u32));
            }
        
			Self::deposit_event(Event::BottleTransferred(
                id, 
                sender.clone(), 
                // milliseconds_timestamp, 
                latitude, 
                longitude
            ));

            Ok(())
		}

		#[pallet::weight(0)]
        pub fn add_bottle_transaction(
			origin: OriginFor<T>,
			id: [u8; 16],
			latitude: Permill,
			longitude: Permill,
		) -> DispatchResult {
            let _ = Self::mint(origin, id, latitude, longitude);
			Ok(())
		}
    }
}
