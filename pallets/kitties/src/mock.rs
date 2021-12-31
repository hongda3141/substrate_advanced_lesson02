use crate as pallet_kitties;
use sp_core::H256;
use frame_support::parameter_types;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header,
};
use frame_system as system;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
pub type Balance = u128;
// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		KittiesModule: pallet_kitties::{Module, Call, Storage, Event<T>},
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Module, Call, Storage},
        Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
   
}

impl system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 1;
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Test {
	type MaxLocks = MaxLocks;
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
}



parameter_types! {
	pub const ReserveValue: Balance = 1;
}
impl pallet_kitties::Config for Test {
	type Event = Event;
	type Randomness = RandomnessCollectiveFlip;
	type Currency = Balances;
	type KittyIndex = u64;
	type ReserveValue = ReserveValue;
}

// Build genesis storage according to the mock runtime.
// pub fn new_test_ext() -> sp_io::TestExternalities {
// 	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
// 	pallet_balances::GenesisConfig::<Test>{
// 		balances: vec![(0, 100), (1, 100), (2, 100)],
// 	}.assimilate_storage(&mut t).unwrap();

// 	t.into()
// }
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap(); //配置初始账户余额
  	pallet_balances::GenesisConfig::<Test>{
		balances: vec![(0, 100), (1, 100), (2, 100)],
	}.assimilate_storage(&mut t).unwrap();
   
    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1)); //设置起始区块的高度
    ext
}
// #[test]
// fn it_works_for_default_value() {
// 	new_test_ext().execute_with(|| {
// 		// Dispatch a signed extrinsic.
// 		assert_eq!(Balances::free_balance(0),100);
// 		// Read pallet storage and assert an expected result.
// 	});
// }