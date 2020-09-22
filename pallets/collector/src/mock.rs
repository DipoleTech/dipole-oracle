// Creating mock runtime here
use super::*;
use crate::{Module, Trait};
use frame_support::{impl_outer_origin, parameter_types, traits::Time, weights::Weight};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	DispatchResult, Perbill,
};
use std::cell::RefCell;
use utilities::{Did, Operator, OperatorCategory, OperatorRole};

impl_outer_origin! {
	pub enum Origin for Test {}
}
pub type AccountId = u64;
pub type PayId = u64;
pub type Balance = u64;

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}
impl system::Trait for Test {
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type BaseCallFilter = ();
}

thread_local! {
	static TIME: RefCell<u32> = RefCell::new(0);
}

pub struct Timestamp;
impl Time for Timestamp {
	type Moment = u32;
	fn now() -> Self::Moment {
		TIME.with(|v| *v.borrow())
	}
}

pub struct MockOperator {
	pub owner: AccountId,
	pub role: OperatorRole,
	pub category: OperatorCategory,
	pub is_legal: bool,
}

impl OperatorManager<Did, AccountId, OperatorRole, OperatorCategory> for MockOperator {
	fn register_operator(
		_who: AccountId,
		_role: OperatorRole,
		_category: OperatorCategory,
	) -> DispatchResult {
		Ok(())
	}

	fn get_operator(_id: Did) -> Option<Operator<AccountId, OperatorRole, OperatorCategory>> {
		Some(Operator {
			owner: 2,
			role: OperatorRole::PrivateProducer,
			category: OperatorCategory::ElectricMeter,
			is_legal: true,
		})
	}

	fn get_owned_operators(_id: AccountId) -> Vec<Did> {
		Vec::new()
	}
}

pub struct MockGoodsOracle {}

impl GoodsDataProvider<AccountId> for MockGoodsOracle {
	fn get_goods_data(_id: AccountId) -> Option<GoodsOracleData<AccountId>> {
		Some(GoodsOracleData {
			owner: 1,
			public_consumer_volume: Vec::new(),
			public_producer_volume: Vec::new(),
			private_consumer_volume: Vec::new(),
			private_producer_volume: Vec::new(),
		})
	}
	fn get_goods_owners_in_update_pool() -> Vec<AccountId> {
		vec![1]
	}
}

pub struct MockPayOracle {}

impl PayDataProvider for MockPayOracle {
	type PayId = PayId;
	type Balance = Balance;
	fn get_pay_data(_id: PayId) -> Option<Balance> {
		Some(1)
	}
	fn get_pay_in_update_pool() -> Vec<PayId> {
		vec![1]
	}
}

impl Trait for Test {
	type Event = ();
	type Operator = MockOperator;
	type GoodsDataProvider = MockGoodsOracle;
	type PayDataProvider = MockPayOracle;
}

pub type CollectorModule = Module<Test>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap()
		.into()
}
