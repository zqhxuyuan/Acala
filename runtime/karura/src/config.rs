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

pub use crate::constants::time::*;
pub use frame_support::parameter_types;
pub use primitives::{Balance, BlockNumber};
pub use runtime_common::{dollar, ExchangeRate, Price, Rate, Ratio, KSM, KUSD, LKSM};
use sp_runtime::{FixedPointNumber, Permill};

parameter_types! {

	/// module_prices
	pub StableCurrencyFixedPrice: Price = Price::saturating_from_rational(1, 1);
	// 17.5% annual staking reward rate of Kusama
	pub RewardRatePerRelaychainBlock: Rate = Rate::saturating_from_rational(3_068, 100_000_000_000u128);

	/// module_auction_manager
	pub MinimumIncrementSize: Rate = Rate::saturating_from_rational(2, 100);
	pub const AuctionTimeToClose: BlockNumber = 15 * MINUTES;
	pub const AuctionDurationSoftCap: BlockNumber = 2 * HOURS;

	/// module_cdp_engine
	pub DefaultLiquidationRatio: Ratio = Ratio::saturating_from_rational(150, 100);
	pub DefaultDebitExchangeRate: ExchangeRate = ExchangeRate::saturating_from_rational(1, 10);
	pub DefaultLiquidationPenalty: Rate = Rate::saturating_from_rational(8, 100);
	pub MinimumDebitValue: Balance = 50 * dollar(KUSD);
	pub MaxSwapSlippageCompareToOracle: Ratio = Ratio::saturating_from_rational(10, 100);
	pub MaxLiquidationContractSlippage: Ratio = Ratio::saturating_from_rational(15, 100);

	/// module_incentives
	pub const AccumulatePeriod: BlockNumber = MINUTES;
	pub const EarnShareBooster: Permill = Permill::from_percent(30);

	/// module_homa
	pub DefaultExchangeRate: ExchangeRate = ExchangeRate::saturating_from_rational(1, 10);
	pub MintThreshold: Balance = dollar(KSM);
	pub RedeemThreshold: Balance = dollar(LKSM);
}
