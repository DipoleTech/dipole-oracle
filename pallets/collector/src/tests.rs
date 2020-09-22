// Tests to be written here

// use super::*;
use crate::mock::*;
use frame_support::assert_ok;

// test cases for goods_oracle
#[test]
fn collect_oracle_data_should_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(CollectorModule::collect_oracle_data(Origin::signed(1)));
    })
}
