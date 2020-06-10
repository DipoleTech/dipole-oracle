#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]
#![allow(clippy::string_lit_as_bytes)]

use system::{self as system, ensure_signed};
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, StorageMap, StorageValue, ensure, Parameter,
	traits::{Time},
	dispatch::Vec,
};
use sp_runtime::{
	traits::{AtLeast32Bit, MaybeSerializeDeserialize, Member},
	DispatchResult,
};
use sp_std::{
	cmp::{Eq, PartialEq},
	prelude::*,
};
use support::{Did, OperatorRole, OperatorCategory, OperatorManager, TimestampedPrice, OperatorPrice, PayOracle, PayDataProvider};


type MomentOf<T> = <<T as Trait>::Time as Time>::Moment;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type PayId: Parameter + Member + AtLeast32Bit + Default + Copy + MaybeSerializeDeserialize;
	type Balance: Parameter + Member + AtLeast32Bit + Default + Copy + MaybeSerializeDeserialize;

	type Time: Time;
	type Operator: OperatorManager<Did, Self::AccountId, OperatorRole, OperatorCategory>;
}

decl_event!(
	pub enum Event<T> where
		<T as system::Trait>::AccountId,
	{	
		/// New feed pay data (sender, Did, OperatorPrice)
		NewFeedPayData(AccountId, Did),
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
	trait Store for Module<T: Trait> as PayData {
		// raw data
		pub PayDataRawValues get(fn pay_data_raw_values): map hasher(blake2_128_concat) T::PayId => Option<TimestampedPrice<T::Balance, MomentOf<T>>>;

		// oracle data
		pub PayData get(fn pay_data): map hasher(twox_64_concat) T::PayId => Option<PayOracle<T::PayId, TimestampedPrice<T::Balance, MomentOf<T>>>>;

		// Pay uodate Pool 
		pub PaysUpdatePool get(fn pays_in_update_pool):  Vec<T::PayId>;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;
		
		#[weight = 10_000]
		pub fn feed_pay_data(origin, operator_id: Did, pay_prices: Vec<OperatorPrice<T::PayId, T::Balance>>) {
			let who = ensure_signed(origin)?;
			Self::_feed_pay_data(operator_id.clone(), pay_prices)?;
			Self::deposit_event(RawEvent::NewFeedPayData(who, operator_id));
		}
	}
}

impl<T: Trait> Module<T> {

	fn _feed_pay_data(operator_id: Did, pay_prices: Vec<OperatorPrice<T::PayId, T::Balance>>) -> DispatchResult {
		let operator = T::Operator::get_operator(operator_id.clone()).ok_or(Error::<T>::UnknownOperator)?;
		ensure!(operator.is_legal, Error::<T>::NoPermission);

		let now = T::Time::now();

		for i in pay_prices {
			let timestamped = TimestampedPrice {
				price: i.price.clone(),
				timestamp: now
			};
			<PayDataRawValues<T>>::insert(i.pay_id.clone(), timestamped);
			Self::_add_pay_to_pays_update_pool(i.pay_id.clone());
		}
		
		Ok(())
	}
}
impl<T: Trait> Module<T> {

	fn _get_pay_data_value(key: T::PayId) -> Option<TimestampedPrice<T::Balance, MomentOf<T>>> {
		Self::_remove_pay_from_pays_update_pool(key.clone());
		if let Some(timestamped_balance) = Self::pay_data_raw_values(key.clone()){
			if let Some(mut pay_data) = Self::pay_data(key.clone()){
				pay_data.pay_price = timestamped_balance.clone();
				<PayData<T>>::insert(key.clone(), pay_data);
			}else{
				let new_pay_data = PayOracle{
					pay_id: key.clone(),
					pay_price: timestamped_balance.clone(),
				};
				<PayData<T>>::insert(key.clone(), new_pay_data);
			}
			return Some(timestamped_balance)
		}
		None
		
	}
}
impl<T: Trait> Module<T> {

	// pays update pool
	fn _add_pay_to_pays_update_pool(id: T::PayId) {
		if !Self::pays_in_update_pool().contains(&id){
			let _err = <PaysUpdatePool<T>>::append(&id);
		}
	}

	fn _remove_pay_from_pays_update_pool(id: T::PayId){
		if Self::pays_in_update_pool().contains(&id){
			// keep all pays except for the given pay 
			<PaysUpdatePool<T>>::mutate(|pays| pays.retain(|p| p != &id));
		}
	}

}

impl<T: Trait> PayDataProvider for Module<T> {
	type PayId = T::PayId;
	type Balance = T::Balance;

	fn get_pay_data(id: Self::PayId) -> Option<Self::Balance> {
		Self::_get_pay_data_value(id).map(|timestamped_price| timestamped_price.price)
	}
	fn get_pay_in_update_pool() -> Vec<Self::PayId>{
		Self::pays_in_update_pool()
	}
}
