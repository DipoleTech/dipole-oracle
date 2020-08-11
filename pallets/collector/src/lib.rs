#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]
#![allow(clippy::string_lit_as_bytes)]


use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, StorageValue, 
	dispatch::Vec,
	traits::Get,
};
use frame_system::ensure_signed;
use sp_std::{ 
	cmp::{Eq, PartialEq},
	prelude::*,
};
use utilities::{ 
	Did, OperatorRole, OperatorCategory, OperatorManager, GoodsOracleData,
	GoodsDataProvider,PayOracleData, PayDataProvider,CollectorData,CollectorManager,
};

type BalanceOf<T> = <<T as Trait>::PayDataProvider as PayDataProvider>::Balance;
type PayIdOf<T> = <<T as Trait>::PayDataProvider as PayDataProvider>::PayId;



pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	type Operator: OperatorManager<Did, Self::AccountId, OperatorRole, OperatorCategory>;
	type GoodsDataProvider: GoodsDataProvider<Self::AccountId>;
	type PayDataProvider: PayDataProvider;
}

decl_event!(
	pub enum Event<T> where
		<T as frame_system::Trait>::AccountId,
	{
		CollectOracleData(AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as Collector {
		
		pub CollectorDatas get(fn collect_data): Option<CollectorData<T::AccountId, PayIdOf<T>, BalanceOf<T>>>;

	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().reads_writes(4,1)]
		fn collect_oracle_data(
			origin,
		) {
			let who = ensure_signed(origin)?;

			Self::_collect_oracle_data();

			Self::deposit_event(RawEvent::CollectOracleData(who));
		}
	}
}

impl<T: Trait> Module<T> {

	pub fn _collect_oracle_data() -> Option<CollectorData<T::AccountId, PayIdOf<T>, BalanceOf<T>>>{
		let goods_owners = T::GoodsDataProvider::get_goods_owners_in_update_pool();
		let mut goods_oracle_data: Vec<GoodsOracleData<T::AccountId>>  = Vec::new();
		for i in  goods_owners{
			if let Some(new_goods_oracle_data) = T::GoodsDataProvider::get_goods_data(i.clone()){
				goods_oracle_data.push(new_goods_oracle_data); 	
			}
		}	

		let pays = T::PayDataProvider::get_pay_in_update_pool();
		let mut pay_oracle_data: Vec<PayOracleData<PayIdOf<T>, BalanceOf<T>>>  = Vec::new();
		for i in  pays{
			if let Some(balance) = T::PayDataProvider::get_pay_data(i.clone()){
				let new_pay_oracle_data = PayOracleData{
					pay_id: i,
					balance: balance,
				};
				pay_oracle_data.push(new_pay_oracle_data); 
			}
		}
		if goods_oracle_data.len()>0 || pay_oracle_data.len()>0{
			let new_collect_data = CollectorData {
				goods_oracle_data: goods_oracle_data,
				pay_oracle_data: pay_oracle_data,
			};
			<CollectorDatas<T>>::put(&new_collect_data);
			return Some(new_collect_data)
		}
		None
		

	}

}

impl<T: Trait> CollectorManager<T::AccountId, PayIdOf<T>, BalanceOf<T>> for Module<T> {
	fn collect_oracle_data() ->  Option<CollectorData<T::AccountId, PayIdOf<T>, BalanceOf<T>>>{
		Self::_collect_oracle_data()
	}
	fn get_collect_oracle_data() ->  Option<CollectorData<T::AccountId, PayIdOf<T>, BalanceOf<T>>>{
		Self::collect_data()
	}
}


