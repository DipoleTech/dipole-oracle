#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]
#![allow(clippy::string_lit_as_bytes)]


use frame_system::ensure_signed;
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, StorageMap, StorageValue, 
	traits::{Randomness, Get},
};
use sp_std::{prelude::*};
use sp_runtime::{DispatchResult};
use sp_io::hashing::blake2_256;
use codec::{Encode};
use randomness;
use utilities::{Did, OperatorRole, OperatorCategory, Operator, OperatorManager};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_event!(
	pub enum Event<T> where
		<T as frame_system::Trait>::AccountId,
	{
		// A new operator has been registered
		OperatorRegistered(AccountId),

		// An existing operator has been unregistered
		OperatorUnregistered(AccountId),
		
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
	trait Store for Module<T: Trait> as Operator {

		// operator
		pub Operators get(fn operator): map hasher(twox_64_concat) Did => Option<Operator<T::AccountId, OperatorRole, OperatorCategory>>;
		pub OperatorsCount get(fn operators_count): u64;
		pub OperatorsIndex get(fn operators_index): map hasher(blake2_128_concat) u64 => Did;

		// Nonce
		pub Nonce get(fn nonce): u64;

		// all goods operators For fast operator checks
		AllGoodsOperators get(fn all_goods_operators): Vec<T::AccountId>;
		// all pay operators For fast operator checks
		AllPayOperators get(fn all_pay_operators): Vec<T::AccountId>;
		// owned operators
		OwnedOperators get(fn owned_operators_in_pool):  map hasher(twox_64_concat) T::AccountId => Vec<Did>;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;

		// Register a new Operator.
		#[weight = 10_000 + T::DbWeight::get().reads_writes(3,6)]
		pub fn register_operator(
			origin,
			role: OperatorRole,
			category: OperatorCategory,
		) {
			let who = ensure_signed(origin)?;

			Self::_register_operator(who.clone(), role, category)?;

			Self::deposit_event(RawEvent::OperatorRegistered(who));

		}

		// close an existing Operator
		#[weight = 10_000 + T::DbWeight::get().reads_writes(3,2)]
		pub fn close_operator(
			origin,
			operator_id: Did,
		){
			let who = ensure_signed(origin)?;

			Self::_close_operator(who.clone(), operator_id.clone())?;
			
			Self::deposit_event(RawEvent::OperatorUnregistered(who));	
		}
	}
}

impl<T: Trait> Module<T> {

	fn _register_operator(who: T::AccountId, role: OperatorRole, category: OperatorCategory) -> DispatchResult {
	
		let nonce = Self::get_nonce();
       	let random_seed = <randomness::Module<T>>::random_seed();
        let encoded = (random_seed, who.clone(), nonce).encode();
        let did = blake2_256(&encoded);
		let new_operator_id = Did { did };
		
		Self::new_operator(who.clone(), new_operator_id.clone(), role.clone(), category.clone())?;
		<OperatorsCount>::put(nonce.clone()+1);
		<OperatorsIndex>::insert(nonce.clone(), new_operator_id.clone());

		Ok(())
	}

	fn new_operator(who: T::AccountId, id: Did, role: OperatorRole, category: OperatorCategory) -> DispatchResult {
	
		let new_operator = Operator {
			owner: who.clone(),
			role: role.clone(),
			category: category.clone(),
			is_legal: true,
		};

		<Operators<T>>::insert(id.clone(), &new_operator);

		Self::_add_operator_to_owned_operators_pool(who.clone(), id.clone());

		if role == OperatorRole::Payer{
			if !Self::all_pay_operators().contains(&who){
				<AllPayOperators<T>>::append(&who);
			}
		}else{
			if !Self::all_goods_operators().contains(&who){
				<AllGoodsOperators<T>>::append(&who);
			}
		}
		Ok(())
	}

	fn update_operator(id: Did, new_operator: Operator<T::AccountId, OperatorRole, OperatorCategory>) -> DispatchResult {
		Self::operator(id.clone()).ok_or(Error::<T>::UnknownOperator)?;
		<Operators<T>>::insert(id, &new_operator);
		Ok(())
	}

	fn _close_operator(who: T::AccountId, operator_id: Did) -> DispatchResult {
		let mut operator = Self::operator(operator_id.clone()).ok_or(Error::<T>::UnknownOperator)?;
		operator.is_legal = false;
		Self::_remove_operator_from_owned_operators_pool(who.clone(), operator_id.clone());
		// <Operators<T>>::insert(operator_id, &operator);
		Self::update_operator(operator_id.clone(), operator.clone())?;
		Ok(())
	}
}

impl<T: Trait> Module<T> {
	// owned operators pool
	fn _add_operator_to_owned_operators_pool(owner: T::AccountId, id: Did) {
		if !Self::owned_operators_in_pool(owner.clone()).contains(&id){
			let mut owned_operators = Self::owned_operators_in_pool(owner.clone());
			owned_operators.push(id.clone());
			<OwnedOperators<T>>::insert(owner, owned_operators);
		}
	}

	fn _remove_operator_from_owned_operators_pool(owner: T::AccountId, id: Did){
		if Self::owned_operators_in_pool(owner.clone()).contains(&id){
			let mut owned_operators = Self::owned_operators_in_pool(owner.clone());
			let mut j = 0;
			for i in &owned_operators{
				if *i == id {
					owned_operators.remove(j);
					break;
				}	
				j +=1; 
			}	
			<OwnedOperators<T>>::insert(owner, owned_operators);
		}
	}

	// nonce
	fn get_nonce() -> u64 {
        let nonce = <Nonce>::get();
        <Nonce>::mutate(|n| *n += 1u64);
        nonce
	}
}

impl<T: Trait> OperatorManager<Did, T::AccountId, OperatorRole, OperatorCategory> for Module<T> {
	fn register_operator(who: T::AccountId, role: OperatorRole, category: OperatorCategory) -> DispatchResult{
		Self::_register_operator(who, role, category)
	}

	fn get_operator(id: Did) -> Option<Operator<T::AccountId, OperatorRole, OperatorCategory>>{
		Self::operator(id)
	}

	fn get_owned_operators(id: T::AccountId) -> Vec<Did>{
		Self::owned_operators_in_pool(id)
	}
}
