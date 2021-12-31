#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*,traits::{Randomness,Currency,ReservableCurrency,
                        ExistenceRequirement::{KeepAlive}}};
	use frame_system::pallet_prelude::*;
    use sp_io::hashing::blake2_128;
    use sp_runtime::{
        traits::{
            One,Zero, AtLeast32BitUnsigned,
            MaybeSerializeDeserialize, Bounded,
        },
    };
    use codec::{Codec, Encode, Decode};
    use sp_std::{fmt::Debug};



    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[derive(Encode,Decode)]
    pub struct Kitty(pub [u8;16]);



	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Randomness: Randomness<Self::Hash>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>; //用于balance交易
        type KittyIndex: Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy +
                         MaybeSerializeDeserialize + Debug; //自定义类型
        type ReserveValue: Get<BalanceOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn kitties_count)]
    pub type KittiesCount<T:Config> = StorageValue<_,T::KittyIndex>;

    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub type Kitties<T: Config> = StorageMap<_,Blake2_128Concat,T::KittyIndex,Option<Kitty>,ValueQuery>;


    #[pallet::storage]
    #[pallet::getter(fn owner)]
    pub type Owner<T: Config> = StorageMap<_,Blake2_128Concat,T::KittyIndex,T::AccountId,ValueQuery>;


    //存放等待交易的kitties
    #[pallet::storage]
    #[pallet::getter(fn kitties_tx_pool)]
    pub type KittiesTxPool<T: Config> = StorageMap<_,Blake2_128Concat,T::KittyIndex,BalanceOf<T>,ValueQuery>;


	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		CreateKitty(T::AccountId, T::KittyIndex),
        TransferKitty(T::AccountId,T::AccountId,T::KittyIndex),
        BreedKitty(T::AccountId,T::KittyIndex),
        BuyKittes(T::AccountId,T::KittyIndex),
        SellKitties(T::AccountId,T::KittyIndex),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
        KittiesCountOverflow,
        NotOwner,
        InvalidKittyIndex,
        SameParentIndex,
        NotsufficientValue,
        NotKittyID,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn create_kitty(origin: OriginFor<T>) -> DispatchResultWithPostInfo{
            let who = ensure_signed(origin)?;
            T::Currency::reserve(&who,T::ReserveValue::get()).map_err(|_| Error::<T>::NotsufficientValue)?; //创建kittie质押资金
          
            let kitty_id = match Self::kitties_count(){
                Some(id) => {
                    ensure!(id != T::KittyIndex::max_value(),Error::<T>::KittiesCountOverflow);
                    id
                },
                None => {
                    Zero::zero()
                }
            };

            let dna =Self::random_value(&who);
    
            Kitties::<T>::insert(kitty_id,Some(Kitty(dna)));
            Owner::<T>::insert(kitty_id,who.clone());
            KittiesCount::<T>::put(kitty_id + One::one());

            Self::deposit_event(Event::CreateKitty(who,kitty_id));
            Ok(().into())
        }

   
        #[pallet::weight(0)]
        pub fn transfer(origin: OriginFor<T>,new_owner: T::AccountId,kitty_id :T::KittyIndex) -> DispatchResultWithPostInfo{
            let  sender = ensure_signed(origin)?;
            ensure!(Owner::<T>::get(kitty_id) ==sender.clone(),Error::<T>::NotOwner); //确保调用者是拥有者
            T::Currency::reserve(&new_owner,T::ReserveValue::get()).map_err(|_| Error::<T>::NotsufficientValue)?; //新的拥有者需要质押金额
            T::Currency::unreserve(&sender, T::ReserveValue::get());//原本的拥有者退回质押金
            Owner::<T>::insert(kitty_id,new_owner.clone());
            Self::deposit_event(Event::TransferKitty(sender,new_owner,kitty_id));
            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn breed_kitty(origin: OriginFor<T>,kitty1_id: T::KittyIndex,kitty2_id: T::KittyIndex) -> DispatchResultWithPostInfo{
            let who = ensure_signed(origin)?;

            ensure!(kitty1_id != kitty2_id,Error::<T>::SameParentIndex);

            let kitty1 = Self::kitties(kitty1_id).ok_or(Error::<T>::InvalidKittyIndex)?;
            let kitty2 = Self::kitties(kitty2_id).ok_or(Error::<T>::InvalidKittyIndex)?;

            let kitty_id = match Self::kitties_count(){
                Some(id) => {
                    ensure!(id != T::KittyIndex::max_value(),Error::<T>::KittiesCountOverflow);
                    id
                },
                None => {
                    One::one()
                }
            };
            let dna_1 = kitty1.0;
            let dna_2 = kitty2.0;

            let selector = Self::random_value(&who);
            let mut new_dna = [0u8;16];
            
            for i in 0..dna_1.len(){
                new_dna[i] = (selector[i] & dna_1[i]) | (!selector[i] & dna_2[i]);
            }
        
            Kitties::<T>::insert(kitty_id,Some(Kitty(new_dna)));
            Owner::<T>::insert(kitty_id,who.clone());
            KittiesCount::<T>::put(kitty_id+One::one());

            Self::deposit_event(Event::BreedKitty(who,kitty_id));
            Ok(().into())
        }


        #[pallet::weight(0)]
        pub fn sell_kitties(origin: OriginFor<T>, kitty_id: T::KittyIndex,value: BalanceOf<T>) -> DispatchResultWithPostInfo{
            let who =ensure_signed(origin)?;

            ensure!(Owner::<T>::get(kitty_id) == who.clone(),Error::<T>::NotOwner);
            KittiesTxPool::<T>::insert(kitty_id,value); //将拥有的kittie和对应的价格放入交易池中
            Self::deposit_event(Event::SellKitties(who,kitty_id));
            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn buy_kitties(origin: OriginFor<T>,kitty_id: T::KittyIndex) -> DispatchResultWithPostInfo{
            let who = ensure_signed(origin)?;
            let value = Self::kitties_tx_pool(kitty_id); //获取kittie的价格
            let owne = Owner::<T>::get(kitty_id);

            ensure!(value != Zero::zero(),Error::<T>::NotKittyID);//出售价格为0表示没有改id的kittie,可以换成Option类型用None去判断
            ensure!(T::Currency::free_balance(&who) > value,Error::<T>::NotsufficientValue);//确保购买者有足够的今额

            T::Currency::transfer(&who,&owne,value,KeepAlive).map_err(|_| Error::<T>::NotsufficientValue)?; //购买
            T::Currency::unreserve(&owne, T::ReserveValue::get()); //退回给原拥有者资金

            KittiesTxPool::<T>::remove(kitty_id); //踢出交易池
            Owner::<T>::insert(kitty_id,&who,);
      
            Self::deposit_event(Event::BuyKittes(who,kitty_id));
            Ok(().into())
        }

	}

    impl<T: Config>Pallet<T>{
        fn random_value(sender: &T::AccountId) -> [u8;16]{
            let payload = (
                T::Randomness::random_seed(),
                &sender,
                <frame_system::Pallet<T>>::extrinsic_index(),
            );
            payload.using_encoded(blake2_128)
        }
    }
}
