// Tests to be written here

use super::*;
use crate::mock::*;
use frame_support::assert_ok;
use sp_io::hashing::blake2_256;

// test cases for pay_oracle
#[test]
fn feed_pay_data_should_work() {
    new_test_ext().execute_with(|| {
        let did = blake2_256(b"test");
        let new_operator_id = Did { did };
        let operator_price = OperatorPrice {
            pay_id: 1,
            price: 100,
        };
        assert_ok!(PayOracleModule::feed_pay_data(
            Origin::signed(1),
            new_operator_id.clone(),
            vec!(operator_price)
        ));
    })
}

// test cases for pay_oracle
#[test]
fn feed_data_should_work() {
    new_test_ext().execute_with(|| {
        let did = blake2_256(b"test");
        let new_operator_id = Did { did };
        let operator_price = OperatorPrice {
            pay_id: 1,
            price: 100,
        };
        assert_ok!(PayOracleModule::feed_data(
            new_operator_id.clone(),
            vec!(operator_price)
        ));
    })
}

// test cases for goods_oracle
#[test]
fn get_pay_in_update_pool_should_work() {
    new_test_ext().execute_with(|| {
        let did = blake2_256(b"test");
        let new_operator_id = Did { did };
        let operator_price = OperatorPrice {
            pay_id: 1,
            price: 100,
        };
        assert_ok!(PayOracleModule::feed_data(
            new_operator_id.clone(),
            vec!(operator_price)
        ));
        assert_eq!(PayOracleModule::get_pay_in_update_pool(), vec!(1));
    })
}

// test cases for goods_oracle
#[test]
fn get_pay_data_should_work() {
    new_test_ext().execute_with(|| {
        let did = blake2_256(b"test");
        let new_operator_id = Did { did };
        let operator_price = OperatorPrice {
            pay_id: 1,
            price: 100,
        };
        assert_ok!(PayOracleModule::feed_data(
            new_operator_id.clone(),
            vec!(operator_price)
        ));
        assert_eq!(PayOracleModule::get_pay_data(1), Some(100));
    })
}
