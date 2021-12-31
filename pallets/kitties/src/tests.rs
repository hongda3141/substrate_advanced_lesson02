use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};


// use sp_runtime::{
//     RuntimeDebug, DispatchResult, DispatchError,
//     traits::{
//         One,Zero, AtLeast32BitUnsigned, StaticLookup, CheckedAdd, CheckedSub,
//         MaybeSerializeDeserialize, Saturating, Bounded, StoredMapError,
//     },
// };
// use sp_io::hashing::blake2_128;
use super::*;



#[test]
fn create_kitties() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(KittiesModule::create_kitty(Origin::signed(0))); //AccoutID=0创建kittie,质押 1 balance
        assert_eq!(Balances::free_balance(0),99);
        assert_eq!(Owner::<Test>::get(0),0);//判断0id的拥有者是不是0 account
        assert_eq!(KittiesCount::<Test>::get().unwrap(),1); //此时kittiesCount从0更新为1
	});
}


#[test]
fn transfer_kitties() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
        assert_ok!(KittiesModule::create_kitty(Origin::signed(0)));
		assert_ok!(KittiesModule::transfer(Origin::signed(0),1,0)); 
        assert_noop!(KittiesModule::transfer(Origin::signed(0),1,0),Error::<Test>::NotOwner);
        assert_eq!(Owner::<Test>::get(0),1); //拥有者从0变成1
	});
}



#[test]
fn breed_kitties() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
        assert_ok!(KittiesModule::create_kitty(Origin::signed(1)));
        assert_ok!(KittiesModule::create_kitty(Origin::signed(1)));

        assert_noop!(KittiesModule::breed_kitty(Origin::signed(1),0,0),Error::<Test>::SameParentIndex);
        assert_noop!(KittiesModule::breed_kitty(Origin::signed(1),0,2),Error::<Test>::InvalidKittyIndex);

        assert_ok!(KittiesModule::breed_kitty(Origin::signed(1),0,1));

		assert_eq!(Owner::<Test>::get(2),1); //新生成的id kittie的拥有者是1 account

        assert_eq!(KittiesCount::<Test>::get().unwrap(),3);
	});
}

#[test]
fn sell_kittie_kitties() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
        assert_ok!(KittiesModule::create_kitty(Origin::signed(0)));
        assert_ok!(KittiesModule::create_kitty(Origin::signed(1)));
		assert_ok!(KittiesModule::sell_kitties(Origin::signed(0),0,1)); 
        
        assert_eq!(KittiesTxPool::<Test>::get(0),1); //确定放入交易池中
      
        assert_noop!(KittiesModule::sell_kitties(Origin::signed(1),2,1),Error::<Test>::NotOwner); //出售不属于自己拥有的kittie会失败
	});
}


#[test]
fn buy_kittie_kitties() {
	new_test_ext().execute_with(|| {
		
        assert_ok!(KittiesModule::create_kitty(Origin::signed(1)));
        assert_ok!(KittiesModule::create_kitty(Origin::signed(1)));

        assert_ok!(KittiesModule::sell_kitties(Origin::signed(1),0,1000)); //出售0 kittie的价格为1000
        assert_ok!(KittiesModule::sell_kitties(Origin::signed(1),1,1)); //出售1 kittie的价格为1

        assert_noop!(KittiesModule::buy_kitties(Origin::signed(2),0),Error::<Test>::NotsufficientValue); //2的初始资金为100，不够支付
        assert_ok!(KittiesModule::buy_kitties(Origin::signed(2),1)); //支付成功
	});
}
