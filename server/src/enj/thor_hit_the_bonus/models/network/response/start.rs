use super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::enums::{ActionsEnum, MultiValueEnum, ModesEnum, StatusCodesEnum, CommandsEnum, BonusTypeEnum, BonusModesEnum, CurrentActionsEnum, StatusTypesEnum, SymbolsTypesEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bs {
	pub position: i64 /* 0, 1, 2, 3, 4 */,
	pub reel: i64 /* 0, 1, 2, 3, 4 */,
	#[serde(rename = "type")]
	pub bs_type: BonusTypeEnum /* collector, infinity, infinity_mystery, infinity_mystery_jp, infinity_wrath, infinity_wrath_sticky, mystery, mystery_jp, regular */,
	pub value: i64 /* 0, 200, 400, 600, 800, 1000, 1200, 1600, 1800, 2000, 2600, 3400, 4000, 4800, 5000, 7000, 10000, 11600, 12800, 13800, 19200 */,
}

impl From<server::Bs> for Bs {
	fn from(obj: server::Bs) -> Self {
		Bs {
			position: obj.position,
			reel: obj.reel,
			bs_type: obj.bs_type.into(),
			value: obj.value,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Drawer {
	pub thor_wrath_spins_left: i64 /* 0, 2 */,
}

impl From<server::Drawer> for Drawer {
	fn from(obj: server::Drawer) -> Self {
		Drawer {
			thor_wrath_spins_left: obj.thor_wrath_spins_left,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Winscatters {
	pub amount: i64 /* 18000 */,
	pub occurrences: i64 /* 11, 13, 14, 16, 17, 20 */,
	pub positions: Vec<Vec<i64>> /* [[0,0],[0,1],[0,2],[0,3],[0,4],[1,0],[1,1],[1,4],[2,2],[2,3],[2,4],[3,0],[3,1],[4,2],[4,3],[4,4]] */,
	pub symbol: i64 /* 1 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub trigger: Option<String>,
}

impl From<server::Winscatters> for Winscatters {
	fn from(obj: server::Winscatters) -> Self {
		Winscatters {
			amount: obj.amount,
			occurrences: obj.occurrences,
			positions: obj.positions,
			symbol: obj.symbol,
			trigger: obj.trigger,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bonus {
	pub bac: Vec<i64> /* [0,0], [0,10], [1,51], [1,75], [2,161], [3,244], [3,245], [3,287], [3,309], [4,350] */,
	pub back_to: String /* spins */,
	pub bet_per_line: i64 /* 20 */,
	pub bg_type: i64 /* 0, 1, 2, 3, 4, 6, 8, 9, 10 */,
	pub board: Vec<Vec<i64>> /* [[0,0,1,0,0],[0,0,5,0,0],[0,0,1,0,0],[0,0,12,0,0],[0,0,1,0,0]] */,
	pub bs: Vec<Bs>,
	pub bs_count: i64 /* 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22 */,
	pub bs_v: Vec<Vec<MultiValueEnum>> /* [[0,0,400,0,0],[0,0,1000,0,0],[0,0,400,0,0],[0,0,7000,0,0],[0,0,400,0,0]] */,
	pub bs_values: Vec<Vec<i64>> /* [[0,0,2,0,0],[0,0,5,0,0],[0,0,2,0,0],[0,0,35,0,0],[0,0,2,0,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub drawer: Option<Drawer>,
	pub is_ultra: bool /* false, true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_bs: Option<Vec<Bs>>,
	pub lines: i64 /* 1 */,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub mps: Vec<Vec<i32>> /* [[1,1,1,1,1],[1,1,1,1,1],[1,1,1,1,1],[1,1,1,1,1],[1,1,1,1,1]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mult_features: Option<Vec<i64>> /* [10], [12], [14], [3], [4], [5], [6], [7], [9] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mult_pos: Option<Vec<Vec<i64>>> /* [[2,3],[2,0],[2,4],[4,0],[0,2]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mysteries: Option<Vec<Bs>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub new_bs: Option<Vec<Bs>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub new_collectors: Option<Vec<Bs>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub orig_board: Option<Vec<Vec<i64>>> /* [[0,0,1,0,0],[0,0,5,0,0],[0,0,1,0,0],[0,0,12,0,0],[0,0,1,0,0]] */,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub orig_bs_v: Vec<Vec<MultiValueEnum>> /* [[0,0,1,0,0],[0,0,5,0,0],[0,0,1,0,0],[0,0,12,0,0],[0,0,1,0,0]] */,
	pub round_bet: i64 /* 200 */,
	pub round_win: i64 /* 0 */,
	pub rounds_granted: i64 /* 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 20, 22 */,
	pub rounds_left: i64 /* 0, 1, 2, 3 */,
	pub rounds_lefts: i64 /* 0, 1, 2, 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<BonusModesEnum> /* 1, 2, 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sticky_bs: Option<Vec<Bs>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sticky_bs_pos: Option<Vec<Vec<i64>>> /* [[1,2],[3,2]] */,
	pub total_win: i64 /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub winscatters: Option<Vec<Winscatters>>,
}

impl From<server::Bonus> for Bonus {
	fn from(obj: server::Bonus) -> Self {
		Bonus {
			bac: obj.bac,
			back_to: obj.back_to,
			bet_per_line: obj.bet_per_line,
			bg_type: obj.bg_type,
			board: obj.board,
			bs: obj.bs.into_iter().map(Into::into).collect(),
			bs_count: obj.bs_count,
			bs_v: obj.bs_v.into_iter().map(|inner_vec| {inner_vec.into_iter().map(Into::into).collect()}).collect(),
			bs_values: obj.bs_values,
			drawer: obj.drawer.map(Into::into),
			is_ultra: obj.is_ultra,
			last_bs: obj.last_bs.map(|vec| vec.into_iter().map(Into::into).collect()),
			lines: obj.lines,
			mps: obj.mps,
			mult_features: obj.mult_features,
			mult_pos: obj.mult_pos,
			mysteries: obj.mysteries.map(|vec| vec.into_iter().map(Into::into).collect()),
			new_bs: obj.new_bs.map(|vec| vec.into_iter().map(Into::into).collect()),
			new_collectors: obj.new_collectors.map(|vec| vec.into_iter().map(Into::into).collect()),
			orig_board: obj.orig_board,
			round_bet: obj.round_bet,
			round_win: obj.round_win,
			rounds_granted: obj.rounds_granted,
			rounds_left: obj.rounds_left,
			rounds_lefts: obj.rounds_lefts,
			selected_mode: obj.selected_mode.map(Into::into),
			sticky_bs: obj.sticky_bs.map(|vec| vec.into_iter().map(Into::into).collect()),
			sticky_bs_pos: obj.sticky_bs_pos,
			total_win: obj.total_win,
			winscatters: obj.winscatters.map(|vec| vec.into_iter().map(Into::into).collect()),
			orig_bs_v: obj.orig_bs_v.unwrap_or(vec![])
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bet20 {
	pub bs: Vec<Bs>,
}

impl From<server::Bet20> for Bet20 {
	fn from(obj: server::Bet20) -> Self {
		Bet20 {
			bs: obj.bs.into_iter().map(Into::into).collect(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Winlines {
	pub amount: i64 /* 0 */,
	pub line: i64 /* 1 */,
	pub occurrences: i64 /* 5 */,
	pub positions: Vec<Vec<i64>> /* [[0,2],[1,2],[2,2],[3,2],[4,2]] */,
	pub symbol: i64 /* 1 */,
	pub trigger: String /* bonus */,
}

impl From<server::Winlines> for Winlines {
	fn from(obj: server::Winlines) -> Self {
		Winlines {
			amount: obj.amount,
			line: obj.line,
			occurrences: obj.occurrences,
			positions: obj.positions,
			symbol: obj.symbol,
			trigger: obj.trigger,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Spins {
	pub bac: Vec<i64> /* [0,0], [0,10], [0,36], [0,43], [1,105], [1,108], [1,110], [1,130], [1,134], [1,137], [1,51], [1,54], [1,55], [1,62], [1,75], [1,85], [2,141], [2,155], [2,156], [2,161], [2,171], [2,189], [2,196], [2,202], [2,219], [2,222], [3,239], [3,244], [3,245], [3,257], [3,269], [3,271], [3,276], [3,287], [3,292], [3,309], [3,341], [4,350] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet20: Option<Bet20>,
	pub bet_per_line: i64 /* 20 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bg_type: Option<i64> /* 0, 1, 2, 3, 4, 6, 8, 9, 10 */,
	pub board: Vec<Vec<i64>> /* [[0,0,1,0,0],[0,1,0,7,0],[8,0,6,0,0],[0,1,0,1,1],[0,3,1,0,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bs: Option<Vec<Bs>>,
	pub bs_count: i64 /* 2, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 18 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bs_new: Option<Vec<Bs>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bs_new_count: Option<i64> /* 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bs_sticky_bet: Option<i64> /* 200 */,
	pub bs_v: Vec<Vec<MultiValueEnum>> /* [[0,0,600,0,0],[0,800,0,200,0],[2000,0,0,0,0],[0,400,0,800,200],[0,"minor",600,0,0]] */,
	pub bs_values: Vec<Vec<i64>> /* [[0,0,3,0,0],[0,4,0,1,0],[10,0,0,0,0],[0,2,0,4,1],[0,20,3,0,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub drawer: Option<Drawer>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub freezing_bs: Option<Vec<Vec<i64>>> /* [[1,2],[3,2]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_bought: Option<bool> /* false, true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_extra: Option<bool> /* false, true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_bet: Option<i64> /* 20 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_bs: Option<Vec<Bs>>,
	pub lines: i64 /* 1 */,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub mps: Vec<Vec<i32>> /* [[1,1,1,1,1],[1,1,1,1,1],[1,1,1,1,1],[1,1,1,1,1],[1,1,1,1,1]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mult_features: Option<Vec<i64>> /* [10], [7], [8] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mult_pos: Option<Vec<Vec<i64>>> /* [[3,3],[2,3],[0,3],[0,3],[0,2],[4,4],[1,2],[3,1],[1,2],[3,1]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub new_collectors: Option<Vec<Bs>>,
	pub round_bet: i64 /* 200 */,
	pub round_win: i64 /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<BonusModesEnum> /* 1, 2, 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sticky_bs: Option<Vec<Bs>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sticky_bs_pos: Option<Vec<Vec<i64>>> /* [[1,2],[3,2]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temp_board: Option<Vec<Vec<i64>>> /* [[0,0,1,0,0],[0,1,0,7,0],[8,0,6,0,0],[0,1,0,1,1],[0,3,1,0,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temp_bs_v: Option<Vec<Vec<MultiValueEnum>>> /* [[0,0,600,0,0],[0,800,0,200,0],[2000,0,0,0,0],[0,400,0,800,200],[0,"minor",600,0,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temp_bs_values: Option<Vec<Vec<i64>>> /* [[0,0,3,0,0],[0,4,0,1,0],[10,0,0,0,0],[0,2,0,4,1],[0,20,3,0,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub total_win: Option<i64> /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub winlines: Option<Vec<Winlines>>,
}

impl From<server::Spins> for Spins {
	fn from(obj: server::Spins) -> Self {
		Spins {
			bac: obj.bac,
			bet20: obj.bet20.map(Into::into),
			bet_per_line: obj.bet_per_line,
			bg_type: obj.bg_type,
			board: obj.board,
			bs: obj.bs.map(|vec| vec.into_iter().map(Into::into).collect()),
			bs_count: obj.bs_count,
			bs_new: obj.bs_new.map(|vec| vec.into_iter().map(Into::into).collect()),
			bs_new_count: obj.bs_new_count,
			bs_sticky_bet: obj.bs_sticky_bet,
			bs_v: obj.bs_v.into_iter().map(|inner_vec| {inner_vec.into_iter().map(Into::into).collect()}).collect(),
			bs_values: obj.bs_values,
			drawer: obj.drawer.map(Into::into),
			freezing_bs: obj.freezing_bs,
			is_bought: obj.is_bought,
			is_extra: obj.is_extra,
			last_bet: obj.last_bet,
			last_bs: obj.last_bs.map(|vec| vec.into_iter().map(Into::into).collect()),
			lines: obj.lines,
			mps: obj.mps,
			mult_features: obj.mult_features,
			mult_pos: obj.mult_pos,
			new_collectors: obj.new_collectors.map(|vec| vec.into_iter().map(Into::into).collect()),
			round_bet: obj.round_bet,
			round_win: obj.round_win,
			selected_mode: obj.selected_mode.map(Into::into),
			sticky_bs: obj.sticky_bs.map(|vec| vec.into_iter().map(Into::into).collect()),
			sticky_bs_pos: obj.sticky_bs_pos,
			temp_board: obj.temp_board,
			temp_bs_v: obj.temp_bs_v.map(|vec| vec.into_iter().map(Into::into).collect()),
			temp_bs_values: obj.temp_bs_values,
			total_win: obj.total_win,
			winlines: obj.winlines.map(|vec| vec.into_iter().map(Into::into).collect()),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Achievements {
	pub level: i64 /* 0, 1, 2, 3, 4 */,
	pub level_percent: f64 /* 0.011, 0.044, 0.056, 0.075, 0.117, 0.125, 0.133, 0.167, 0.178, 0.2, 0.225, 0.233, 0.278, 0.325, 0.342, 0.344, 0.383, 0.389, 0.475, 0.517, 0.544, 0.611, 0.622, 0.644, 0.658, 0.667, 0.689, 0.72, 0.86, 0.878, 0.889, 0.911, 0.925, 0.933, 0.967, 0, 1 */,
	pub number: i64 /* 0, 10, 36, 43, 51, 54, 55, 62, 75, 85, 105, 108, 110, 130, 134, 137, 141, 155, 156, 161, 171, 189, 196, 202, 219, 222, 239, 244, 245, 257, 269, 271, 276, 287, 292, 309, 341, 350 */,
	pub total_percent: f64 /* 0.029, 0.103, 0.123, 0.146, 0.154, 0.157, 0.177, 0.214, 0.243, 0.3, 0.309, 0.314, 0.371, 0.383, 0.391, 0.403, 0.443, 0.446, 0.46, 0.489, 0.54, 0.56, 0.577, 0.626, 0.634, 0.683, 0.697, 0.7, 0.734, 0.769, 0.774, 0.789, 0.82, 0.834, 0.883, 0.974, 0, 1 */,
}

impl From<server::Achievements> for Achievements {
	fn from(obj: server::Achievements) -> Self {
		Achievements {
			level: obj.level,
			level_percent: obj.level_percent,
			number: obj.number,
			total_percent: obj.total_percent,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bb {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub thor_state: Option<i64> /* 0, 1, 2 */,
	pub thor_wrath_spins_left: i64 /* 0, 1, 2 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub thor_wrath_symbol_pos: Option<(usize, usize)> /* [3,0] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub thor_wrath_total: Option<i64> /* 0, 5000, 11600 */,
}

impl From<server::Bb> for Bb {
	fn from(obj: server::Bb) -> Self {
		Bb {
			thor_state: obj.thor_state,
			thor_wrath_spins_left: obj.thor_wrath_spins_left,
			thor_wrath_symbol_pos: obj.thor_wrath_symbol_pos,
			thor_wrath_total: obj.thor_wrath_total,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LastArgs {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_factor: Option<i64> /* 10 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_per_line: Option<i64> /* 20 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lines: Option<i64> /* 1 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<BonusModesEnum> /* 1, 2, 3 */,
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
pub struct Context {
	pub achievements: Achievements,
	pub actions: Vec<ActionsEnum> /* bonus_init, bonus_spins_stop, buy_spin, respin, spin */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bb: Option<Bb>,
	pub bet20: Bb,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus: Option<Bonus>,
	pub current: CurrentActionsEnum /* bonus, spins */,
	pub last_action: ActionsEnum /* bonus_init, bonus_spins_stop, buy_spin, init, respin, spin */,
	pub last_args: LastArgs,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_win: Option<i64> /* 18000 */,
	pub round_finished: bool /* false, true */,
	pub spins: Spins,
	pub version: i64 /* 1 */,
}

impl From<server::Context> for Context {
	fn from(obj: server::Context) -> Self {
		Context {
			achievements: obj.achievements.into(),
			actions: obj.actions.into_iter().map(Into::into).collect(),
			bb: obj.bb.map(Into::into),
			bet20: obj.bet20.into(),
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
pub struct Data {
	pub quick_spin: bool /* false */,
}

impl From<server::Data> for Data {
	fn from(obj: server::Data) -> Self {
		Data {
			quick_spin: obj.quick_spin,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct OriginData {
	pub data: Data,
	pub quick_spin: bool /* false */,
}

impl From<server::OriginData> for OriginData {
	fn from(obj: server::OriginData) -> Self {
		OriginData {
			data: obj.data.into(),
			quick_spin: obj.quick_spin,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CurrencyFormat {
	pub currency_style: String /* code */,
	pub decimal_separator: Option<String> /* . */,
	pub denominator: i64 /* 100 */,
	pub style: String /* money */,
	pub thousands_separator: Option<String> /*  */,
}

impl From<server::CurrencyFormat> for CurrencyFormat {
	fn from(obj: server::CurrencyFormat) -> Self {
		CurrencyFormat {
			currency_style: obj.currency_style,
			decimal_separator: obj.decimal_separator,
			denominator: obj.denominator,
			style: obj.style,
			thousands_separator: obj.thousands_separator,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct FreespinsBuyingPrice {
	#[serde(rename = "1")]
	pub freespins_buying_price_1: i64 /* 50 */,
	#[serde(rename = "2")]
	pub freespins_buying_price_2: i64 /* 150 */,
	#[serde(rename = "3")]
	pub freespins_buying_price_3: i64 /* 400 */,
}

impl From<server::FreespinsBuyingPrice> for FreespinsBuyingPrice {
	fn from(obj: server::FreespinsBuyingPrice) -> Self {
		FreespinsBuyingPrice {
			freespins_buying_price_1: obj.freespins_buying_price_1,
			freespins_buying_price_2: obj.freespins_buying_price_2,
			freespins_buying_price_3: obj.freespins_buying_price_3,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Jackpots {
	pub grand: i64 /* 2000 */,
	pub major: i64 /* 50 */,
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
	pub multiplier: i64 /* 0 */,
	pub occurrences: i64 /* 4 */,
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
	#[serde(rename = "0")]
	pub paytable_0: Vec<PaytableElem>,
}

impl From<server::Paytable> for Paytable {
	fn from(obj: server::Paytable) -> Self {
		Paytable {
			paytable_0: obj.paytable_0.into_iter().map(Into::into).collect(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Symbols {
	pub id: i64 /* 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 */,
	pub name: String /* el_0, el_bonus, el_bonus_sticky, el_collect, el_jackpot_major, el_jackpot_mini, el_jackpot_minor, el_mystery, el_mystery_jackpot, el_sticky_mystery, el_sticky_mystery_jackpot, el_thunder_wrath, el_thunder_wrath_sticky */,
	#[serde(rename = "type")]
	pub symbols_type: SymbolsTypesEnum /* line, scat */,
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
	pub bet_factor: Vec<i64> /* [10] */,
	pub bets: Vec<i64> /* [1,2,3,4,5,8,10,15,20,30,40,50,75,100,200,300,500,750,1000] */,
	pub bonus_symbols_values: Vec<i64> /* [1,2,3,4] */,
	pub cols: i64 /* 5 */,
	pub currency_format: CurrencyFormat,
	pub default_multiplier: i64 /* 10 */,
	pub freespins_buying_price: FreespinsBuyingPrice,
	pub jackpots: Jackpots,
	pub lines: Vec<i64> /* [1] */,
	pub mystery_values: Vec<i64> /* [1,2,3,4,10,20,50] */,
	pub paylines: Vec<Vec<i64>> /* [[2,2,2,2,2]] */,
	pub paytable: Paytable,
	pub rows: i64 /* 5 */,
	pub rtp: String /* 95.75% */,
	pub sticky_values: Vec<i64> /* [5,6,7,8,9] */,
	pub symbols: Vec<Symbols>,
	pub symbols_line: Vec<i64> /* [0] */,
	pub symbols_scat: Vec<i64> /* [1,2,3,4,5,6,7,8,9,10,11,12] */,
}

impl From<server::Settings> for Settings {
	fn from(obj: server::Settings) -> Self {
		Settings {
			bet_factor: obj.bet_factor,
			bets: obj.bets,
			bonus_symbols_values: obj.bonus_symbols_values,
			cols: obj.cols,
			currency_format: obj.currency_format.into(),
			default_multiplier: obj.default_multiplier,
			freespins_buying_price: obj.freespins_buying_price.into(),
			jackpots: obj.jackpots.into(),
			lines: obj.lines,
			mystery_values: obj.mystery_values,
			paylines: obj.paylines,
			paytable: obj.paytable.into(),
			rows: obj.rows,
			rtp: obj.rtp,
			sticky_values: obj.sticky_values,
			symbols: obj.symbols.into_iter().map(Into::into).collect(),
			symbols_line: obj.symbols_line,
			symbols_scat: obj.symbols_scat,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Status {
	pub code: StatusCodesEnum /* FUNDS_EXCEED, OK */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reason: Option<String> /* Insufficient balance */,
	#[serde(rename = "type", skip_serializing_if = "Option::is_none")]
	pub status_type: Option<StatusTypesEnum> /* exceed */,
}

impl From<server::Status> for Status {
	fn from(obj: server::Status) -> Self {
		Status {
			code: obj.code.into(),
			reason: obj.reason,
			status_type: obj.status_type,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
	pub balance: i64 /* 500000 */,
	pub balance_version: i64 /* 1752067922213 */,
	pub currency: String /* FUN */,
	pub huid: String /* "686e6f522c7c80483b132b54" */,
	pub nick: String /* Player 3a9b80b6-e351-4713-9627-3ad37a961139, Player 481f841a-566d-4f5d-99b7-c085c96e378e, Player 5a8c32af-ffda-4247-8eb5-05f73d3148e9, Player c80b1aac-2423-4f93-9058-3392e18805de */,
	pub show_balance: bool /* true */,
}

impl From<server::User> for User {
	fn from(obj: server::User) -> Self {
		User {
			balance: obj.balance,
			balance_version: obj.balance_version,
			currency: obj.currency,
			huid: obj.huid,
			nick: obj.nick.into(),
			show_balance: obj.show_balance,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Start {
	pub command: CommandsEnum /* start */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub context: Option<Context>,
	pub modes: Vec<ModesEnum> /* auto, play */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_data: Option<OriginData>,
	pub request_id: String /* "e144a8b71e4b4438a30d71dbbebe6c8e" */,
	pub session_id: String /* "17520679221969UH15ouuh3xFUSvXY.EmVaz7x07pImki9byd2v" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub settings: Option<Settings>,
	pub status: Status,
	pub user: User,
}

impl From<server::Server> for Start {
	fn from(obj: server::Server) -> Self {
		Start {
			command: obj.command,
			context: obj.context.map(Into::into),
			modes: obj.modes.into_iter().map(Into::into).collect(),
			origin_data: obj.origin_data.map(|o|o.into()),
			request_id: obj.request_id,
			session_id: obj.session_id,
			settings: obj.settings.map(|s|s.into()),
			status: obj.status.into(),
			user: obj.user.into(),
		}
	}
}

