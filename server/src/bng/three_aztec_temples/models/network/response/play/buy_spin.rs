use super::super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::super::enums::{ActionsEnum, BonusModesEnum, MultiValueEnum, ModesEnum, StatusCodesEnum, CommandsEnum, StatusTypesEnum, CurrentActionsEnum, CurrenciesEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LastArgs {
	pub bet_factor: i64 /* 20 */,
	pub bet_per_line: i64 /* 1 */,
	pub lines: i64 /* 25 */,
	pub selected_mode: BonusModesEnum /* 1, 2 */,
}

impl From<server::LastArgs> for LastArgs {
	fn from(obj: server::LastArgs) -> Self {
		LastArgs {
			bet_factor: obj.bet_factor.unwrap_or_default(),
			bet_per_line: obj.bet_per_line.unwrap_or_default(),
			lines: obj.lines.unwrap_or_default(),
			selected_mode: obj.selected_mode.unwrap_or_default().into(),
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
pub struct Spins {
	pub bac: Bac,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bac_pos: Option<Vec<Vec<i64>>> /* [[4,1],[4,0],[3,0],[2,0],[0,2]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bac_win: Option<bool> /* false */,
	pub bet_per_line: i64 /* 1 */,
	pub board: Vec<Vec<i64>> /* [[5,5,10],[4,4,13],[10,4,4],[10,4,4],[10,10,4]] */,
	pub bonus_mechanic: Vec<i64> /* [1,2,3], [1,2], [1,3], [1], [2,3], [2], [3] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bs_count: Option<i64> /* 6 */,
	pub bs_v: Vec<Vec<MultiValueEnum>> /* [[0,0,20],[0,0,0],[30.0,0,0],[10.0,0,0],[40,50.0,0]] */,
	pub bs_values: Vec<Vec<MultiValueEnum>> /* [[0,0,1],[0,0,0],[1.5,0,0],[0.5,0,0],[2,2.5,0]] */,
	pub lines: i64 /* 25 */,
	pub lucky_spin_win: bool /* false */,
	pub origin_board: Vec<Vec<i64>> /* [[5,5,1],[4,4,13],[9,4,4],[4,4,4],[6,4,4]] */,
	pub round_bet: i64 /* 20 */,
	pub round_win: i64 /* 0 */,
	pub selected_mode: BonusModesEnum /* 1, 2 */,
	pub total_win: i64 /* 0 */,
}

impl From<server::Spins> for Spins {
	fn from(obj: server::Spins) -> Self {
		Spins {
			bac: obj.bac.into(),
			bac_pos: obj.bac_pos,
			bac_win: obj.bac_win,
			bet_per_line: obj.bet_per_line,
			board: obj.board,
			bonus_mechanic: obj.bonus_mechanic.unwrap_or_default(),
			bs_count: obj.bs_count,
			bs_v: obj.bs_v.into_iter().map(|inner_vec| {inner_vec.into_iter().map(Into::into).collect()}).collect(),
			bs_values: obj.bs_values,
			lines: obj.lines,
			lucky_spin_win: obj.lucky_spin_win.unwrap_or_default(),
			origin_board: obj.origin_board.unwrap_or_default(),
			round_bet: obj.round_bet,
			round_win: obj.round_win,
			selected_mode: obj.selected_mode.unwrap_or_default().into(),
			total_win: obj.total_win.unwrap_or_default(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Context {
	pub actions: Vec<ActionsEnum> /* bonus_init */,
	pub current: CurrentActionsEnum /* spins */,
	pub last_action: ActionsEnum /* buy_spin */,
	pub last_args: LastArgs,
	pub last_win: i64 /* 50 */,
	pub round_finished: bool /* false */,
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
	pub feature: bool /* true */,
	pub mobile: String /* 0 */,
	pub portrait: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "52f719fc-f5b1-462d-9916-436a954394b8" */,
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
	#[serde(skip_serializing_if = "Option::is_none")]
	pub traceback: Option<String> /* NOT_ENOUGH_MONEY */,
	#[serde(rename = "type", skip_serializing_if = "Option::is_none")]
	pub status_type: Option<StatusTypesEnum> /* exceed */,
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
	pub balance: i64 /* 97985 */,
	pub balance_version: i64 /* 48 */,
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
pub struct BuySpin {
	pub command: CommandsEnum /* play */,
	pub context: Context,
	pub modes: Vec<ModesEnum> /* auto, freebet, play */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_data: Option<OriginData>,
	pub request_id: String /* "b99dc040-5942-4f54-a639-3421242af4ac" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roundnum: Option<String> /* "2505171000004303109" */,
	pub session_id: String /* "04d1923972bc43a9a629302732728d65" */,
	pub status: Status,
	pub user: User,
}

impl From<server::Server> for BuySpin {
	fn from(obj: server::Server) -> Self {
		BuySpin {
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

