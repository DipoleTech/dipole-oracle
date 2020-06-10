#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]
#![allow(clippy::string_lit_as_bytes)]


use system::{self as system, ensure_signed};
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, StorageMap, StorageValue, ensure, 
	traits::{Time},
};
use sp_std::{prelude::*};
use sp_runtime::{DispatchResult};
use support::{Did, OperatorRole, OperatorCategory, OperatorManager, TimestampedVolume, OperatorVolume, GoodsOracle, GoodsOracleData, GoodsDataProvider};


type MomentOf<T> = <<T as Trait>::Time as Time>::Moment;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	type Time: Time;
	type Operator: OperatorManager<Did, Self::AccountId, OperatorRole, OperatorCategory> ;
}

decl_event!(
	pub enum Event<T> where
		<T as system::Trait>::AccountId,
	{
		/// New feed goods item data (sender, Did, values)
		NewFeedGoodsItemData(AccountId, Did),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		NoPermission,
		OperatorAlreadyRegistered,
		UnknownOperator,
		NotFound,
	}
}


decl_storage! {
	trait Store for Module<T: Trait> as GoodsOracle {
		// raw data
		pub GoodsDataRawValues get(fn goods_data_raw_values): map hasher(blake2_128_concat) Did => Option<TimestampedVolume<MomentOf<T>>>;

		// oracle data
		pub GoodsData get(fn goods_data): map hasher(twox_64_concat) Did => Option<GoodsOracle<Did, TimestampedVolume<MomentOf<T>>>>;
	
		// Goods uodate Pool 
		pub GoodsUpdatePool get(fn goods_in_update_pool):  Vec<Did>;
		pub GoodsOwnersUpdatePool get(fn goods_owners_in_update_pool):  Vec<T::AccountId>;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;

		#[weight = 10_000]
		pub fn feed_goods_data(origin, feed_operator_id: Did, operator_volumes: Vec<OperatorVolume<Did>>) {
			let who = ensure_signed(origin)?;
			Self::_feed_goods_data(feed_operator_id.clone(), operator_volumes)?;
			
			
			Self::deposit_event(RawEvent::NewFeedGoodsItemData(who, feed_operator_id));
		}
	}
}

impl<T: Trait> Module<T> {

	fn _feed_goods_data(feed_operator_id: Did, values: Vec<OperatorVolume<Did>>) -> DispatchResult {
		let operator = T::Operator::get_operator(feed_operator_id.clone()).ok_or(Error::<T>::UnknownOperator)?;
		ensure!(operator.is_legal, Error::<T>::NoPermission);
		let now = T::Time::now();
		for i in values {
			let operator_temp = T::Operator::get_operator(i.operator_id.clone()).ok_or(Error::<T>::UnknownOperator)?;
			let mut goods_data_exist = false;
			if let Some(_timestamped_volume) = Self::goods_data_raw_values(i.operator_id.clone()){
				goods_data_exist = true;
			}
			let timestamped = TimestampedVolume {
				volume: i.volume.clone(),
				timestamp: now
			};
			<GoodsDataRawValues<T>>::insert(i.operator_id.clone(), timestamped);
			Self::_add_goods_to_goods_update_pool(i.operator_id.clone());
			Self::_add_goods_owner_to_goods_owners_update_pool(operator_temp.owner.clone());

			if goods_data_exist == false{
				let new_goods_data = GoodsOracle {
					oracle_operator_id: i.operator_id.clone(),
					init_volume: timestamped.clone(),
					current_volume:  timestamped.clone(),
				};
				<GoodsData<T>>::insert(i.operator_id.clone(), new_goods_data);
			}
		}
		
		Ok(())
	}
	
}
impl<T: Trait> Module<T> {
	fn _get_goods_data_value(key: T::AccountId) -> Option<GoodsOracleData<T::AccountId>> {
		Self::_remove_goods_owner_from_goods_owners_update_pool(key.clone());
		let mut is_data_changed = false;
		let mut new_goods_oracle_data = GoodsOracleData{
			owner: key.clone(),
			consumer_volume: 0,
			producer_volume: 0,
		};
		let operator_ids = T::Operator::get_owned_operators(key.clone());
		for i in operator_ids{
			if let Some(volume) = Self::_get_goods_data_update_value(i.clone()){
				if let Some(goods_info) = T::Operator::get_operator(i){
					if goods_info.role == OperatorRole::Producer{
						new_goods_oracle_data.producer_volume += volume;
					}else{
						new_goods_oracle_data.consumer_volume += volume;
					}
					is_data_changed = true;
				}
			}
		}
		if is_data_changed == true{
			return Some(new_goods_oracle_data)
		}
		None
	}

	fn _get_goods_data_update_value(key: Did) -> Option<u64> {
		Self::_remove_goods_from_goods_update_pool(key.clone());
		if let Some(timestamped_volume) = Self::goods_data_raw_values(key.clone()){
			if let Some(mut goods_data) = Self::goods_data(key.clone()){
				goods_data.current_volume = timestamped_volume;
				let volume = goods_data.current_volume.volume - goods_data.init_volume.volume;
				if volume != 0{
					<GoodsData<T>>::insert(key.clone(), goods_data.clone());
					return Some(volume)
				}
			}
		}
		None
	}
}
impl<T: Trait> Module<T> {
	// goods update pool
	fn _add_goods_to_goods_update_pool(id: Did) {
		if !Self::goods_in_update_pool().contains(&id){
			let _err = <GoodsUpdatePool>::append(&id);
		}
	}

	fn _remove_goods_from_goods_update_pool(id: Did){
		if Self::goods_in_update_pool().contains(&id){
			// keep all goods except for the given goods
			<GoodsUpdatePool>::mutate(|goods| goods.retain(|g| g != &id));
		}
	}

	// goods owners update pool
	fn _add_goods_owner_to_goods_owners_update_pool(id: T::AccountId) {
		if !Self::goods_owners_in_update_pool().contains(&id){
			let _err = <GoodsOwnersUpdatePool<T>>::append(&id);
		}
	}

	fn _remove_goods_owner_from_goods_owners_update_pool(owner: T::AccountId){
		if Self::goods_owners_in_update_pool().contains(&owner){
			// keep all goods owner except for the given goods owner
			<GoodsOwnersUpdatePool<T>>::mutate(|goods| goods.retain(|g| g != &owner));
		}
	}
}


impl<T: Trait> GoodsDataProvider<T::AccountId> for Module<T> {
	fn get_goods_data(id: T::AccountId) -> Option<GoodsOracleData<T::AccountId>> {
		Self::_get_goods_data_value(id)
	}
	fn get_goods_owners_in_update_pool() -> Vec<T::AccountId>{
		Self::goods_owners_in_update_pool()
	}
}