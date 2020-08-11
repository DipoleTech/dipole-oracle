#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]
#![allow(clippy::string_lit_as_bytes)]


use frame_support::dispatch::Vec;
use sp_std::{
	cmp::{Eq, PartialEq},
	fmt::Debug,
	prelude::*,
};

use sp_runtime::{
	traits::{AtLeast32Bit, MaybeSerializeDeserialize},
	RuntimeDebug, DispatchResult
};
use codec::{Encode, Decode, FullCodec};


// operator
#[derive(Encode, Decode, Default, PartialOrd, Ord, PartialEq, Eq, Clone, RuntimeDebug)]
pub struct Did {
    pub did: [u8; 32],
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum OperatorRole {
	PublicProducer = 0,
	PublicConsumer,
	PrivateProducer,
	PrivateConsumer,
	Payer,
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum OperatorCategory {
	ElectricMeter = 0,
	ChargingPoint,
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum VolumeType {
	Peak = 0,
	Flat,
	Valley,
}

#[derive(Encode, Decode, RuntimeDebug, Eq, PartialEq, Clone, Copy)]
pub struct Operator<AccountId, OperatorRole, OperatorCategory> {
	pub owner: AccountId,
	pub role: OperatorRole,
	pub category: OperatorCategory,
	pub is_legal: bool,
}

pub trait OperatorManager<Did, AccountId, OperatorRole, OperatorCategory> {
	fn register_operator(who: AccountId, role: OperatorRole, category: OperatorCategory) -> DispatchResult;
	fn get_operator(id: Did) -> Option<Operator<AccountId, OperatorRole, OperatorCategory>>;
	fn get_owned_operators(id: AccountId) -> Vec<Did>;
}

// goods data
#[derive(Encode, Decode, RuntimeDebug, Eq, PartialEq, Clone, Copy)]
pub struct RawVolume {
	pub volume_type: VolumeType,
	pub volume: u64,
}

#[derive(Encode, Decode, RuntimeDebug, Eq, PartialEq, Clone)]
pub struct OperatorVolume<Did> {
	pub operator_id: Did,
	pub operator_raw_volume: Vec<RawVolume>,
}


#[derive(Encode, Decode, RuntimeDebug, Eq, PartialEq, Clone, Copy)]
pub struct TimestampedVolume<Moment> {
	pub volume: u64,
	pub timestamp: Moment,
}

#[derive(Encode, Decode, RuntimeDebug, Eq, PartialEq, Clone, Copy)]
pub struct GoodsOperatorRawVolume<Moment> {
	pub volume_type: VolumeType,
	pub timestamed_volume: TimestampedVolume<Moment>,
}

#[derive(Encode, Decode, RuntimeDebug, Eq, PartialEq, Clone, Copy)]
pub struct GoodsOperatorVolume<Moment> {
	pub volume_type: VolumeType,
	pub init_volume: TimestampedVolume<Moment> ,
	pub current_volume: TimestampedVolume<Moment>,
}

#[derive(Encode, Decode, RuntimeDebug, Eq, PartialEq, Clone)]
pub struct GoodsOracle<Did, Moment>{
	pub oracle_operator_id: Did,
	pub goods_operator_volume: Vec<GoodsOperatorVolume<Moment>>,
}

#[derive(Encode, Decode, RuntimeDebug, Eq, PartialEq, Clone)]
pub struct GoodsOracleData<AccountId>{
	pub owner: AccountId,
	pub public_consumer_volume: Vec<RawVolume>,
	pub public_producer_volume: Vec<RawVolume>,
	pub private_consumer_volume: Vec<RawVolume>,
	pub private_producer_volume: Vec<RawVolume>,
}

pub trait GoodsDataProvider<AccountId> {
	fn get_goods_data(id: AccountId) -> Option<GoodsOracleData<AccountId>>;
	fn get_goods_owners_in_update_pool() -> Vec<AccountId>;
}


// pay data
#[derive(Encode, Decode, RuntimeDebug, Eq, PartialEq, Clone, Copy)]
pub struct TimestampedPrice<Balance, Moment> {
	pub price: Balance,
	pub timestamp: Moment,
}

#[derive(Encode, Decode, RuntimeDebug, Eq, PartialEq, Clone, Copy)]
pub struct OperatorPrice<PayId, Balance> {
	pub pay_id: PayId,
	pub price: Balance,
}

#[derive(Encode, Decode, RuntimeDebug, Eq, PartialEq, Clone, Copy)]
pub struct PayOracle<PayId, TimestampedBalance>{
	pub pay_id: PayId,
	pub pay_price: TimestampedBalance,
}

#[derive(Encode, Decode, RuntimeDebug, Eq, PartialEq, Clone, Copy)]
pub struct PayOracleData<PayId, Balance>{
	pub pay_id: PayId,
	pub balance: Balance,
}

pub trait PayDataProvider {
	type PayId: FullCodec + Default + Copy + Eq + PartialEq + MaybeSerializeDeserialize + Debug;
	type Balance: AtLeast32Bit + FullCodec + Copy + MaybeSerializeDeserialize + Debug + Default;

	fn get_pay_data(id: Self::PayId) -> Option<Self::Balance>;
	fn get_pay_in_update_pool() -> Vec<Self::PayId>;
}


// colleter
#[derive(Encode, Decode, RuntimeDebug, Eq, PartialEq, Clone)]
pub struct CollectorData<AccountId, PayId, Balance>{
	pub goods_oracle_data: Vec<GoodsOracleData<AccountId>>,
	pub pay_oracle_data: Vec<PayOracleData<PayId, Balance>>,
}

pub trait CollectorManager<AccountId, PayId, Balance>{

	fn collect_oracle_data()->  Option<CollectorData<AccountId, PayId, Balance>>;
	fn get_collect_oracle_data() ->  Option<CollectorData<AccountId, PayId, Balance>>;
}

