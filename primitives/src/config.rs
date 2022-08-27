// This file is part of Acala.

// Copyright (C) 2020-2022 Acala Foundation.
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

use super::{Balance, BlockNumber};
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{FixedU128, RuntimeDebug};

pub type Price = FixedU128;
pub type ExchangeRate = FixedU128;
pub type Ratio = FixedU128;
pub type Rate = FixedU128;

#[derive(Clone, Copy, Encode, Decode, RuntimeDebug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub enum ConfigKey {
	/// module_prices
	// The fixed prices of stable currency, it should be 1 USD in Acala.
	StableCurrencyFixedPrice,

	// The staking reward rate per relaychain block for StakingCurrency.
	// In fact, the staking reward is not settled according to the block on relaychain.
	// 17.5% annual staking reward rate of Kusama
	// 14% annual staking reward rate of Polkadot
	RewardRatePerRelaychainBlock,

	/// module_auction_manager
	MinimumIncrementSize,

	AuctionTimeToClose,

	AuctionDurationSoftCap,

	/// module_cdp_engine
	DefaultLiquidationRatio,

	DefaultDebitExchangeRate,

	DefaultLiquidationPenalty,

	MinimumDebitValue,

	MaxSwapSlippageCompareToOracle,

	MaxLiquidationContractSlippage,

	/// module_incentives
	AccumulatePeriod,

	EarnShareBooster,

	/// module_homa
	DefaultExchangeRate,

	MintThreshold,

	RedeemThreshold,
}

#[derive(Clone, Copy, Encode, Decode, RuntimeDebug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub enum ConfigValue {
	Balance(bool, Balance, Balance, Balance),
	BlockNumber(bool, BlockNumber, BlockNumber, BlockNumber),
	Rate(bool, Rate, Rate, Rate),
	Ratio(bool, Ratio, Ratio, Ratio),
	ExchangeRate(bool, ExchangeRate, ExchangeRate, ExchangeRate),
	Price(bool, Price, Price, Price),
}
