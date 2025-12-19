use super::super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::super::enums::{ActionsEnum, MultiValueEnum, ModesEnum, StatusCodesEnum, CommandsEnum, StatusTypesEnum, CurrentActionsEnum, CurrenciesEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LastArgs {
	pub bet_per_line: i64 /* 1 */,
	pub lines: i64 /* 25 */,
}

impl From<server::LastArgs> for LastArgs {
	fn from(obj: server::LastArgs) -> Self {
		LastArgs {
			bet_per_line: obj.bet_per_line.unwrap_or_default(),
			lines: obj.lines.unwrap_or_default(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bac {
	#[serde(rename = "1")]
	pub bac_1: Vec<i64> /* [1,0], [2,0], [2,1], [3,0], [3,1], [3,2], [4,0], [4,1], [4,2], [4,3], [5,0], [5,1], [5,2], [5,3], [6,0], [6,1], [6,2], [6,3], [7,0], [7,1], [7,2], [7,3], [7,4], [8,0], [8,1], [8,2], [8,3], [8,4], [9,0] */,
	#[serde(rename = "2")]
	pub bac_2: Vec<i64> /* [1,0], [2,0], [2,1], [3,0], [3,1], [3,2], [4,0], [4,1], [4,2], [4,3], [5,0], [5,1], [5,2], [5,3], [6,0], [6,1], [6,2], [6,3], [7,0], [7,1], [7,2], [7,3], [7,4], [8,0], [8,1], [8,2], [8,3], [8,4], [9,0] */,
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
	pub bac_pos: Option<Vec<Vec<i64>>> /* [[0,0],[1,2],[3,1],[3,2],[4,2]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bac_win: Option<bool> /* false, true */,
	pub bet_per_line: i64 /* 1 */,
	pub board: Vec<Vec<i64>> /* [[4,4,13],[1,8,8],[7,7,3],[8,8,8],[3,3,6]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus_mechanic: Option<Vec<i64>> /* [1,2,3], [1,2], [1,3], [1], [2,3], [2], [3] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bs_count: Option<i64> /* 6 */,
	pub bs_v: Vec<Vec<MultiValueEnum>> /* [[0,0,0],[0,0,0],[0,0,0],[0,0,0],[0,0,0]] */,
	pub bs_values: Vec<Vec<MultiValueEnum>> /* [[0,0,0],[0,0,0],[0,0,0],[0,0,0],[0,0,0]] */,
	pub lines: i64 /* 25 */,
	pub lucky_spin_win: bool /* false, true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_board: Option<Vec<Vec<i64>>> /* [[4,6,6],[6,6,2],[1,1,12],[3,7,7],[1,1,6]] */,
	pub round_bet: i64 /* 20 */,
	pub round_win: i64 /* 0 */,
	pub total_win: i64 /* 0 */,
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
			lucky_spin_win: obj.lucky_spin_win.unwrap_or_default(),
			origin_board: obj.origin_board,
			round_bet: obj.round_bet,
			round_win: obj.round_win,
			total_win: obj.total_win.unwrap_or_default(),
			winlines: obj.winlines.map(|vec| vec.into_iter().map(Into::into).collect()),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Context {
	pub actions: Vec<ActionsEnum> /* bonus_init, buy_spin, spin */,
	pub current: CurrentActionsEnum /* spins */,
	pub last_action: ActionsEnum /* spin */,
	pub last_args: LastArgs,
	pub last_win: i64 /* 0 */,
	pub round_finished: bool /* false, true */,
	pub spins: Spins,
	pub version: i64 /* 1 */,
}

impl From<server::Context> for Context {
	fn from(obj: server::Context) -> Self {
		Context {
			actions: obj.actions.into_iter().map(Into::into).collect(),
			current: obj.current,
			last_action: obj.last_action,
			last_args: obj.last_args.into(),
			last_win: obj.last_win.unwrap_or_default(),
			round_finished: obj.round_finished,
			spins: obj.spins.into(),
			version: obj.version,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct OriginData {
	pub autogame: bool /* true */,
	pub command: CommandsEnum /* play */,
	pub feature: bool /* false */,
	pub mobile: String /* 0 */,
	pub portrait: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "ae4106cc-dca2-4910-b2d6-a05f38d71f07" */,
	pub quickspin: i64 /* 2 */,
	pub set_denominator: i64 /* 1 */,
	pub sound: bool /* false */,
}

impl From<server::OriginData> for OriginData {
	fn from(obj: server::OriginData) -> Self {
		OriginData {
			autogame: obj.autogame,
			command: obj.command,
			feature: obj.feature,
			mobile: obj.mobile,
			portrait: obj.portrait,
			prev_request_id: obj.prev_request_id,
			quickspin: obj.quickspin,
			set_denominator: obj.set_denominator,
			sound: obj.sound,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Status {
	pub code: StatusCodesEnum /* FUNDS_EXCEED, OK */,
	#[serde(rename = "type", skip_serializing_if = "Option::is_none")]
	pub status_type: Option<StatusTypesEnum> /* exceed */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub traceback: Option<String> /* NOT_ENOUGH_MONEY */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<i64> /* -1 */,
}

impl From<server::Status> for Status {
	fn from(obj: server::Status) -> Self {
		Status {
			code: obj.code.into(),
			traceback: obj.traceback,
			status_type: obj.status_type,
			user_id: obj.user_id,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
	pub balance: i64 /* 99980 */,
	pub balance_version: i64 /* 3 */,
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
pub struct Spin {
	pub command: CommandsEnum /* play */,
	pub context: Context,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_data: Option<OriginData>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roundnum: Option<String> /* "2505171000004302972" */,
	pub modes: Vec<ModesEnum> /* auto, freebet, play */,
	pub session_id: String /* "04d1923972bc43a9a629302732728d65" */,
	pub request_id: String /* "c63ecae9-2b5e-4e2d-b2d4-324e6a81add5" */,
	pub status: Status,
	pub user: User,
}

impl From<server::Server> for Spin {
	fn from(obj: server::Server) -> Self {
		Spin {
			command: obj.command,
			context: obj.context.unwrap_or_default().into(),
			modes: obj.modes.unwrap_or_default().into_iter().map(Into::into).collect(),
			origin_data: obj.origin_data.map(Into::into),
			request_id: obj.request_id,
			roundnum: obj.roundnum,
			session_id: obj.session_id,
			status: obj.status.into(),
			user: obj.user.unwrap_or_default().into(),
		}
	}
}

