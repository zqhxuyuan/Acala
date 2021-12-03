// This file is part of Acala.

// Copyright (C) 2020-2021 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::setup::*;
use frame_support::{
	assert_err, assert_ok, parameter_types,
	traits::{Everything, IsInVec},
	weights::Weight,
};
use karura_runtime::{KsmPerSecond, ZeroAccountId};
use xcm_builder::{AllowSubscriptionsFrom, AllowTopLevelPaidExecutionFrom, FixedRateOfFungible, TakeWeightCredit};
use xcm_executor::{traits::*, Assets, Config, XcmExecutor};

#[test]
fn currency_id_convert() {
	ExtBuilder::default().build().execute_with(|| {
		let id: u32 = ParachainInfo::get().into();

		assert_eq!(
			CurrencyIdConvert::convert(RELAY_CHAIN_CURRENCY),
			Some(MultiLocation::parent())
		);

		assert_eq!(
			CurrencyIdConvert::convert(NATIVE_CURRENCY),
			Some(MultiLocation::sibling_parachain_general_key(
				id,
				NATIVE_CURRENCY.encode()
			))
		);
		assert_eq!(
			CurrencyIdConvert::convert(USD_CURRENCY),
			Some(MultiLocation::sibling_parachain_general_key(id, USD_CURRENCY.encode()))
		);
		assert_eq!(
			CurrencyIdConvert::convert(LIQUID_CURRENCY),
			Some(MultiLocation::sibling_parachain_general_key(
				id,
				LIQUID_CURRENCY.encode()
			))
		);
		assert_eq!(
			CurrencyIdConvert::convert(MultiLocation::parent()),
			Some(RELAY_CHAIN_CURRENCY)
		);
		assert_eq!(
			CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(
				id,
				NATIVE_CURRENCY.encode()
			)),
			Some(NATIVE_CURRENCY)
		);
		assert_eq!(
			CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(id, USD_CURRENCY.encode())),
			Some(USD_CURRENCY)
		);
		assert_eq!(
			CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(
				id,
				LIQUID_CURRENCY.encode()
			)),
			Some(LIQUID_CURRENCY)
		);

		#[cfg(feature = "with-mandala-runtime")]
		{
			assert_eq!(CurrencyIdConvert::convert(KAR), None);
			assert_eq!(CurrencyIdConvert::convert(KUSD), None);
			assert_eq!(CurrencyIdConvert::convert(KSM), None);
			assert_eq!(CurrencyIdConvert::convert(LKSM), None);

			assert_eq!(
				CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(id, RENBTC.encode())),
				Some(RENBTC)
			);
			assert_eq!(
				CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(id, KAR.encode())),
				None
			);
			assert_eq!(
				CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(id, KUSD.encode())),
				None
			);
			assert_eq!(
				CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(id, KSM.encode())),
				None
			);
			assert_eq!(
				CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(id, KSM.encode())),
				None
			);

			assert_eq!(
				CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(id + 1, RENBTC.encode())),
				None
			);

			let native_currency: MultiAsset = (
				MultiLocation::sibling_parachain_general_key(id, NATIVE_CURRENCY.encode()),
				1,
			)
				.into();
			assert_eq!(CurrencyIdConvert::convert(native_currency), Some(NATIVE_CURRENCY));
		}

		#[cfg(feature = "with-karura-runtime")]
		{
			assert_eq!(CurrencyIdConvert::convert(ACA), None);
			assert_eq!(CurrencyIdConvert::convert(AUSD), None);
			assert_eq!(CurrencyIdConvert::convert(DOT), None);
			assert_eq!(CurrencyIdConvert::convert(LDOT), None);

			assert_eq!(
				CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(id, ACA.encode())),
				None
			);
			assert_eq!(
				CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(id, AUSD.encode())),
				None
			);
			assert_eq!(
				CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(id, DOT.encode())),
				None
			);
			assert_eq!(
				CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(id, LDOT.encode())),
				None
			);
			assert_eq!(
				CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(
					parachains::bifrost::ID,
					parachains::bifrost::BNC_KEY.to_vec()
				)),
				Some(BNC)
			);
			assert_eq!(
				CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(
					parachains::bifrost::ID,
					parachains::bifrost::VSKSM_KEY.to_vec()
				)),
				Some(VSKSM)
			);

			assert_eq!(
				CurrencyIdConvert::convert(BNC),
				Some(MultiLocation::sibling_parachain_general_key(
					parachains::bifrost::ID,
					parachains::bifrost::BNC_KEY.to_vec()
				))
			);
			assert_eq!(
				CurrencyIdConvert::convert(VSKSM),
				Some(MultiLocation::sibling_parachain_general_key(
					parachains::bifrost::ID,
					parachains::bifrost::VSKSM_KEY.to_vec()
				))
			);

			let native_currency: MultiAsset = (
				MultiLocation::sibling_parachain_general_key(id, NATIVE_CURRENCY.encode()),
				1,
			)
				.into();
			assert_eq!(CurrencyIdConvert::convert(native_currency), Some(NATIVE_CURRENCY));
		}

		#[cfg(feature = "with-acala-runtime")]
		{
			assert_eq!(CurrencyIdConvert::convert(KAR), None);
			assert_eq!(CurrencyIdConvert::convert(KUSD), None);
			assert_eq!(CurrencyIdConvert::convert(KSM), None);
			assert_eq!(CurrencyIdConvert::convert(LKSM), None);

			assert_eq!(
				CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(id, RENBTC.encode())),
				None
			);
			assert_eq!(
				CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(id, KAR.encode())),
				None
			);
			assert_eq!(
				CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(id, KUSD.encode())),
				None
			);
			assert_eq!(
				CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(id, KSM.encode())),
				None
			);
			assert_eq!(
				CurrencyIdConvert::convert(MultiLocation::sibling_parachain_general_key(id, LKSM.encode())),
				None
			);

			let native_currency: MultiAsset = (
				MultiLocation::sibling_parachain_general_key(id, NATIVE_CURRENCY.encode()),
				1,
			)
				.into();
			assert_eq!(CurrencyIdConvert::convert(native_currency), Some(NATIVE_CURRENCY));
		}
	});
}

#[test]
fn parachain_subaccounts_are_unique() {
	ExtBuilder::default().build().execute_with(|| {
		let parachain: AccountId = ParachainInfo::parachain_id().into_account();
		assert_eq!(
			parachain,
			hex_literal::hex!["70617261d0070000000000000000000000000000000000000000000000000000"].into()
		);

		assert_eq!(
			RelayChainSovereignSubAccount::get(),
			create_x2_parachain_multilocation(0)
		);

		assert_eq!(
			create_x2_parachain_multilocation(0),
			MultiLocation::new(
				1,
				X1(Junction::AccountId32 {
					network: NetworkId::Any,
					id: hex_literal::hex!["d7b8926b326dd349355a9a7cca6606c1e0eb6fd2b506066b518c7155ff0d8297"].into(),
				})
			),
		);
		assert_eq!(
			create_x2_parachain_multilocation(1),
			MultiLocation::new(
				1,
				X1(Junction::AccountId32 {
					network: NetworkId::Any,
					id: hex_literal::hex!["74d37d762e06c6841a5dad64463a9afe0684f7e45245f6a7296ca613cca74669"].into(),
				})
			),
		);
	});
}

#[test]
fn weigher_weight_and_take_weight_credit_barrier_works() {
	let mut message = Xcm(vec![
		ReserveAssetDeposited((Parent, 100).into()),
		BuyExecution {
			fees: (Parent, 1).into(),
			weight_limit: Limited(10),
		},
		DepositAsset {
			assets: All.into(),
			max_assets: 1,
			beneficiary: Here.into(),
		},
	]);

	#[cfg(feature = "with-karura-runtime")]
	{
		let expect_weight: Weight = 600_000_000;
		let mut weight_credit = 1_000_000_000;
		assert_eq!(<XcmConfig as Config>::Weigher::weight(&mut message), Ok(expect_weight));
		let r = TakeWeightCredit::should_execute(&Parent.into(), &mut message, expect_weight, &mut weight_credit);
		assert_ok!(r);
		assert_eq!(weight_credit, 400_000_000);

		let r = TakeWeightCredit::should_execute(&Parent.into(), &mut message, expect_weight, &mut weight_credit);
		assert_eq!(r, Err(()));
		assert_eq!(weight_credit, 400_000_000);

		let r = XcmExecutor::<XcmConfig>::execute_xcm(Parent, message.clone(), 10);
		assert_eq!(r, Outcome::Error(XcmError::WeightLimitReached(expect_weight)));
	}

	#[cfg(feature = "with-mandala-runtime")]
	{
		let expect_weight: Weight = 3_000_000;
		let mut weight_credit = 4_000_000;
		assert_eq!(<XcmConfig as Config>::Weigher::weight(&mut message), Ok(expect_weight));
		let r = TakeWeightCredit::should_execute(&Parent.into(), &mut message, expect_weight, &mut weight_credit);
		assert_ok!(r);
		assert_eq!(weight_credit, 1_000_000);

		let r = TakeWeightCredit::should_execute(&Parent.into(), &mut message, expect_weight, &mut weight_credit);
		assert_eq!(r, Err(()));
		assert_eq!(weight_credit, 1_000_000);

		let r = XcmExecutor::<XcmConfig>::execute_xcm(Parent, message.clone(), 10);
		assert_eq!(r, Outcome::Error(XcmError::WeightLimitReached(expect_weight)));
	}

	#[cfg(feature = "with-acala-runtime")]
	{
		assert_eq!(<XcmConfig as Config>::Weigher::weight(&mut message), Ok(600_000_000));
	}
}

#[cfg(feature = "with-karura-runtime")]
#[test]
fn top_level_paid_execution_barrier_works() {
	let mut message = Xcm::<karura_runtime::Call>(vec![
		ReserveAssetDeposited((Parent, 100).into()),
		BuyExecution {
			fees: (Parent, 1).into(),
			weight_limit: Limited(10),
		},
		DepositAsset {
			assets: All.into(),
			max_assets: 1,
			beneficiary: Here.into(),
		},
	]);

	// BuyExecution weight_limit set to 10
	let r = AllowTopLevelPaidExecutionFrom::<Everything>::should_execute(&Parent.into(), &mut message, 10, &mut 0);
	assert_ok!(r);

	// BuyExecution weight_limit less than max_weight, error
	let r = AllowTopLevelPaidExecutionFrom::<Everything>::should_execute(&Parent.into(), &mut message, 20, &mut 0);
	assert_eq!(r, Err(()));
}

#[test]
fn barrier_contains_works() {
	parameter_types! {
		pub static AllowUnpaidFrom: Vec<MultiLocation> = vec![];
		pub static AllowPaidFrom: Vec<MultiLocation> = vec![];
		pub static AllowSubsFrom: Vec<MultiLocation> = vec![Parent.into()];
	}
	let mut message1 = Xcm::<()>(vec![
		ReserveAssetDeposited((Parent, 100).into()),
		BuyExecution {
			fees: (Parent, 1).into(),
			weight_limit: Limited(20),
		},
		DepositAsset {
			assets: All.into(),
			max_assets: 1,
			beneficiary: Here.into(),
		},
	]);
	let mut message2 = Xcm::<()>(vec![SubscribeVersion {
		query_id: 42,
		max_response_weight: 5000,
	}]);

	// T::Contains set to Parent
	AllowSubsFrom::set(vec![Parent.into()]);
	let r = AllowTopLevelPaidExecutionFrom::<IsInVec<AllowSubsFrom>>::should_execute(
		&Parent.into(),
		&mut message1,
		10,
		&mut 0,
	);
	assert_ok!(r);
	let r = AllowSubscriptionsFrom::<IsInVec<AllowSubsFrom>>::should_execute(&Parent.into(), &mut message2, 20, &mut 0);
	assert_ok!(r);

	// T::Contains set to Parachain(1000)
	AllowSubsFrom::set(vec![Parachain(1000).into()]);
	let r = AllowTopLevelPaidExecutionFrom::<IsInVec<AllowSubsFrom>>::should_execute(
		&Parent.into(),
		&mut message1,
		10,
		&mut 0,
	);
	assert_eq!(r, Err(()));
	let r = AllowSubscriptionsFrom::<IsInVec<AllowSubsFrom>>::should_execute(&Parent.into(), &mut message2, 20, &mut 0);
	assert_eq!(r, Err(()));

	// T::Contains set to empty
	AllowSubsFrom::set(vec![]);
	let r = AllowTopLevelPaidExecutionFrom::<IsInVec<AllowSubsFrom>>::should_execute(
		&Parent.into(),
		&mut message1,
		10,
		&mut 0,
	);
	assert_eq!(r, Err(()));
	let r = AllowSubscriptionsFrom::<IsInVec<AllowSubsFrom>>::should_execute(&Parent.into(), &mut message2, 20, &mut 0);
	assert_eq!(r, Err(()));
	let r = AllowTopLevelPaidExecutionFrom::<Everything>::should_execute(&Parent.into(), &mut message1, 10, &mut 0);
	assert_ok!(r);
	let r = AllowSubscriptionsFrom::<Everything>::should_execute(&Parent.into(), &mut message2, 20, &mut 0);
	assert_ok!(r);
}

#[test]
fn xcm_executor_execute_xcm() {
	#[cfg(feature = "with-karura-runtime")]
	{
		ExtBuilder::default().build().execute_with(|| {
			// weight limited set to Unlimited, it's ok
			let message = Xcm::<karura_runtime::Call>(vec![
				ReserveAssetDeposited((Parent, 600_000_000).into()),
				BuyExecution {
					fees: (Parent, 600_000_000).into(),
					weight_limit: Unlimited,
				},
				DepositAsset {
					assets: All.into(),
					max_assets: 1,
					beneficiary: Here.into(),
				},
			]);

			let r = XcmExecutor::<XcmConfig>::execute_xcm(Parent, message, 600_000_000);
			assert_eq!(r, Outcome::Complete(600_000_000));
		});
	}

	#[cfg(feature = "with-acala-runtime")]
	{
		ExtBuilder::default().build().execute_with(|| {
			// weight limited large than xcm_weight, it's ok
			let message = Xcm::<acala_runtime::Call>(vec![
				ReserveAssetDeposited((Parent, 600_000_000).into()),
				BuyExecution {
					fees: (Parent, 600_000_000).into(),
					weight_limit: Limited(6_000_000_000),
				},
				DepositAsset {
					assets: All.into(),
					max_assets: 1,
					beneficiary: Here.into(),
				},
			]);

			let r = XcmExecutor::<acala_runtime::XcmConfig>::execute_xcm(Parent, message, 600_000_000);
			assert_eq!(r, Outcome::Complete(600_000_000));
		});
	}

	#[cfg(feature = "with-mandala-runtime")]
	{
		ExtBuilder::default().build().execute_with(|| {
			// weight limited less than xcm_weight, it's error
			let message = Xcm::<mandala_runtime::Call>(vec![
				ReserveAssetDeposited((Parent, 3_000_000).into()),
				BuyExecution {
					fees: (Parent, 3_000_000).into(),
					weight_limit: Limited(300_000),
				},
				DepositAsset {
					assets: All.into(),
					max_assets: 1,
					beneficiary: Here.into(),
				},
			]);

			let r = XcmExecutor::<mandala_runtime::XcmConfig>::execute_xcm(Parent, message, 3_000_000);
			assert_eq!(r, Outcome::Error(XcmError::Barrier));
		});
	}
}

#[cfg(feature = "with-karura-runtime")]
#[test]
fn subscribe_version_barrier_works() {
	ExtBuilder::default().build().execute_with(|| {
		// BadOrigin if original origin is not equal to origin
		let origin = Parachain(1000).into();
		let message = Xcm(vec![
			DescendOrigin(X1(AccountIndex64 { index: 1, network: Any })),
			SubscribeVersion {
				query_id: 42,
				max_response_weight: 5000,
			},
		]);
		let weight_limit = 2_000_000_000;
		let r = XcmExecutor::<kusama_runtime::XcmConfig>::execute_xcm_in_credit(
			origin.clone(),
			message.clone(),
			weight_limit,
			weight_limit,
		);
		assert_eq!(r, Outcome::Incomplete(weight_limit, XcmError::BadOrigin));

		// relay chain force subscribe version notify of karura para chain
		let message = Xcm(vec![SubscribeVersion {
			query_id: 42,
			max_response_weight: 5000,
		}]);
		let weight_limit = 1_000_000_000;
		let r = XcmExecutor::<karura_runtime::XcmConfig>::execute_xcm_in_credit(
			Parent,
			message.clone(),
			weight_limit,
			weight_limit,
		);
		assert_eq!(r, Outcome::Complete(200_000_000));
	});
}

#[test]
fn test_native_token_can_not_use_tokens_module() {
	// Karura::execute_with(|| {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(Tokens::free_balance(KAR, &AccountId::from(ALICE)), 0);

		let _ = pallet_balances::Pallet::<Runtime>::deposit_creating(&AccountId::from(ALICE), 1000 * dollar(KUSD));
		assert_eq!(Balances::free_balance(&AccountId::from(ALICE)), 1000 * dollar(KUSD));

		assert_ok!(orml_tokens::Pallet::<Runtime>::deposit(
			KUSD,
			&AccountId::from(ALICE),
			1000 * dollar(KUSD)
		));
		assert_eq!(Tokens::free_balance(KUSD, &AccountId::from(ALICE)), 1000 * dollar(KUSD));

		assert_ok!(Tokens::deposit(KUSD, &AccountId::from(ALICE), 1000 * dollar(KUSD)));
		assert_eq!(Tokens::free_balance(KUSD, &AccountId::from(ALICE)), 2000 * dollar(KUSD));

		// 1 KAR = 1 KUSD
		let _ = pallet_balances::Pallet::<Runtime>::deposit_creating(&AccountId::from(ALICE), 1000 * dollar(KAR));
		assert_eq!(Balances::free_balance(&AccountId::from(ALICE)), 2000 * dollar(KAR));

		// orml_tokens ExistentialDeposits of KAR... is Balance::max_value(), so not support direct deposit
		// KAR to someone 因为 native token 是 KAR，不能通过 tokens 直接 deposit KAR，只能通过 balances 模块
		assert_err!(
			orml_tokens::Pallet::<Runtime>::deposit(KAR, &AccountId::from(ALICE), 1000 * dollar(KAR)),
			orml_tokens::Error::<Runtime>::ExistentialDeposit
		);
		assert_eq!(Tokens::free_balance(KAR, &AccountId::from(ALICE)), 0);
		// 除了 native token，还有其他跨链暂时不支持，比如当前是 karura runtime，就没办法 deposit
		// ACA、DOT 因为 ACA 是另外一条链，而 DOT 是另外一条中继链（而 KSM 可以，因为 karura 和
		// kusama 是允许跨链通信的）
	});
}

#[test]
fn test_account() {
	// 5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM
	// https://polkadot.subscan.io/tools/format_transform?input=111111111111111111111111111111111HC1&type=All
	// https://polkadot.subscan.io/account/111111111111111111111111111111111HC1
	let zero = ZeroAccountId::get();
	println!("{}", zero);
}

#[test]
fn test_weight_fee_static() {
	use frame_support::weights::constants::WEIGHT_PER_SECOND;
	use karura_runtime::kar_per_second;

	println!("kar_ps:{}", kar_per_second());

	let weight = 100;
	let amount = kar_per_second() * (weight as u128) / (WEIGHT_PER_SECOND as u128);
	println!("amount:{}", amount);
}

#[cfg(feature = "with-karura-runtime")]
#[test]
fn test_fix_rate() {
	let mut message = Xcm(vec![
		ReserveAssetDeposited((Parent, 100).into()),
		BuyExecution {
			fees: (Parent, 100).into(),
			weight_limit: Limited(100),
		},
		DepositAsset {
			assets: All.into(),
			max_assets: 1,
			beneficiary: Here.into(),
		},
	]);

	let expect_weight: Weight = 600_000_000;
	let xcm_weight: Weight = <XcmConfig as Config>::Weigher::weight(&mut message).unwrap();
	assert_eq!(xcm_weight, expect_weight);

	// ksm: 0.16 * weight
	let mock_weight: Weight = 100;
	let asset: MultiAsset = (Parent, 100).into();
	let expect_result: MultiAsset = (Parent, 84).into();
	let assets: Assets = asset.into();
	let mut trader = FixedRateOfFungible::<KsmPerSecond, ()>::new();
	let assets =
		<FixedRateOfFungible<KsmPerSecond, ()> as WeightTrader>::buy_weight(&mut trader, mock_weight, assets).unwrap();
	let asset: Vec<MultiAsset> = assets.into();
	assert_eq!(vec![expect_result], asset);

	// 0.16 * 600_000_000 = 96_000_000
	let asset: MultiAsset = (Parent, 96_000_000).into();
	let assets: Assets = asset.into();
	let mut trader = FixedRateOfFungible::<KsmPerSecond, ()>::new();
	let assets =
		<FixedRateOfFungible<KsmPerSecond, ()> as WeightTrader>::buy_weight(&mut trader, xcm_weight, assets).unwrap();
	let asset: Vec<MultiAsset> = assets.into();
	assert_eq!(asset, vec![]);

	// 100_000_000 - 96_000_000 = 4_000_000
	let asset: MultiAsset = (Parent, 100_000_000).into();
	let expect_result: MultiAsset = (Parent, 4_000_000).into();
	let assets: Assets = asset.into();
	let mut trader = FixedRateOfFungible::<KsmPerSecond, ()>::new();
	let assets =
		<FixedRateOfFungible<KsmPerSecond, ()> as WeightTrader>::buy_weight(&mut trader, xcm_weight, assets).unwrap();
	let asset: Vec<MultiAsset> = assets.into();
	assert_eq!(vec![expect_result], asset);
}

// #[cfg(feature = "with-karura-runtime")]
// mod karura_runtime_test {
// 	use super::*;
// use pallet_transaction_payment::InclusionFee;
// use sp_runtime::traits::Extrinsic;
//
// #[test]
// fn check_transaction_fee_for_empty_remark_karura() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		let call = Call::System(frame_system::Call::remark { remark: vec![] });
// 		let ext = UncheckedExtrinsic::new(call.into(), None).expect("This should not fail");
// 		let bytes = ext.encode();
// 		println!("remark:{:?}", hex::encode(bytes));
//
// 		// Get information on the fee for the call.
// 		let fee = TransactionPayment::query_fee_details(ext, bytes.len() as u32);
//
// 		let InclusionFee {
// 			base_fee,
// 			len_fee,
// 			adjusted_weight_fee,
// 		} = fee.inclusion_fee.unwrap();
//
// 		// assert_eq!(base_fee, 1_000_000_000);
// 		// assert_eq!(len_fee, 500_000_000);
// 		// assert_eq!(adjusted_weight_fee, 4_592_000);
// 		//
// 		// let total_fee = base_fee.saturating_add(len_fee).saturating_add(adjusted_weight_fee);
// 		// assert_eq!(total_fee, 1_504_592_000);
// 		println!("fee:{}", fee.inclusion_fee);
// 	});
// }
// }

#[cfg(feature = "with-mandala-runtime")]
mod mandala_only_tests {
	use super::*;
	use pallet_transaction_payment::InclusionFee;
	use sp_runtime::traits::Extrinsic;
	#[test]
	fn check_transaction_fee_for_empty_remark() {
		ExtBuilder::default().build().execute_with(|| {
			let call = Call::System(frame_system::Call::remark { remark: vec![] });
			let ext = UncheckedExtrinsic::new(call.into(), None).expect("This should not fail");
			let bytes = ext.encode();

			// Get information on the fee for the call.
			let fee = TransactionPayment::query_fee_details(ext, bytes.len() as u32);

			let InclusionFee {
				base_fee,
				len_fee,
				adjusted_weight_fee,
			} = fee.inclusion_fee.unwrap();

			assert_eq!(base_fee, 1_000_000_000);
			assert_eq!(len_fee, 500_000_000);
			assert_eq!(adjusted_weight_fee, 4_592_000);

			let total_fee = base_fee.saturating_add(len_fee).saturating_add(adjusted_weight_fee);
			assert_eq!(total_fee, 1_504_592_000);
		});
	}
}
