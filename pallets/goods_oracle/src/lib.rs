#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]
#![allow(clippy::string_lit_as_bytes)]


use frame_system::ensure_signed;
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, StorageMap, StorageValue, ensure, 
	traits::{Time, Get},
};
use sp_std::{prelude::*};
use sp_runtime::{DispatchResult};
use utilities::{
	Did, OperatorRole, OperatorCategory, OperatorManager, 
	RawVolume, OperatorVolume, TimestampedVolume, GoodsOperatorRawVolume, GoodsOperatorVolume,
	GoodsOracle, GoodsOracleData, GoodsDataProvider
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type MomentOf<T> = <<T as Trait>::Time as Time>::Moment;

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	type Time: Time;
	type Operator: OperatorManager<Did, Self::AccountId, OperatorRole, OperatorCategory> ;
}

decl_event!(
	pub enum Event<T> where
		<T as frame_system::Trait>::AccountId,
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
		pub GoodsDataRawValues get(fn goods_data_raw_values): map hasher(blake2_128_concat) Did => Vec<GoodsOperatorRawVolume<MomentOf<T>>>;

		// oracle data
		pub GoodsData get(fn goods_data): map hasher(twox_64_concat) Did => Option<GoodsOracle<Did, MomentOf<T>>>;
	
		// Goods uodate Pool 
		pub GoodsUpdatePool get(fn goods_in_update_pool):  Vec<Did>;
		pub GoodsOwnersUpdatePool get(fn goods_owners_in_update_pool):  Vec<T::AccountId>;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().reads_writes(4,3)]
		pub fn feed_goods_data(origin, feed_operator_id: Did, operator_volumes: Vec<OperatorVolume<Did>>) {
			let who = ensure_signed(origin)?;
			let operator = T::Operator::get_operator(feed_operator_id.clone()).ok_or(Error::<T>::UnknownOperator)?;
			ensure!(operator.is_legal, Error::<T>::NoPermission);
			Self::feed_data(feed_operator_id.clone(), operator_volumes)?;
			
			Self::deposit_event(RawEvent::NewFeedGoodsItemData(who, feed_operator_id));
		}
	}
}

impl<T: Trait> Module<T> {

	fn feed_data(feed_operator_id: Did, values: Vec<OperatorVolume<Did>>) -> DispatchResult {
		let operator = T::Operator::get_operator(feed_operator_id.clone()).ok_or(Error::<T>::UnknownOperator)?;
		ensure!(operator.is_legal, Error::<T>::NoPermission);
		let now = T::Time::now();
		for i in values {
			let operator_temp = T::Operator::get_operator(i.operator_id.clone()).ok_or(Error::<T>::UnknownOperator)?;
			let mut goods_data_exist = false;
			let timestamped_volumes = Self::goods_data_raw_values(i.operator_id.clone());
			if timestamped_volumes.len() >0{
				goods_data_exist = true;
			}
			
			
			let mut gorvs: Vec<GoodsOperatorRawVolume<MomentOf<T>>> = Vec::new();
			for j in i.operator_raw_volume{
				
				let timestamped = TimestampedVolume {
					volume: j.volume.clone(),
					timestamp: now
				};

				let gorv = GoodsOperatorRawVolume{
					volume_type:   j.volume_type.clone(),
					timestamed_volume: timestamped.clone()
				};
				gorvs.push(gorv);
			}
			
			
			<GoodsDataRawValues<T>>::insert(i.operator_id.clone(), gorvs.clone());
			Self::_add_goods_to_goods_update_pool(i.operator_id.clone());
			Self::_add_goods_owner_to_goods_owners_update_pool(operator_temp.owner.clone());

			if goods_data_exist == false{
				let mut govs: Vec<GoodsOperatorVolume<MomentOf<T>>> = Vec::new();
				for k in gorvs{
					let gov = GoodsOperatorVolume{
						volume_type: k.volume_type.clone(),
						init_volume: k.timestamed_volume.clone(),
						current_volume:  k.timestamed_volume.clone(),
					};
					govs.push(gov);
				}
				
				let new_goods_data = GoodsOracle {
					oracle_operator_id: i.operator_id.clone(),
					goods_operator_volume:  govs.clone(),
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
			public_consumer_volume: Vec::new(),
			public_producer_volume: Vec::new(),
			private_consumer_volume: Vec::new(),
			private_producer_volume: Vec::new(),
		};
		let operator_ids = T::Operator::get_owned_operators(key.clone());
		for i in operator_ids{
			let rvs = Self::_get_goods_data_update_value(i.clone());
			if rvs.len()>0{
				if let Some(goods_info) = T::Operator::get_operator(i){
					match goods_info.role {
						OperatorRole::PublicProducer => {
							new_goods_oracle_data.public_producer_volume = rvs;
						},
						OperatorRole::PublicConsumer => {
							new_goods_oracle_data.public_consumer_volume = rvs;
						},
						OperatorRole::PrivateProducer => {
							new_goods_oracle_data.private_producer_volume = rvs;
						},
						OperatorRole::PrivateConsumer => {
							new_goods_oracle_data.private_consumer_volume = rvs;
						},
						_=> {},
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

	fn _get_goods_data_update_value(key: Did) -> Vec<RawVolume> {
		Self::_remove_goods_from_goods_update_pool(key.clone());
		let mut rvs: Vec<RawVolume> = Vec::new();
		let gorvs = Self::goods_data_raw_values(key.clone());
		if gorvs.len() > 0{
			if let Some(mut goods_data) = Self::goods_data(key.clone()){
				for  i in gorvs{
					for j in 0..goods_data.goods_operator_volume.len(){
						if i.volume_type == goods_data.goods_operator_volume[j].volume_type{
							goods_data.goods_operator_volume[j].current_volume = i.timestamed_volume;
							let rv = RawVolume{
								volume_type: i.volume_type,
								volume: goods_data.goods_operator_volume[j].current_volume.volume - goods_data.goods_operator_volume[j].init_volume.volume,
							};
							rvs.push(rv)
						}
					}
				}
				<GoodsData<T>>::insert(key.clone(), goods_data.clone());
			}
		}
		rvs
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