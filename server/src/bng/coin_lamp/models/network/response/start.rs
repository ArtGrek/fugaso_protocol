use super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::server::{ContextActionsEnum, BonusBsVEnum, ContextCurrentEnum, ContextLastActionEnum, LastargsSelectedModeEnum, ServerModesEnum, SymbolsNameEnum, SymbolsTypeEnum, ServerCommandEnum, StatusCodeEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Mults {
	pub mult: i64 /* 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14 */,
	pub pos: Vec<i64> /* [3,5] */,
}

impl From<server::Mults> for Mults {
	fn from(obj: server::Mults) -> Self {
		Mults {
			mult: obj.mult,
			pos: obj.pos,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MysteryValues {
	pub bs_v: f64 /* 20 */,
	pub id: i64 /* 1, 3 */,
	pub pos: Vec<i64> /* [1,5] */,
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
	pub bac: Vec<i64> /* [1,0], [1,1], [2,0], [2,1], [2,2], [2,3], [2,4], [3,0], [3,2], [3,3], [3,4], [3,5], [3,6], [3,7], [3,8], [3,9], [4,0], [4,10], [4,11], [4,12], [4,13], [4,14], [4,1], [4,2], [4,3], [4,4], [4,5], [4,6], [4,7], [4,8], [4,9], [5,0], [5,10], [5,11], [5,12], [5,13], [5,14], [5,15], [5,16], [5,17], [5,19], [5,1], [5,2], [5,3], [5,4], [5,5], [5,6], [5,7], [5,8], [5,9], [6,10], [6,11], [6,12], [6,14], [6,16], [6,17], [6,19], [6,1], [6,20], [6,21], [6,22], [6,23], [6,24], [6,25], [6,27], [6,29], [6,2], [6,3], [6,4], [6,5], [6,6], [6,7], [6,8], [6,9], [7,0] */,
	pub back_to: String /* spins */,
	pub bet_per_line: i64 /* 1 */,
	pub board: Vec<Vec<i64>> /* [[0,0,0,0,1,0],[0,0,0,0,1,0],[0,0,0,0,1,0],[0,0,0,0,1,0]] */,
	pub bonus_game_type: i64 /* 1, 2, 3 */,
	pub bs_count: i64 /* 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bs_pos: Option<Vec<Vec<i64>>> /* [[0,4],[1,4],[2,4],[3,4]] */,
	pub bs_v: Vec<Vec<BonusBsVEnum>> /* [[0,0,0,0,20,0],[0,0,0,0,10.0,0],[0,0,0,0,20,0],[0,0,0,0,80,0]] */,
	pub bs_values: Vec<Vec<f64>> /* [[0,0,0,0,1,0],[0,0,0,0,0.5,0],[0,0,0,0,1,0],[0,0,0,0,4,0]] */,
	pub current_win: i64 /* 0 */,
	pub lines: i64 /* 1 */,
	pub mps_: Vec<Vec<i64>> /* [[1,1,1,1,1,1],[1,1,1,1,1,1],[1,1,1,1,1,1],[1,1,1,1,1,1]] */,
	pub mults_: Vec<Mults>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mystery_values: Option<Vec<MysteryValues>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub new_bs: Option<Vec<Vec<i64>>> /* [[0,2]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub orig_board: Option<Vec<Vec<i64>>> /* [[0,0,0,0,1,0],[0,0,0,0,1,0],[0,0,0,0,1,0],[0,0,0,0,1,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub orig_bs_v: Option<Vec<Vec<BonusBsVEnum>>> /* [[0,0,10.0,0,20,0],[60,0,0,0,10.0,0],[40,0,0,30.0,20,0],[0,60,0,0,80,0]] */,
	pub round_bet: i64 /* 20 */,
	pub round_win: i64 /* 0 */,
	pub rounds_count: i64 /* 0 */,
	pub rounds_granted: i64 /* 3 */,
	pub rounds_left: i64 /* 0, 1, 2, 3 */,
	pub total_win: i64 /* 0 */,
	pub unlock_row_idx: i64 /* 0, 1, 2, 3 */,
}

impl From<server::Bonus> for Bonus {
	fn from(obj: server::Bonus) -> Self {
		Bonus {
			bac: obj.bac,
			back_to: obj.back_to,
			bet_per_line: obj.bet_per_line,
			board: obj.board,
			bonus_game_type: obj.bonus_game_type,
			bs_count: obj.bs_count,
			bs_pos: obj.bs_pos,
			bs_v: obj.bs_v.into_iter().map(|inner_vec| {inner_vec.into_iter().map(Into::into).collect()}).collect(),
			bs_values: obj.bs_values,
			current_win: obj.current_win,
			lines: obj.lines,
			mps_: obj.mps_,
			mults_: obj.mults_.into_iter().map(Into::into).collect(),
			mystery_values: obj.mystery_values.map(|vec| vec.into_iter().map(Into::into).collect()),
			new_bs: obj.new_bs,
			orig_board: obj.orig_board,
			orig_bs_v: obj.orig_bs_v.map(|vec| vec.into_iter().map(Into::into).collect()),
			round_bet: obj.round_bet,
			round_win: obj.round_win,
			rounds_count: obj.rounds_count,
			rounds_granted: obj.rounds_granted,
			rounds_left: obj.rounds_left,
			total_win: obj.total_win,
			unlock_row_idx: obj.unlock_row_idx,
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
	pub lines: Option<i64> /* 1 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<LastargsSelectedModeEnum> /* 1, 2, 3 */,
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
pub struct Progress {
	pub bet: String /* 20 */,
	pub data: Vec<Vec<i64>> /* [[1,2,7]] */,
}

impl From<server::Progress> for Progress {
	fn from(obj: server::Progress) -> Self {
		Progress {
			bet: obj.bet,
			data: obj.data,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Spins {
	pub bac: Vec<i64> /* [1,0], [1,1], [2,0], [2,1], [2,2], [2,3], [2,4], [3,0], [3,1], [3,2], [3,3], [3,4], [3,5], [3,6], [3,7], [3,8], [3,9], [4,0], [4,10], [4,11], [4,12], [4,13], [4,14], [4,1], [4,2], [4,3], [4,4], [4,5], [4,6], [4,7], [4,8], [4,9], [5,0], [5,10], [5,11], [5,12], [5,13], [5,14], [5,15], [5,16], [5,17], [5,18], [5,19], [5,1], [5,2], [5,3], [5,4], [5,5], [5,6], [5,7], [5,8], [5,9], [6,0], [6,10], [6,11], [6,12], [6,13], [6,14], [6,15], [6,16], [6,17], [6,18], [6,19], [6,1], [6,20], [6,21], [6,22], [6,24], [6,25], [6,26], [6,27], [6,28], [6,29], [6,2], [6,3], [6,4], [6,5], [6,6], [6,7], [6,8], [6,9], [7,0] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bac_win: Option<bool> /* false */,
	pub bet_per_line: i64 /* 1, 15 */,
	pub board: Vec<Vec<i64>> /* [[0,4,0],[0,0,1],[0,1,0],[5,0,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus_game_type: Option<i64> /* 0, 1, 2, 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bs_sticky_pos_new_per_spin: Option<Vec<Vec<i64>>> /* [[3,2]] */,
	pub bs_v: Vec<Vec<BonusBsVEnum>> /* [[0,0,0],[0,0,1500],[0,"minor",0],[0,0,0]] */,
	pub bs_values: Vec<Vec<f64>> /* [[0,0,0],[0,0,5],[0,20,0],[0,0,0]] */,
	pub lines: i64 /* 1 */,
	pub mps_: Vec<Vec<i64>> /* [[1,1,1],[1,1,1],[1,1,1],[1,1,1]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mult_new: Option<Vec<Vec<i64>>> /* [[3,2],[1,2]] */,
	pub mults_: Vec<Mults>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub new_bs: Option<Vec<Vec<i64>>> /* [[1,1],[0,1],[3,1],[2,1]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub paid: Option<bool> /* false, true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub previous_board: Option<Vec<Vec<i64>>> /* [[0,1,0],[0,1,0],[0,1,0],[0,1,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub previous_bs_sticky_pos: Option<Vec<Vec<i64>>> /* [[3,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub previous_bs_v: Option<Vec<Vec<BonusBsVEnum>>> /* [[0,20,0],[0,10.0,0],[0,20,0],[0,80,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub previous_bs_values: Option<Vec<Vec<f64>>> /* [[0,1,0],[0,0.5,0],[0,1,0],[0,4,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub progress: Option<Vec<Progress>>,
	pub round_bet: i64 /* 20, 300 */,
	pub round_win: i64 /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<LastargsSelectedModeEnum> /* 1, 2, 3 */,
	pub total_win: i64 /* 0 */,
}

impl From<server::Spins> for Spins {
	fn from(obj: server::Spins) -> Self {
		Spins {
			bac: obj.bac,
			bac_win: obj.bac_win,
			bet_per_line: obj.bet_per_line,
			board: obj.board,
			bonus_game_type: obj.bonus_game_type,
			bs_sticky_pos_new_per_spin: obj.bs_sticky_pos_new_per_spin,
			bs_v: obj.bs_v.into_iter().map(|inner_vec| {inner_vec.into_iter().map(Into::into).collect()}).collect(),
			bs_values: obj.bs_values,
			lines: obj.lines,
			mps_: obj.mps_,
			mult_new: obj.mult_new,
			mults_: obj.mults_.into_iter().map(Into::into).collect(),
			new_bs: obj.new_bs,
			paid: obj.paid,
			previous_board: obj.previous_board,
			previous_bs_sticky_pos: obj.previous_bs_sticky_pos,
			previous_bs_v: obj.previous_bs_v.map(|vec| vec.into_iter().map(Into::into).collect()),
			previous_bs_values: obj.previous_bs_values,
			progress: obj.progress.map(|vec| vec.into_iter().map(Into::into).collect()),
			round_bet: obj.round_bet,
			round_win: obj.round_win,
			selected_mode: obj.selected_mode.map(Into::into),
			total_win: obj.total_win,
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
	pub last_win: Option<i64> /* 0 */,
	pub round_finished: bool /* false, true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub spins: Option<Spins>,
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
			spins: obj.spins.map(Into::into),
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
	pub id: i64 /* 0, 1, 2, 3, 4, 5 */,
	pub name: SymbolsNameEnum /* el_0, el_bonus, el_bonus_sticky, el_collect, el_mystery, el_mystery_jackpot */,
	#[serde(rename = "type")]
	pub symbols_type: SymbolsTypeEnum /* line, scat */,
}

impl From<server::Symbols> for Symbols {
	fn from(obj: server::Symbols) -> Self {
		Symbols {
			id: obj.id,
			name: obj.name.into(),
			symbols_type: obj.symbols_type.into(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Settings {
	pub bet_factor: Vec<i64> /* [20] */,
	pub bets: Vec<i64> /* [1,2,3,4,5,6,7,8,10,15,20,25,30,40,50,75,100,150,200,250,300] */,
	pub bonus_sticky_symbol_values: Vec<i64> /* [5,6,7,8,9] */,
	pub bonus_symbol_v: Vec<BonusBsVEnum> /* [0.5,1,1.5,2,3,4,"mini","minor","major"] */,
	pub buy_bonus_price: Vec<i64> /* [40,100,200] */,
	pub cols: i64 /* 4 */,
	pub currency_format: CurrencyFormat,
	pub jackpots: Jackpots,
	pub keys_to_unlock_rows: Vec<i64> /* [15,11,8] */,
	pub lines: Vec<i64> /* [1] */,
	pub paytable: Paytable,
	pub respins_granted: i64 /* 3 */,
	pub rows: i64 /* 3 */,
	pub symbols: Vec<Symbols>,
	pub symbols_scat: Vec<i64> /* [1,2,3,4,5] */,
}

impl From<server::Settings> for Settings {
	fn from(obj: server::Settings) -> Self {
		Settings {
			bet_factor: obj.bet_factor,
			bets: obj.bets,
			bonus_sticky_symbol_values: obj.bonus_sticky_symbol_values,
			bonus_symbol_v: obj.bonus_symbol_v.into_iter().map(Into::into).collect(),
			buy_bonus_price: obj.buy_bonus_price,
			cols: obj.cols,
			currency_format: obj.currency_format.into(),
			jackpots: obj.jackpots.into(),
			keys_to_unlock_rows: obj.keys_to_unlock_rows,
			lines: obj.lines,
			paytable: obj.paytable.into(),
			respins_granted: obj.respins_granted,
			rows: obj.rows,
			symbols: obj.symbols.into_iter().map(Into::into).collect(),
			symbols_scat: obj.symbols_scat,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Status {
	pub code: StatusCodeEnum /* OK */,
}

impl From<server::Status> for Status {
	fn from(obj: server::Status) -> Self {
		Status {
			code: obj.code,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
	pub balance: i64 /* 100000 */,
	pub balance_version: i64 /* 2 */,
	pub currency: String /* FUN */,
	pub huid: String /* "demo-e55b3f9a0a5f4e42ac8114faaf0d413f" */,
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
	pub context: Context,
	pub modes: Vec<ServerModesEnum> /* auto, freebet, play */,
	pub request_id: String /* "85e5a638-611a-467e-a7b1-9c9d104b0041" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roundnum: Option<String> /* "2505181000005471265" */,
	pub session_id: String /* "577040c7bf0b4dc18036a41bc4527fb7" */,
	pub settings: Settings,
	pub status: Status,
	pub user: User,
}

impl From<server::Server> for Start {
	fn from(obj: server::Server) -> Self {
		Start {
			command: obj.command,
			context: obj.context.unwrap_or_default().into(),
			modes: obj.modes.unwrap_or_default().into_iter().map(Into::into).collect(),
			request_id: obj.request_id,
			roundnum: obj.roundnum,
			session_id: obj.session_id,
			settings: obj.settings.unwrap_or_default().into(),
			status: obj.status.into(),
			user: obj.user.unwrap_or_default().into(),
		}
	}
}

