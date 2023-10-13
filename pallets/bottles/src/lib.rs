#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
// pub use std::time::{ SystemTime, UNIX_EPOCH };

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
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
        latitude: Option<i64>,          // Posición X (latitud)
        longitude: Option<i64>,         // Posición Y (longitud)
    }

    #[pallet::storage]
    #[pallet::getter(fn get_bottle_transactions)]
    pub(super) type BottleTransactions<T: Config> =
        StorageMap<
            _,
            Twox64Concat,
            [u8; 16],
            Vec<BottleTransaction<T>>,
            ValueQuery
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
            Option<i64>, 
            Option<i64>
        )
    }

    // Implementación de funciones de la paleta
    #[pallet::call]
    impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
        pub fn add_bottle_transaction(
			origin: OriginFor<T>,
			id: [u8; 16],
			latitude: Option<i64>,
			longitude: Option<i64>,
		) -> DispatchResultWithPostInfo {
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

            if BottleTransactions::<T>::contains_key(&id) {
                let mut transactions = BottleTransactions::<T>::get(&id);
                transactions.push(transaction);
                // Almacena la lista actualizada de transacciones
                BottleTransactions::<T>::set(&id, transactions);
            } else {
                let mut transactions = Vec::new();
                transactions.push(transaction);
                
                BottleTransactions::<T>::insert(&id, transactions);
            }

        
			Self::deposit_event(Event::BottleTransferred(
                id, 
                sender.clone(), 
                // milliseconds_timestamp, 
                latitude, 
                longitude
            ));
		
			Ok(Pays::No.into())
		}	
    }
}
