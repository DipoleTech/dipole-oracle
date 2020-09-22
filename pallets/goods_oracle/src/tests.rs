// Tests to be written here

use super::*;
use crate::mock::*;
use frame_support::assert_ok;
use sp_io::hashing::blake2_256;

// test cases for goods_oracle
#[test]
fn feed_goods_data_should_work() {
    new_test_ext().execute_with(|| {
        let did = blake2_256(b"test");
        let new_operator_id = Did { did };
        let raw_volume = RawVolume {
            volume_type: VOLUME_TYPE_PEAK,
            volume: 1,
        };
        let operator_raw_volumes = vec![raw_volume];
        let operator_volume = OperatorVolume {
            operator_id: new_operator_id.clone(),
            operator_raw_volume: operator_raw_volumes.clone(),
        };
        assert_ok!(GoodsOracleModule::feed_goods_data(
            Origin::signed(1),
            new_operator_id.clone(),
            vec!(operator_volume)
        ));
    })
}

// test cases for goods_oracle
#[test]
fn feed_data_should_work() {
    new_test_ext().execute_with(|| {
        let did = blake2_256(b"test");
        let new_operator_id = Did { did };
        let raw_volume_1 = RawVolume {
            volume_type: VOLUME_TYPE_PEAK,
            volume: 1,
        };
        let raw_volume_2 = RawVolume {
            volume_type: VOLUME_TYPE_FLAT,
            volume: 2,
        };
        let raw_volume_3 = RawVolume {
            volume_type: VOLUME_TYPE_VALLEY,
            volume: 3,
        };
        let operator_raw_volumes = vec![raw_volume_1, raw_volume_2, raw_volume_3];
        let operator_volume = OperatorVolume {
            operator_id: new_operator_id.clone(),
            operator_raw_volume: operator_raw_volumes.clone(),
        };
        assert_ok!(GoodsOracleModule::feed_data(
            new_operator_id.clone(),
            vec!(operator_volume)
        ));
    })
}

// test cases for goods_oracle
#[test]
fn get_goods_owners_in_update_pool_should_work() {
    new_test_ext().execute_with(|| {
        let did = blake2_256(b"test");
        let new_operator_id = Did { did };
        let raw_volume_1 = RawVolume {
            volume_type: VOLUME_TYPE_PEAK,
            volume: 1,
        };
        let operator_raw_volumes = vec![raw_volume_1];
        let operator_volume = OperatorVolume {
            operator_id: new_operator_id.clone(),
            operator_raw_volume: operator_raw_volumes.clone(),
        };
        assert_ok!(GoodsOracleModule::feed_goods_data(
            Origin::signed(2),
            new_operator_id.clone(),
            vec!(operator_volume)
        ));
        assert_eq!(
            GoodsOracleModule::get_goods_owners_in_update_pool(),
            vec!(2)
        );
    })
}
