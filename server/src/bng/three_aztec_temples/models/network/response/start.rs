use super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::enums::{ActionsEnum, MultiValueEnum, BonusModesEnum, CurrentActionsEnum, ModesEnum, SymbolsTypesEnum, StatusCodesEnum, CommandsEnum, StatusTypesEnum, CurrenciesEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bac {
	#[serde(rename = "1")]
	pub bac_1: Vec<i64> /* [1,0], [2,0], [2,1], [3,0], [3,1], [3,2], [4,0], [4,1], [4,2], [4,3], [5,0], [5,1], [5,2], [5,3], [6,0], [6,1], [6,2], [6,3], [7,0], [7,1], [7,2], [7,3], [7,4], [8,0], [8,1], [8,2], [8,3], [8,4], [9,0] */,
	#[serde(rename = "2")]
	pub bac_2: Vec<i64> /* [1,0], [2,0], [2,1], [3,0], [3,1], [3,2], [4,0], [4,1], [4,2], [4,3], [5,0], [5,1], [5,2], [5,3], [6,0], [6,1], [6,2], [7,0], [7,1], [7,2], [7,3], [7,4], [8,0], [8,1], [8,3], [9,0] */,
	#[serde(rename = "3")]
	pub bac_3: Vec<i64> /* [1,0], [2,0], [2,1], [3,0], [3,1], [3,2], [4,0], [4,1], [4,2], [4,3], [5,0], [5,1], [5,2], [5,3], [6,0], [6,1], [6,2], [6,3], [7,0], [7,1], [7,2], [7,3], [7,4], [8,0], [8,1], [8,2], [8,3], [8,4], [9,0] */,
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
	pub bs_v: MultiValueEnum /* 40 */,
	pub pos: Vec<i64> /* [3,2] */,
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
	pub bs_v: MultiValueEnum /* 120.0 */,
	pub pos: Vec<i64> /* [3,1] */,
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
pub struct MultiValues {
	pub bs_v: MultiValueEnum /* 100 */,
	pub mult_value: i64 /* 2, 5 */,
	pub pos: Vec<i64> /* [3,2] */,
}

impl From<server::MultiValues> for MultiValues {
	fn from(obj: server::MultiValues) -> Self {
		MultiValues {
			bs_v: obj.bs_v,
			mult_value: obj.mult_value,
			pos: obj.pos,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MysteryValues {
	pub bs_v: MultiValueEnum /* 0 */,
	pub id: i64 /* 11, 12, 13 */,
	pub pos: Vec<i64> /* [2,1] */,
}

impl From<server::MysteryValues> for MysteryValues {
	fn from(obj: server::MysteryValues) -> Self {
		MysteryValues {
			bs_v: obj.bs_v,
			id: obj.id,
			pos: obj.pos,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bonus {
	pub bac: Bac,
	pub back_to: CurrentActionsEnum /* spins */,
	pub bet_per_line: i64 /* 1 */,
	pub board: Vec<Vec<i64>> /* [[10,7,7],[7,7,4],[5,10,10],[4,12,4],[10,10,1]] */,
	pub bonus_game_type: i64 /* 1, 2, 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus_mechanic: Option<Vec<i64>> /* [1,2,3], [1,2], [1,3], [1], [2,3], [2], [3] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub boost_values: Option<Vec<BoostValues>>,
	pub bs_count: i64 /* 6, 7, 8, 9, 10, 11, 12, 13, 14 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bs_pos: Option<Vec<Vec<i64>>> /* [[0,0],[2,1],[2,2],[3,1],[4,0],[4,1]] */,
	pub bs_v: Vec<Vec<MultiValueEnum>> /* [[10.0,0,0],[0,0,0],[0,20,50.0],[0,120.0,0],[30.0,10.0,0]] */,
	pub bs_values: Vec<Vec<MultiValueEnum>> /* [[0.5,0,0],[0,0,0],[0,1,2.5],[0,6.0,0],[1.5,0.5,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub collect_values: Option<Vec<CollectValues>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub init_bs_count: Option<bool> /* true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jackpot_positions: Option<Vec<Vec<MultiValueEnum>>> /* [[0,0,0],[0,0,0],[0,0,0],[0,0,"major"],[0,0,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jackpot_values: Option<Vec<i64>> /* [200,400,2000] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jackpots_boost_values: Option<Vec<Vec<i64>>> /* [[0,0,0],[0,0,0],[160,0,0],[0,0,0],[0,0,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jackpots_multiplier_values: Option<Vec<Vec<i64>>> /* [[0,0,0],[0,0,0],[4,0,0],[0,0,0],[0,0,0]] */,
	pub last_respin: bool /* false */,
	pub lines: i64 /* 25 */,
	pub lucky_spin_win: bool /* false, true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub multi_values: Option<Vec<MultiValues>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mystery_values: Option<Vec<MysteryValues>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub new_bs: Option<Vec<Vec<i64>>> /* [[3,2]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_board: Option<Vec<Vec<i64>>> /* [[5,5,1],[2,10,10],[1,14,11],[1,10,13],[6,6,10]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_bs_v: Option<Vec<Vec<MultiValueEnum>>> /* [[10.0,0,0],[0,0,0],[0,20,50.0],[0,0,0],[30.0,10.0,0]] */,
	pub round_bet: i64 /* 20 */,
	pub round_win: i64 /* 0 */,
	pub rounds_granted: i64 /* 3 */,
	pub rounds_left: i64 /* 0, 1, 2, 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<BonusModesEnum> /* 1, 2 */,
	pub total_win: i64 /* 0 */,
}

impl From<server::Bonus> for Bonus {
	fn from(obj: server::Bonus) -> Self {
		Bonus {
			bac: obj.bac.into(),
			back_to: obj.back_to,
			bet_per_line: obj.bet_per_line,
			board: obj.board,
			bonus_game_type: obj.bonus_game_type,
			bonus_mechanic: obj.bonus_mechanic,
			boost_values: obj.boost_values.map(|vec| vec.into_iter().map(Into::into).collect()),
			bs_count: obj.bs_count,
			bs_pos: obj.bs_pos,
			bs_v: obj.bs_v.into_iter().map(|inner_vec| {inner_vec.into_iter().map(Into::into).collect()}).collect(),
			bs_values: obj.bs_values,
			collect_values: obj.collect_values.map(|vec| vec.into_iter().map(Into::into).collect()),
			init_bs_count: obj.init_bs_count,
			jackpot_positions: obj.jackpot_positions.map(|vec| vec.into_iter().map(Into::into).collect()),
			jackpot_values: obj.jackpot_values,
			jackpots_boost_values: obj.jackpots_boost_values,
			jackpots_multiplier_values: obj.jackpots_multiplier_values,
			last_respin: obj.last_respin,
			lines: obj.lines,
			lucky_spin_win: obj.lucky_spin_win,
			multi_values: obj.multi_values.map(|vec| vec.into_iter().map(Into::into).collect()),
			mystery_values: obj.mystery_values.map(|vec| vec.into_iter().map(Into::into).collect()),
			new_bs: obj.new_bs,
			origin_board: obj.origin_board,
			origin_bs_v: obj.origin_bs_v.map(|vec| vec.into_iter().map(Into::into).collect()),
			round_bet: obj.round_bet,
			round_win: obj.round_win,
			rounds_granted: obj.rounds_granted,
			rounds_left: obj.rounds_left,
			selected_mode: obj.selected_mode.map(Into::into),
			total_win: obj.total_win,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LastArgs {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_factor: Option<i64> /* 20 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_per_line: Option<i64> /* 1 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lines: Option<i64> /* 25 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<BonusModesEnum> /* 1, 2 */,
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
	pub amount: i64 /* 5 */,
	pub line: i64 /* 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25 */,
	pub occurrences: i64 /* 3, 4, 5 */,
	pub positions: Vec<Vec<i64>> /* [[0,1],[1,1],[2,1]] */,
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
	pub bac_pos: Option<Vec<Vec<i64>>> /* [[4,1],[4,0],[3,0],[2,0],[0,2]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bac_win: Option<bool> /* false, true */,
	pub bet_per_line: i64 /* 1, 35 */,
	pub board: Vec<Vec<i64>> /* [[6,6,13],[1,12,7],[2,2,11],[10,10,10],[9,9,9]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus_mechanic: Option<Vec<i64>> /* [1,2,3], [1,2], [1,3], [1], [2,3], [2], [3] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bs_count: Option<i64> /* 6 */,
	pub bs_v: Vec<Vec<MultiValueEnum>> /* [[0,0,0],[0,0,0],[0,0,0],["major",1400,5600],[0,0,0]] */,
	pub bs_values: Vec<Vec<MultiValueEnum>> /* [[0,0,0],[0,0,0],[0,0,0],[100,2,8],[0,0,0]] */,
	pub lines: i64 /* 25 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lucky_spin_win: Option<bool> /* false, true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_board: Option<Vec<Vec<i64>>> /* [[5,5,1],[4,4,13],[9,4,4],[4,4,4],[6,4,4]] */,
	pub round_bet: i64 /* 20, 700 */,
	pub round_win: i64 /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<BonusModesEnum> /* 1, 2 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub total_win: Option<i64> /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub winlines: Option<Vec<Winlines>>,
}

impl From<server::Spins> for Spins {
	fn from(obj: server::Spins) -> Self {
		Spins {
			bac: obj.bac.into(),
			bac_pos: obj.bac_pos,
			bac_win: obj.bac_win,
			bet_per_line: obj.bet_per_line,
			board: obj.board,
			bonus_mechanic: obj.bonus_mechanic,
			bs_count: obj.bs_count,
			bs_v: obj.bs_v.into_iter().map(|inner_vec| {inner_vec.into_iter().map(Into::into).collect()}).collect(),
			bs_values: obj.bs_values,
			lines: obj.lines,
			lucky_spin_win: obj.lucky_spin_win,
			origin_board: obj.origin_board,
			round_bet: obj.round_bet,
			round_win: obj.round_win,
			selected_mode: obj.selected_mode.map(Into::into),
			total_win: obj.total_win,
			winlines: obj.winlines.map(|vec| vec.into_iter().map(Into::into).collect()),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Context {
	pub actions: Vec<ActionsEnum> /* bonus_init, bonus_spins_stop, buy_spin, respin, spin */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus: Option<Bonus>,
	pub current: CurrentActionsEnum /* bonus, spins */,
	pub last_action: ActionsEnum /* bonus_init, bonus_spins_stop, buy_spin, init, respin, spin */,
	pub last_args: LastArgs,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_win: Option<i64> /* 80 */,
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
	pub grand: i64 /* 2000 */,
	pub major: i64 /* 100 */,
	pub mini: i64 /* 10 */,
	pub minor: i64 /* 20 */,
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
	pub multiplier: i64 /* 5, 10, 50 */,
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
	pub spins: Vec<Vec<i64>> /* [[1,2,3,4,5,6,7,8,9,10,11,12,13],[1,2,3,4,5,6,7,8,9,10,11,12,13],[1,2,3,4,5,6,7,8,9,10,11,12,13],[1,2,3,4,5,6,7,8,9,10,11,12,13],[1,2,3,4,5,6,7,8,9,10,11,12,13]] */,
}

impl From<server::Reelsamples> for Reelsamples {
	fn from(obj: server::Reelsamples) -> Self {
		Reelsamples {
			spins: obj.spins,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Symbols {
	pub id: i64 /* 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14 */,
	pub name: String /* el_01, el_02, el_03, el_04, el_05, el_06, el_07, el_08, el_bonus, el_bonus_boost, el_bonus_collect, el_bonus_multi, el_bonus_mystery, el_wild */,
	#[serde(rename = "type")]
	pub symbols_type: SymbolsTypesEnum /* line, scat, wild */,
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
	pub bets: Option<Vec<i64>> /* [1,2,3,4,5,8,10,15,25,35,50,75,100,150,200,250,300,350,400,450,500] */,
	pub big_win: Vec<i64> /* [15,25,50,80] */,
	pub bonus_symbols: Vec<MultiValueEnum> /* 10.0, 100.0, 1000.0, 1020.0, 1060.0, 1080.0, 1090.0, 110.0, 1100.0, 1140.0, 1180.0, 120.0, 1200.0, 1240.0, 1250.0, 130.0, 1300.0, 1310.0, 1320.0, 1340.0, 1370.0, 140.0, 1440.0, 1480.0, 150.0, 1560.0, 1580.0, 160.0, 1600.0, 1620.0, 1670.0, 1680.0, 170.0, 180.0, 190.0, 1920.0, 1960.0, 20.0, 200.0, 2000.0, 2050.0, 210.0, 2100.0, 2170.0, 220.0, 2200.0, 2220.0, 230.0, 2380.0, 240.0, 2400.0, 2440.0, 250.0, 2520.0, 260.0, 270.0, 2700.0, 2740.0, 280.0, 2870.0, 290.0, 2920.0, 2960.0, 30.0, 300.0, 320.0, 330.0, 3300.0, 340.0, 350.0, 3540.0, 360.0, 3600.0, 3700.0, 380.0, 3840.0, 390.0, 3900.0, 40.0, 400.0, 410.0, 420.0, 430.0, 440.0, 450.0, 460.0, 470.0, 480.0, 490.0, 50.0, 500.0, 510.0, 520.0, 5250.0, 530.0, 540.0, 560.0, 570.0, 580.0, 590.0, 60.0, 600.0, 610.0, 6160.0, 620.0, 630.0, 640.0, 650.0, 660.0, 670.0, 6860.0, 70.0, 700.0, 710.0, 720.0, 740.0, 750.0, 760.0, 780.0, 80.0, 800.0, 820.0, 840.0, 850.0, 880.0, 90.0, 900.0, 920.0, 930.0, 940.0, 980.0, major, mini, minor, 0, 20, 40, 60, 80, 100, 120, 140, 160, 180, 200, 220, 240, 260, 280, 300, 320, 340, 360, 380, 400, 420, 440, 460, 480, 500, 520, 540, 560, 580, 600, 620, 640, 660, 700, 720, 740, 800, 840, 1000, 1120, 1220, 1440, 1520, 1720, 1800, 1920, 2440, 2800, 4880 */,
	pub boost_symbols: Vec<i64> /* [2,3,4,5,6,7,8,10] */,
	pub buy_bonus_price: Vec<i64> /* [100,300] */,
	pub cols: i64 /* 5 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub currency_format: Option<CurrencyFormat>,
	pub default_multiplier: i64 /* 4 */,
	pub jackpots: Jackpots,
	pub lines: Vec<i64> /* [25] */,
	pub multi_symbols: Vec<i64> /* [2,3,4,5] */,
	pub multiplier_values: Vec<i64> /* [2,3,5] */,
	pub paylines: Vec<Vec<i64>> /* [[1,1,1,1,1],[0,0,0,0,0],[2,2,2,2,2],[0,1,2,1,0],[2,1,0,1,2],[1,0,0,0,1],[1,2,2,2,1],[0,0,1,2,2],[2,2,1,0,0],[1,2,1,0,1],[1,0,1,2,1],[0,1,1,1,0],[2,1,1,1,2],[0,1,0,1,0],[2,1,2,1,2],[1,1,0,1,1],[1,1,2,1,1],[0,0,2,0,0],[2,2,0,2,2],[0,2,2,2,0],[2,0,0,0,2],[1,2,0,2,1],[1,0,2,0,1],[0,2,0,2,0],[2,0,2,0,2]] */,
	pub paytable: Paytable,
	pub reelsamples: Reelsamples,
	pub respins_granted: i64 /* 3 */,
	pub rows: i64 /* 3 */,
	pub small_win: i64 /* 3 */,
	pub symbols: Vec<Symbols>,
	pub symbols_line: Vec<i64> /* [1,2,3,4,5,6,7,8] */,
	pub symbols_scat: Vec<i64> /* [10,11,12,13,14] */,
	pub symbols_wild: Vec<i64> /* [9] */,
}

impl From<server::Settings> for Settings {
	fn from(obj: server::Settings) -> Self {
		Settings {
			bet_factor: obj.bet_factor,
			bets: obj.bets,
			big_win: obj.big_win,
			bonus_symbols: obj.bonus_symbols.into_iter().map(Into::into).collect(),
			boost_symbols: obj.boost_symbols,
			buy_bonus_price: obj.buy_bonus_price,
			cols: obj.cols,
			currency_format: obj.currency_format.map(Into::into),
			default_multiplier: obj.default_multiplier,
			jackpots: obj.jackpots.into(),
			lines: obj.lines,
			multi_symbols: obj.multi_symbols,
			multiplier_values: obj.multiplier_values,
			paylines: obj.paylines,
			paytable: obj.paytable.into(),
			reelsamples: obj.reelsamples.into(),
			respins_granted: obj.respins_granted,
			rows: obj.rows,
			small_win: obj.small_win,
			symbols: obj.symbols.into_iter().map(Into::into).collect(),
			symbols_line: obj.symbols_line,
			symbols_scat: obj.symbols_scat,
			symbols_wild: obj.symbols_wild,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Status {
	pub code: StatusCodesEnum /* FUNDS_EXCEED, GAME_REOPENED, OK, PLAYER_DISCONNECTED */,
	#[serde(rename = "type", skip_serializing_if = "Option::is_none")]
	pub status_type: Option<StatusTypesEnum> /* crit, exceed */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub traceback: Option<String> /* NOT_ENOUGH_MONEY */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<i64> /* -1 */,
}

impl From<server::Status> for Status {
	fn from(obj: server::Status) -> Self {
		Status {
			code: obj.code,
			status_type: obj.status_type,
			traceback: obj.traceback,
			user_id: obj.user_id,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
	pub balance: i64 /* 100000 */,
	pub balance_version: i64 /* 2 */,
	pub currency: CurrenciesEnum /* FUN */,
	pub huid: String /* "demo-106758a99a3346fba872f844aa187a8c" */,
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
	pub command: CommandsEnum /* start */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub context: Option<Context>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modes: Option<Vec<ModesEnum>> /* auto, freebet, play */,
	pub request_id: String /* "7577bb90-9cbe-4f6e-bfaa-19a58c94e61a" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roundnum: Option<String> /* "2505171000004313102" */,
	pub session_id: String /* "04d1923972bc43a9a629302732728d65" */,
	pub settings: Settings,
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
			settings: obj.settings.into(),
			status: obj.status.into(),
			user: obj.user.map(Into::into),
		}
	}
}

