// Tests to be written here

use crate::{mock::*};
use frame_support::{assert_ok};
use super::*;
use sp_io::hashing::blake2_256;

// test cases for goods_oracle
#[test]
fn feed_goods_data_should_work() {
    new_test_ext().execute_with(|| {
        let did = blake2_256(b"test");
        let new_operator_id = Did { did };
        let raw_volume = RawVolume{
            volume_type: VOLUME_TYPE_PEAK,
            volume: 1,
        };
        let operator_raw_volumes = vec!(raw_volume);
        let operator_volume = OperatorVolume{
            operator_id: new_operator_id.clone(),
            operator_raw_volume: operator_raw_volumes.clone(),
        };
        assert_ok!(GoodsOracleModule::feed_data(new_operator_id.clone(), vec!(operator_volume)));
    })
}