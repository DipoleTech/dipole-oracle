// Tests to be written here

use crate::{mock::*};
use frame_support::{assert_ok};
use super::*;
use sp_io::hashing::blake2_256;

// test cases for pay_oracle
#[test]
fn feed_pay_data_should_work() {
    new_test_ext().execute_with(|| {
        let did = blake2_256(b"test");
        let new_operator_id = Did { did };
        let operator_price = OperatorPrice{
            pay_id: 1,
            price: 100,
        };
        assert_ok!(PayOracleModule::feed_data(new_operator_id.clone(), vec!(operator_price)));
    })
}