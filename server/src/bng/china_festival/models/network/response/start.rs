use super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::server::{ContextActionsEnum, BonusBsVEnum, ContextCurrentEnum, ContextLastActionEnum, LastargsSelectedModeEnum, ServerModesEnum, SymbolsTypeEnum, StatusCodeEnum, ServerCommandEnum, StatusTypeEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bac {
	#[serde(rename = "1")]
	pub bac_1: Vec<i64> /* [0,0], [1,0], [1,1], [10,1], [11,11], [11,19], [11,23], [11,2], [12,15], [12,24], [12,28], [13,0], [2,0], [2,2], [3,0], [3,1], [3,2], [3,3], [4,0], [4,1], [4,2], [4,3], [5,0], [5,1], [5,2], [5,3], [5,4], [6,0], [6,1], [7,2], [8,1], [8,4], [8,5], [8,8], [9,11], [9,14], [9,6], [9,7], [9,9] */,
	#[serde(rename = "2")]
	pub bac_2: Vec<i64> /* [0,0], [10,11], [10,14], [10,16], [10,2], [10,8], [11,10], [11,23], [12,10], [12,15], [12,22], [12,4], [13,0], [2,0], [2,1], [3,0], [3,1], [3,2], [3,3], [4,0], [4,1], [4,2], [4,3], [5,0], [5,1], [5,2], [6,0], [6,2], [6,3], [7,5], [7,6], [7,7], [8,7], [8,8], [9,12], [9,14], [9,4] */,
	#[serde(rename = "3")]
	pub bac_3: Vec<i64> /* [0,0], [1,0], [10,11], [10,5], [11,10], [11,1], [11,24], [12,11], [12,12], [12,1], [12,23], [12,28], [13,0], [2,2], [3,0], [3,1], [3,2], [3,3], [4,0], [4,1], [4,2], [5,0], [5,1], [5,2], [5,3], [5,4], [6,2], [6,4], [7,0], [8,1], [8,3], [8,4], [8,6], [8,7], [8,8], [9,0], [9,12], [9,9] */,
}

impl From<server::Bac> for Bac {
	fn from(obj: server::Bac) -> Self {
		Bac {
			bac_1: obj.bac_1,
			bac_2: obj.bac_2,
			bac_3: obj.bac_3,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BoostValues {
	pub bs_v: i64 /* 15000 */,
	pub pos: Vec<i64> /* [2,1] */,
}

impl From<server::BoostValues> for BoostValues {
	fn from(obj: server::BoostValues) -> Self {
		BoostValues {
			bs_v: obj.bs_v,
			pos: obj.pos,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CollectValues {
	pub bs_v: f64 /* 117000.0 */,
	pub pos: Vec<i64> /* [4,1] */,
}

impl From<server::CollectValues> for CollectValues {
	fn from(obj: server::CollectValues) -> Self {
		CollectValues {
			bs_v: obj.bs_v,
			pos: obj.pos,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MysteryValues {
	pub bs_v: BonusBsVEnum /* mini */,
	pub id: i64 /* 12, 13, 14 */,
	pub pos: Vec<i64> /* [1,0] */,
}

impl From<server::MysteryValues> for MysteryValues {
	fn from(obj: server::MysteryValues) -> Self {
		MysteryValues {
			bs_v: obj.bs_v.into(),
			id: obj.id,
			pos: obj.pos,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bonus {
	pub bac: Bac,
	pub back_to: String /* spins */,
	pub bet_per_line: i64 /* 1, 150 */,
	pub board: Vec<Vec<i64>> /* [[14,3,3],[1,14,1],[2,11,3],[14,14,8],[4,13,14]] */,
	pub bonus_mechanic: Vec<i64> /* [1,2,3], [1,2], [1,3], [1], [2,3], [2], [3] */,
	pub bonus_scenario: i64 /* 0, 1, 2 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub boost_values: Option<Vec<BoostValues>>,
	pub bs_count: i64 /* 6, 7, 8, 9, 10, 11, 12, 13, 15 */,
	pub bs_v: Vec<Vec<BonusBsVEnum>> /* [[19500.0,0,0],[0,19500.0,0],[0,15000,0],[19500.0,19500.0,0],[0,117000.0,24000]] */,
	pub bs_values: Vec<Vec<f64>> /* [[6.5,0,0],[0,6.5,0],[0,5,0],[6.5,6.5,0],[0,39.0,8]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub collect_values: Option<Vec<CollectValues>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub copy_new_bs: Option<Vec<Vec<i64>>> /* [[3,0],[0,0],[3,1],[4,2],[1,1],[2,1],[4,1]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub double_values: Option<Vec<CollectValues>>,
	pub is_lucky_spin: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jackpot_values: Option<Vec<i64>> /* [300,600,2000], [45000,90000,300000] */,
	pub last_respin: bool /* false */,
	pub lines: i64 /* 25 */,
	pub mystery_count: i64 /* 0, 1, 2, 4, 5 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mystery_pos: Option<Vec<Vec<i64>>> /* [[1,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mystery_values: Option<Vec<MysteryValues>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub new_bs: Option<Vec<Vec<i64>>> /* [[3,0],[0,0],[3,1],[4,2],[1,1]] */,
	pub orig_board: Vec<Vec<i64>> /* [[3,3,3],[1,1,1],[2,11,3],[5,8,8],[4,13,5]] */,
	pub orig_bs_v: Vec<Vec<BonusBsVEnum>> /* [[4500.0,0,0],[0,4500.0,0],[0,15000,0],[4500.0,4500.0,0],[0,0,9000]] */,
	pub round_bet: i64 /* 20, 3000 */,
	pub round_win: i64 /* 0 */,
	pub rounds_granted: i64 /* 3 */,
	pub rounds_left: i64 /* 0, 1, 2, 3 */,
	pub total_win: i64 /* 0 */,
}

impl From<server::Bonus> for Bonus {
	fn from(obj: server::Bonus) -> Self {
		Bonus {
			bac: obj.bac.into(),
			back_to: obj.back_to,
			bet_per_line: obj.bet_per_line,
			board: obj.board,
			bonus_mechanic: obj.bonus_mechanic,
			bonus_scenario: obj.bonus_scenario,
			boost_values: obj.boost_values.map(|vec| vec.into_iter().map(Into::into).collect()),
			bs_count: obj.bs_count,
			bs_v: obj.bs_v.into_iter().map(|inner_vec| {inner_vec.into_iter().map(Into::into).collect()}).collect(),
			bs_values: obj.bs_values,
			collect_values: obj.collect_values.map(|vec| vec.into_iter().map(Into::into).collect()),
			copy_new_bs: obj.copy_new_bs,
			double_values: obj.double_values.map(|vec| vec.into_iter().map(Into::into).collect()),
			is_lucky_spin: obj.is_lucky_spin,
			jackpot_values: obj.jackpot_values,
			last_respin: obj.last_respin,
			lines: obj.lines,
			mystery_count: obj.mystery_count,
			mystery_pos: obj.mystery_pos,
			mystery_values: obj.mystery_values.map(|vec| vec.into_iter().map(Into::into).collect()),
			new_bs: obj.new_bs,
			orig_board: obj.orig_board,
			orig_bs_v: obj.orig_bs_v.into_iter().map(|inner_vec| {inner_vec.into_iter().map(Into::into).collect()}).collect(),
			round_bet: obj.round_bet,
			round_win: obj.round_win,
			rounds_granted: obj.rounds_granted,
			rounds_left: obj.rounds_left,
			total_win: obj.total_win,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LastArgs {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_factor: Option<i64> /* 20 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_per_line: Option<i64> /* 1, 150 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lines: Option<i64> /* 25 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<LastargsSelectedModeEnum> /* 1, 2 */,
}

impl From<server::LastArgs> for LastArgs {
	fn from(obj: server::LastArgs) -> Self {
		LastArgs {
			bet_factor: obj.bet_factor,
			bet_per_line: obj.bet_per_line,
			lines: obj.lines,
			selected_mode: obj.selected_mode.map(Into::into),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Winlines {
	pub amount: i64 /* 3000 */,
	pub line: i64 /* 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25 */,
	pub occurrences: i64 /* 3, 4, 5 */,
	pub positions: Vec<Vec<i64>> /* [[0,1],[1,1],[2,1],[3,1],[4,1]] */,
	pub symbol: i64 /* 1, 2, 3, 4, 5, 6, 7, 8, 9 */,
	#[serde(rename = "type")]
	pub winlines_type: String /* lb */,
}

impl From<server::Winlines> for Winlines {
	fn from(obj: server::Winlines) -> Self {
		Winlines {
			amount: obj.amount,
			line: obj.line,
			occurrences: obj.occurrences,
			positions: obj.positions,
			symbol: obj.symbol,
			winlines_type: obj.winlines_type,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Spins {
	pub bac: Bac,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bac_win: Option<bool> /* false, true */,
	pub bet_per_line: i64 /* 1, 12, 15, 150 */,
	pub board: Vec<Vec<i64>> /* [[2,2,1],[3,6,6],[9,9,9],[2,8,8],[5,2,2]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus_mechanic: Option<Vec<i64>> /* [1,2,3], [1,2], [1,3], [1], [2,3], [2], [3] */,
	pub bs_count: i64 /* 0 */,
	pub is_lucky_spin: bool /* false */,
	pub lines: i64 /* 25 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub paid: Option<bool> /* false, true */,
	pub round_bet: i64 /* 20, 240, 300, 3000 */,
	pub round_win: i64 /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<LastargsSelectedModeEnum> /* 1, 2 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub spin_type: Option<i64> /* 0, 1, 2, 3, 4, 5, 6, 10, 11, 12, 13, 15 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub total_win: Option<i64> /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub winlines: Option<Vec<Winlines>>,
}

impl From<server::Spins> for Spins {
	fn from(obj: server::Spins) -> Self {
		Spins {
			bac: obj.bac.into(),
			bac_win: obj.bac_win,
			bet_per_line: obj.bet_per_line,
			board: obj.board,
			bonus_mechanic: obj.bonus_mechanic,
			bs_count: obj.bs_count,
			is_lucky_spin: obj.is_lucky_spin,
			lines: obj.lines,
			paid: obj.paid,
			round_bet: obj.round_bet,
			round_win: obj.round_win,
			selected_mode: obj.selected_mode.map(Into::into),
			spin_type: obj.spin_type,
			total_win: obj.total_win,
			winlines: obj.winlines.map(|vec| vec.into_iter().map(Into::into).collect()),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Context {
	pub actions: Vec<ContextActionsEnum> /* bonus_init, bonus_spins_stop, buy_spin, respin, spin */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus: Option<Bonus>,
	pub current: ContextCurrentEnum /* bonus, spins */,
	pub last_action: ContextLastActionEnum /* bonus_init, bonus_spins_stop, buy_spin, init, respin, spin */,
	pub last_args: LastArgs,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_win: Option<i64> /* 24000 */,
	pub round_finished: bool /* false, true */,
	pub spins: Spins,
	pub version: i64 /* 1 */,
}

impl From<server::Context> for Context {
	fn from(obj: server::Context) -> Self {
		Context {
			actions: obj.actions.into_iter().map(Into::into).collect(),
			bonus: obj.bonus.map(Into::into),
			current: obj.current.into(),
			last_action: obj.last_action.into(),
			last_args: obj.last_args.into(),
			last_win: obj.last_win,
			round_finished: obj.round_finished,
			spins: obj.spins.into(),
			version: obj.version,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CurrencyFormat {
	pub currency_style: String /* symbol */,
	pub denominator: i64 /* 100 */,
	pub style: String /* money */,
}

impl From<server::CurrencyFormat> for CurrencyFormat {
	fn from(obj: server::CurrencyFormat) -> Self {
		CurrencyFormat {
			currency_style: obj.currency_style,
			denominator: obj.denominator,
			style: obj.style,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Jackpots {
	pub grand: i64 /* 5000 */,
	pub major: i64 /* 100 */,
	pub mini: i64 /* 15 */,
	pub minor: i64 /* 30 */,
}

impl From<server::Jackpots> for Jackpots {
	fn from(obj: server::Jackpots) -> Self {
		Jackpots {
			grand: obj.grand,
			major: obj.major,
			mini: obj.mini,
			minor: obj.minor,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PaytableElem {
	pub multiplier: i64 /* 2, 4, 20 */,
	pub occurrences: i64 /* 3, 4, 5 */,
	#[serde(rename = "type")]
	pub paytable_elem_type: String /* lb */,
}

impl From<server::PaytableElem> for PaytableElem {
	fn from(obj: server::PaytableElem) -> Self {
		PaytableElem {
			multiplier: obj.multiplier,
			occurrences: obj.occurrences,
			paytable_elem_type: obj.paytable_elem_type,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Paytable {
	#[serde(rename = "1")]
	pub paytable_1: Vec<PaytableElem>,
	#[serde(rename = "2")]
	pub paytable_2: Vec<PaytableElem>,
	#[serde(rename = "3")]
	pub paytable_3: Vec<PaytableElem>,
	#[serde(rename = "4")]
	pub paytable_4: Vec<PaytableElem>,
	#[serde(rename = "5")]
	pub paytable_5: Vec<PaytableElem>,
	#[serde(rename = "6")]
	pub paytable_6: Vec<PaytableElem>,
	#[serde(rename = "7")]
	pub paytable_7: Vec<PaytableElem>,
	#[serde(rename = "8")]
	pub paytable_8: Vec<PaytableElem>,
	#[serde(rename = "9")]
	pub paytable_9: Vec<PaytableElem>,
}

impl From<server::Paytable> for Paytable {
	fn from(obj: server::Paytable) -> Self {
		Paytable {
			paytable_1: obj.paytable_1.into_iter().map(Into::into).collect(),
			paytable_2: obj.paytable_2.into_iter().map(Into::into).collect(),
			paytable_3: obj.paytable_3.into_iter().map(Into::into).collect(),
			paytable_4: obj.paytable_4.into_iter().map(Into::into).collect(),
			paytable_5: obj.paytable_5.into_iter().map(Into::into).collect(),
			paytable_6: obj.paytable_6.into_iter().map(Into::into).collect(),
			paytable_7: obj.paytable_7.into_iter().map(Into::into).collect(),
			paytable_8: obj.paytable_8.into_iter().map(Into::into).collect(),
			paytable_9: obj.paytable_9.into_iter().map(Into::into).collect(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Reelsamples {
	pub buy_feature: Vec<Vec<i64>> /* [[1,2,3,4,5,6,7,8,9,11,12,13],[1,2,3,4,5,6,7,8,9,11,12,13],[1,2,3,4,5,6,7,8,9,11,12,13],[1,2,3,4,5,6,7,8,9,11,12,13],[1,2,3,4,5,6,7,8,9,11,12,13]] */,
	pub spins: Vec<Vec<i64>> /* [[1,2,3,4,5,6,7,8,9,11,12,13],[1,2,3,4,5,6,7,8,9,10,11,12,13],[1,2,3,4,5,6,7,8,9,10,11,12,13],[1,2,3,4,5,6,7,8,9,10,11,12,13],[1,2,3,4,5,6,7,8,9,10,11,12,13]] */,
	pub spins_0: Vec<Vec<i64>> /* [[1,2,3,4,5,6,7,8,9],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_1: Vec<Vec<i64>> /* [[1,5,9],[2,10,6,7],[8,3,4],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_10: Vec<Vec<i64>> /* [[4],[10,4],[10,4],[10,4],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_11: Vec<Vec<i64>> /* [[5],[10,5],[10,5],[10,5],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_12: Vec<Vec<i64>> /* [[3,4,5,6,7,8,9],[1,2,3,6,8,9,10],[1,2,4,5,6,7,10],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_13: Vec<Vec<i64>> /* [[1,4,5,6,7,8,9],[1,2,3,4,7,9,10],[2,3,5,6,7,8,10],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_14: Vec<Vec<i64>> /* [[1,2,5,6,7,8,9],[1,2,3,4,5,8,10],[3,4,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_15: Vec<Vec<i64>> /* [[1,2,3,6,7,8,9],[2,3,4,5,6,9,10],[1,4,5,7,8,9,10],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_2: Vec<Vec<i64>> /* [[1,5,9],[2,6,7],[8,10,3,4],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_3: Vec<Vec<i64>> /* [[8,3,6],[1,10,4,9],[2,5,7],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_4: Vec<Vec<i64>> /* [[8,3,6],[1,4,9],[2,10,5,7],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_5: Vec<Vec<i64>> /* [[2,4,7],[8,10,3,5],[1,9,6],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_6: Vec<Vec<i64>> /* [[2,4,7],[8,3,5],[1,10,9,6],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_7: Vec<Vec<i64>> /* [[1],[1,10],[1,10],[1,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_8: Vec<Vec<i64>> /* [[2],[2,10],[2,10],[2,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_9: Vec<Vec<i64>> /* [[3],[10,3],[10,3],[10,3],[1,2,3,4,5,6,7,8,9,10]] */,
}

impl From<server::Reelsamples> for Reelsamples {
	fn from(obj: server::Reelsamples) -> Self {
		Reelsamples {
			buy_feature: obj.buy_feature,
			spins: obj.spins,
			spins_0: obj.spins_0,
			spins_1: obj.spins_1,
			spins_10: obj.spins_10,
			spins_11: obj.spins_11,
			spins_12: obj.spins_12,
			spins_13: obj.spins_13,
			spins_14: obj.spins_14,
			spins_15: obj.spins_15,
			spins_2: obj.spins_2,
			spins_3: obj.spins_3,
			spins_4: obj.spins_4,
			spins_5: obj.spins_5,
			spins_6: obj.spins_6,
			spins_7: obj.spins_7,
			spins_8: obj.spins_8,
			spins_9: obj.spins_9,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Symbols {
	pub id: i64 /* 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 */,
	pub name: String /* el_01, el_02, el_03, el_04, el_05, el_06, el_07, el_08, el_09, el_bonus, el_bonus_feature_1, el_bonus_feature_2, el_bonus_feature_3, el_bonus_mystery, el_wild */,
	#[serde(rename = "type")]
	pub symbols_type: SymbolsTypeEnum /* line, scat, wild */,
}

impl From<server::Symbols> for Symbols {
	fn from(obj: server::Symbols) -> Self {
		Symbols {
			id: obj.id,
			name: obj.name,
			symbols_type: obj.symbols_type.into(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Settings {
	pub bet_factor: Vec<i64> /* [20] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bets: Option<Vec<i64>> /* [1,2,3,4,5,6,7,8,10,12,15,20,25,30,35,40,45,50,75,100,125,150], [1,2,3,4,5,6,7,9,10,15,16,20,25,30,35,50,55,75,80,100,125] */,
	pub big_win: Vec<i64> /* [15,20,30,50] */,
	pub bonus_symbol_v: Vec<BonusBsVEnum> /* [0.5,1,1.5,2,3,5,6,8,10,"mini","minor","major"] */,
	pub buy_bonus_price_1: i64 /* 100 */,
	pub buy_bonus_price_2: i64 /* 300 */,
	pub cols: i64 /* 5 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub currency_format: Option<CurrencyFormat>,
	pub jackpots: Jackpots,
	pub lines: Vec<i64> /* [25] */,
	pub payer_values: Vec<i64> /* [1,2,3,5,6,8,10,15,30,100] */,
	pub paylines: Vec<Vec<i64>> /* [[1,1,1,1,1],[0,0,0,0,0],[2,2,2,2,2],[0,1,2,1,0],[2,1,0,1,2],[1,0,0,0,1],[1,2,2,2,1],[0,0,1,2,2],[2,2,1,0,0],[1,2,1,0,1],[1,0,1,2,1],[0,1,1,1,0],[2,1,1,1,2],[0,1,0,1,0],[2,1,2,1,2],[1,1,0,1,1],[1,1,2,1,1],[0,0,2,0,0],[2,2,0,2,2],[0,2,2,2,0],[2,0,0,0,2],[1,2,0,2,1],[1,0,2,0,1],[0,2,0,2,0],[2,0,2,0,2]] */,
	pub paytable: Paytable,
	pub reelsamples: Reelsamples,
	pub respins_granted: i64 /* 3 */,
	pub rows: i64 /* 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rtp: Option<String> /* 95.70% */,
	pub symbols: Vec<Symbols>,
	pub symbols_line: Vec<i64> /* [1,2,3,4,5,6,7,8,9] */,
	pub symbols_scat: Vec<i64> /* [11,12,13,14,15] */,
	pub symbols_wild: Vec<i64> /* [10] */,
}

impl From<server::Settings> for Settings {
	fn from(obj: server::Settings) -> Self {
		Settings {
			bet_factor: obj.bet_factor,
			bets: obj.bets,
			big_win: obj.big_win,
			bonus_symbol_v: obj.bonus_symbol_v.into_iter().map(Into::into).collect(),
			buy_bonus_price_1: obj.buy_bonus_price_1,
			buy_bonus_price_2: obj.buy_bonus_price_2,
			cols: obj.cols,
			currency_format: obj.currency_format.map(Into::into),
			jackpots: obj.jackpots.into(),
			lines: obj.lines,
			payer_values: obj.payer_values,
			paylines: obj.paylines,
			paytable: obj.paytable.into(),
			reelsamples: obj.reelsamples.into(),
			respins_granted: obj.respins_granted,
			rows: obj.rows,
			rtp: obj.rtp,
			symbols: obj.symbols.into_iter().map(Into::into).collect(),
			symbols_line: obj.symbols_line,
			symbols_scat: obj.symbols_scat,
			symbols_wild: obj.symbols_wild,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Status {
	pub code: StatusCodeEnum /* GAME_REOPENED, OK, SESSION_LOST */,
	#[serde(rename = "type", skip_serializing_if = "Option::is_none")]
	pub status_type: Option<StatusTypeEnum> /* crit */,
}

impl From<server::Status> for Status {
	fn from(obj: server::Status) -> Self {
		Status {
			code: obj.code.into(),
			status_type: obj.status_type,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
	pub balance: i64 /* 100000 */,
	pub balance_version: i64 /* 2 */,
	pub currency: String /* FUN */,
	pub huid: String /* "demo-88fb2ae9a9fb434c9407a6322c941377" */,
	pub show_balance: bool /* true */,
}

impl From<server::User> for User {
	fn from(obj: server::User) -> Self {
		User {
			balance: obj.balance,
			balance_version: obj.balance_version,
			currency: obj.currency,
			huid: obj.huid,
			show_balance: obj.show_balance,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Start {
	pub command: ServerCommandEnum /* start */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub context: Option<Context>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modes: Option<Vec<ServerModesEnum>> /* auto, freebet, play */,
	pub request_id: String /* "b6d6863b-8c46-45bf-b000-1c45f35a6a3c" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roundnum: Option<String> /* "2505161000003735610" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session_id: Option<String> /* "54d657fdddea4c76800b216371ea868e" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub settings: Option<Settings>,
	pub status: Status,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<User>,
}

impl From<server::Server> for Start {
	fn from(obj: server::Server) -> Self {
		Start {
			command: obj.command,
			context: obj.context.map(Into::into),
			modes: obj.modes.map(|vec| vec.into_iter().map(Into::into).collect()),
			request_id: obj.request_id,
			roundnum: obj.roundnum,
			session_id: obj.session_id,
			settings: obj.settings.map(Into::into),
			status: obj.status.into(),
			user: obj.user.map(Into::into),
		}
	}
}

