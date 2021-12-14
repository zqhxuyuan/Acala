use crate::setup::*;

use frame_support::{
	assert_err, assert_ok, parameter_types,
	traits::{Everything, IsInVec},
	weights::Weight,
};
use xcm_builder::{AllowSubscriptionsFrom, AllowTopLevelPaidExecutionFrom, FixedRateOfFungible, TakeWeightCredit};
use xcm_executor::{traits::*, Assets, Config, XcmExecutor};

use karura_runtime::KsmPerSecond;

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
	// let zero = ZeroAccountId::get();
	// println!("{}", zero);
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

	// 4 instructions
	let mut message = Xcm(vec![
		ReserveAssetDeposited((Parent, 100).into()),
		ClearOrigin,
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
	let expect_weight: Weight = 800_000_000;
	let xcm_weight: Weight = <XcmConfig as Config>::Weigher::weight(&mut message).unwrap();
	assert_eq!(xcm_weight, expect_weight);

	// 0.16 * 800_000_000 = 128_000_000
	let asset: MultiAsset = (Parent, 150_000_000).into();
	let expect_result: MultiAsset = (Parent, 22_000_000).into();
	let assets: Assets = asset.into();
	let mut trader = FixedRateOfFungible::<KsmPerSecond, ()>::new();
	let assets =
		<FixedRateOfFungible<KsmPerSecond, ()> as WeightTrader>::buy_weight(&mut trader, xcm_weight, assets).unwrap();
	let asset: Vec<MultiAsset> = assets.into();
	assert_eq!(vec![expect_result], asset);
}
