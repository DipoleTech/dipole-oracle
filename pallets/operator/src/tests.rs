// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

// test cases for operator
#[test]
fn register_operator_should_work() {
    new_test_ext().execute_with(|| {
        let did = blake2_256(b"test");
		let new_operator_id = Did { did };
        assert_ok!(OperatorModule::new_operator(1, new_operator_id.clone(), OperatorRole::PublicProducer, OperatorCategory::ElectricMeter));
    })
}

// test cases for operator
#[test]
fn update_operator_should_work() {
    new_test_ext().execute_with(|| {
        let did = blake2_256(b"test");
		let new_operator_id = Did { did };
        assert_ok!(OperatorModule::new_operator(1, new_operator_id.clone(), OperatorRole::PublicProducer, OperatorCategory::ElectricMeter));
        assert_ok!(OperatorModule::update_operator(
            new_operator_id.clone(), 
            Operator {
                owner: 2,
                role: OperatorRole::PrivateProducer,
                category: OperatorCategory::ElectricMeter,
                is_legal: true,
            }
        ));
    })
}

// test cases for operator
#[test]
fn close_operator_should_work() {
    new_test_ext().execute_with(|| {
        let did = blake2_256(b"test");
		let new_operator_id = Did { did };
        assert_ok!(OperatorModule::new_operator(1, new_operator_id.clone(), OperatorRole::PublicProducer, OperatorCategory::ElectricMeter));
        assert_ok!(OperatorModule::close_operator(Origin::signed(1), new_operator_id.clone()));
        assert_eq!(Operators::<Test>::get(&new_operator_id).unwrap().is_legal, false);
    })
}

// test cases for operator
#[test]
fn cannot_close_operator_when_operator_not_exists() {
    new_test_ext().execute_with(|| {
        let did = blake2_256(b"test");
        let new_operator_id = Did { did };
        assert_noop!(
			OperatorModule::close_operator(Origin::signed(1), new_operator_id.clone()),
			Error::<Test>::UnknownOperator
		);
    })
}