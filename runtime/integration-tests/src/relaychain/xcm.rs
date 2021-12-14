use crate::relaychain::kusama_test_net::*;
use crate::setup::*;

use frame_support::{assert_err, assert_noop, assert_ok};

use cumulus_primitives_core::ParaId;
use karura_runtime::{AssetRegistry, KaruraTreasuryAccount};
use module_asset_registry::AssetMetadata;
use module_relaychain::RelayChainCallBuilder;
use module_support::CallBuilder;
use orml_traits::MultiCurrency;
use sp_runtime::traits::AccountIdConversion;
use xcm_emulator::TestExt;

pub type RelayChainPalletXcm = pallet_xcm::Pallet<kusama_runtime::Runtime>;
pub type ParachainPalletXcm = pallet_xcm::Pallet<karura_runtime::Runtime>;
pub type RelayBalances = pallet_balances::Pallet<kusama_runtime::Runtime>;
pub type ParaBalances = pallet_balances::Pallet<karura_runtime::Runtime>;
pub type ParaTokens = orml_tokens::Pallet<karura_runtime::Runtime>;
pub type ParaXTokens = orml_xtokens::Pallet<karura_runtime::Runtime>;

pub fn para_karura_account() -> sp_runtime::AccountId32 {
	ParaId::from(2000).into_account()
}

#[test]
fn teleport_from_relay_chain_v1_imbalance() {
	// env_logger::init();

	KusamaNet::execute_with(|| {
		assert_ok!(RelayChainPalletXcm::teleport_assets(
			kusama_runtime::Origin::signed(ALICE.into()),
			Box::new(Parachain(2000).into().into()),
			Box::new(
				Junction::AccountId32 {
					id: BOB,
					network: NetworkId::Any
				}
				.into()
				.into()
			),
			Box::new((Here, dollar(KSM)).into()),
			0
		));
		// RelayChain account withdrawn, but ParaChain account not deposited
		assert_eq!(RelayBalances::free_balance(&AccountId::from(ALICE)), 2001 * dollar(KSM));
	});

	Karura::execute_with(|| {
		assert_eq!(Tokens::free_balance(KSM, &AccountId::from(BOB)), 0);
	});
}

#[test]
fn transfer_from_para_chain_v1_imbalance() {
	// env_logger::init();

	Karura::execute_with(|| {
		assert_ok!(ParachainPalletXcm::reserve_transfer_assets(
			karura_runtime::Origin::signed(ALICE.into()),
			Box::new(xcm::VersionedMultiLocation::V1(xcm::v1::Parent.into())),
			// Box::new(xcm::v1::Parent.into().into()),
			Box::new(
				Junction::AccountId32 {
					id: BOB,
					network: NetworkId::Any
				}
				.into()
				.into()
			),
			Box::new((xcm::v1::Parent, 1000000000000).into()),
			0,
		));

		assert_eq!(ParaTokens::free_balance(KSM, &AccountId::from(ALICE)), 9 * dollar(KSM));
	});

	KusamaNet::execute_with(|| {
		assert_eq!(RelayBalances::free_balance(&AccountId::from(BOB)), 0);
	});
}

#[test]
fn teleport_from_para_chain_v1_filtered() {
	// env_logger::init();

	Karura::execute_with(|| {
		assert_noop!(
			ParachainPalletXcm::teleport_assets(
				karura_runtime::Origin::signed(ALICE.into()),
				Box::new(xcm::VersionedMultiLocation::V1(xcm::v1::Parent.into())),
				// Box::new(xcm::v1::Parent.into().into()),
				Box::new(
					Junction::AccountId32 {
						id: BOB,
						network: NetworkId::Any
					}
					.into()
					.into()
				),
				Box::new((xcm::v1::Parent, 1000000000000).into()),
				0,
			),
			pallet_xcm::Error::<karura_runtime::Runtime>::Filtered
		);
		assert_eq!(ParaTokens::free_balance(KSM, &AccountId::from(ALICE)), 10 * dollar(KSM));
	});

	KusamaNet::execute_with(|| {
		assert_eq!(Tokens::free_balance(KSM, &AccountId::from(BOB)), 0);
	});
}

#[test]
fn transfer_from_relay_chain_v0() {
	use xcm::v0::Junction::*;
	use xcm::v0::MultiAsset::*;
	use xcm::v0::Order::*;
	use xcm::v0::*;
	use xcm::*;

	KusamaNet::execute_with(|| {
		assert_ok!(RelayChainPalletXcm::reserve_transfer_assets(
			kusama_runtime::Origin::signed(ALICE.into()),
			Box::new(VersionedMultiLocation::V0(X1(Parachain(2000)))),
			Box::new(VersionedMultiLocation::V0(X1(AccountId32 {
				network: Any,
				id: BOB.into()
			}))),
			Box::new(VersionedMultiAssets::V0(vec![ConcreteFungible {
				id: MultiLocation::Null,
				amount: 1000000000000,
			}])),
			0,
		));
		assert_eq!(RelayBalances::free_balance(&AccountId::from(ALICE)), 2001 * dollar(KSM));
		assert_eq!(
			RelayBalances::free_balance(&AccountId::from(para_karura_account())),
			3 * dollar(KSM)
		);
	});

	Karura::execute_with(|| {
		// assert_eq!(Tokens::free_balance(KSM, &AccountId::from(BOB)), 999_936_000_000);
		assert_eq!(Tokens::free_balance(KSM, &AccountId::from(BOB)), 999872000000);
	});
}

#[test]
fn transfer_from_para_chain_v0_imbalance() {
	// env_logger::init();
	use xcm::v0::Junction::*;
	use xcm::v0::MultiAsset::*;
	use xcm::v0::Order::*;
	use xcm::v0::*;
	use xcm::*;

	Karura::execute_with(|| {
		assert_ok!(ParachainPalletXcm::reserve_transfer_assets(
			karura_runtime::Origin::signed(ALICE.into()),
			Box::new(VersionedMultiLocation::V0(X1(Parent))),
			Box::new(VersionedMultiLocation::V0(X1(AccountId32 {
				network: Any,
				id: BOB.into()
			}))),
			Box::new(VersionedMultiAssets::V0(vec![ConcreteFungible {
				id: MultiLocation::X1(Parent),
				amount: 1000000000000,
			}])),
			0,
		));
		assert_eq!(ParaTokens::free_balance(KSM, &AccountId::from(ALICE)), 9 * dollar(KSM));
	});

	KusamaNet::execute_with(|| {
		assert_eq!(Tokens::free_balance(KSM, &AccountId::from(BOB)), 0);
	});
}

#[test]
fn teleport_from_para_chain_v0() {
	use xcm::v0::Junction::*;
	use xcm::v0::MultiAsset::*;
	use xcm::v0::Order::*;
	use xcm::v0::*;
	use xcm::*;
	// env_logger::init();

	Karura::execute_with(|| {
		assert_noop!(
			ParachainPalletXcm::teleport_assets(
				karura_runtime::Origin::signed(ALICE.into()),
				Box::new(VersionedMultiLocation::V0(X1(Parent))),
				Box::new(VersionedMultiLocation::V0(X1(AccountId32 {
					network: Any,
					id: BOB.into()
				}))),
				Box::new(VersionedMultiAssets::V0(vec![ConcreteFungible {
					id: MultiLocation::X1(Parent),
					amount: 1000000000000,
				}])),
				0,
			),
			pallet_xcm::Error::<karura_runtime::Runtime>::Filtered
		);
		assert_eq!(ParaTokens::free_balance(KSM, &AccountId::from(ALICE)), 10 * dollar(KSM));
	});

	KusamaNet::execute_with(|| {
		assert_eq!(Tokens::free_balance(KSM, &AccountId::from(BOB)), 0);
	});
}
