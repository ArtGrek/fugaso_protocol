use super::super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::super::server::{ContextActionsEnum, LastargsSelectedModeEnum, BonusBsVEnum, ServerModesEnum, StatusCodeEnum, ServerCommandEnum, StatusTypeEnum, ContextCurrentEnum, ContextLastActionEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LastArgs {
	pub bet_factor: i64 /* 20 */,
	pub bet_per_line: i64 /* 1 */,
	pub lines: i64 /* 1 */,
	pub selected_mode: LastargsSelectedModeEnum /* 1, 2, 3 */,
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
pub struct Mults {
	pub mult: i64 /* 2, 3, 4, 5, 6, 7, 8, 9, 10 */,
	pub pos: Vec<i64> /* [1,2] */,
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
pub struct Progress {
	pub bet: String /* 20 */,
	pub data: Vec<Vec<i64>> /* [[2,2,5]] */,
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
	pub bac: Vec<i64> /* [1,0], [1,1], [2,0], [2,1], [2,2], [2,3], [2,4], [3,0], [3,1], [3,2], [3,3], [3,4], [3,5], [3,6], [3,7], [3,8], [3,9], [4,0], [4,10], [4,11], [4,12], [4,13], [4,14], [4,1], [4,2], [4,3], [4,4], [4,5], [4,6], [4,7], [4,8], [4,9], [5,0], [5,10], [5,11], [5,12], [5,13], [5,14], [5,15], [5,16], [5,17], [5,18], [5,19], [5,1], [5,2], [5,3], [5,4], [5,5], [5,6], [5,7], [5,8], [5,9], [6,0], [6,10], [6,11], [6,12], [6,13], [6,14], [6,15], [6,16], [6,17], [6,18], [6,19], [6,1], [6,20], [6,21], [6,22], [6,23], [6,24], [6,25], [6,26], [6,27], [6,28], [6,29], [6,2], [6,3], [6,4], [6,5], [6,6], [6,7], [6,8], [6,9], [7,0] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bac_win: Option<bool> /* false */,
	pub bet_per_line: i64 /* 1 */,
	pub board: Vec<Vec<i64>> /* [[0,4,0],[0,1,1],[0,1,0],[0,1,0]] */,
	pub bonus_game_type: i64 /* 1, 2, 3 */,
	pub bs_v: Vec<Vec<BonusBsVEnum>> /* [[0,0,0],[0,30.0,10.0],[0,30.0,0],[0,20,0]] */,
	pub bs_values: Vec<Vec<f64>> /* [[0,0,0],[0,1.5,0.5],[0,1.5,0],[0,1,0]] */,
	pub lines: i64 /* 1 */,
	pub mps_: Vec<Vec<i64>> /* [[1,1,1],[1,1,3],[1,1,1],[1,1,2]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mult_new: Option<Vec<Vec<i64>>> /* [[3,2],[1,2]] */,
	pub mults_: Vec<Mults>,
	pub new_bs: Vec<Vec<i64>> /* [[2,1],[3,1],[1,1],[0,1],[1,2]] */,
	pub paid: bool /* true */,
	pub previous_board: Vec<Vec<i64>> /* [[0,1,0],[0,1,0],[0,1,0],[0,1,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub previous_bs_sticky_pos: Option<Vec<Vec<i64>>> /* [[2,2]] */,
	pub previous_bs_v: Vec<Vec<BonusBsVEnum>> /* [[0,20,0],[0,10.0,0],[0,20,0],[0,80,0]] */,
	pub previous_bs_values: Vec<Vec<f64>> /* [[0,1,0],[0,0.5,0],[0,1,0],[0,4,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub progress: Option<Vec<Progress>>,
	pub round_bet: i64 /* 20 */,
	pub round_win: i64 /* 0 */,
	pub selected_mode: LastargsSelectedModeEnum /* 1, 2, 3 */,
	pub total_win: i64 /* 0 */,
}

impl From<server::Spins> for Spins {
	fn from(obj: server::Spins) -> Self {
		Spins {
			bac: obj.bac,
			bac_win: obj.bac_win,
			bet_per_line: obj.bet_per_line,
			board: obj.board,
			bonus_game_type: obj.bonus_game_type.unwrap_or_default(),
			bs_v: obj.bs_v.into_iter().map(|inner_vec| {inner_vec.into_iter().map(Into::into).collect()}).collect(),
			bs_values: obj.bs_values,
			lines: obj.lines,
			mps_: obj.mps_,
			mult_new: obj.mult_new,
			mults_: obj.mults_.into_iter().map(Into::into).collect(),
			new_bs: obj.new_bs.unwrap_or_default(),
			paid: obj.paid.unwrap_or_default(),
			previous_board: obj.previous_board.unwrap_or_default(),
			previous_bs_sticky_pos: obj.previous_bs_sticky_pos,
			previous_bs_v: obj.previous_bs_v.unwrap_or_default().into_iter().map(|inner_vec| {inner_vec.into_iter().map(Into::into).collect()}).collect(),
			previous_bs_values: obj.previous_bs_values.unwrap_or_default(),
			progress: obj.progress.map(|vec| vec.into_iter().map(Into::into).collect()),
			round_bet: obj.round_bet,
			round_win: obj.round_win,
			selected_mode: obj.selected_mode.unwrap_or_default().into(),
			total_win: obj.total_win,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Context {
	pub actions: Vec<ContextActionsEnum> /* bonus_init */,
	pub current: ContextCurrentEnum /* spins */,
	pub last_action: ContextLastActionEnum /* buy_spin */,
	pub last_args: LastArgs,
	pub last_win: i64 /* 160 */,
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
			spins: obj.spins.unwrap_or_default().into(),
			version: obj.version,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct OriginData {
	pub autogame: bool /* true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub command: Option<String> /* play */,
	pub feature: bool /* true */,
	pub mobile: String /* 0 */,
	pub portrait: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "3e935ac4-8445-4e63-867c-8bdbf052bea3" */,
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
	pub code: StatusCodeEnum /* FUNDS_EXCEED, OK */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub traceback: Option<String> /* NOT_ENOUGH_MONEY */,
	#[serde(rename = "type", skip_serializing_if = "Option::is_none")]
	pub status_type: Option<StatusTypeEnum> /* exceed */,
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
	pub balance: i64 /* 97920 */,
	pub balance_version: i64 /* 27 */,
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
pub struct BuySpin {
	pub command: ServerCommandEnum /* play */,
	pub context: Context,
	pub modes: Vec<ServerModesEnum> /* auto, freebet, play */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_data: Option<OriginData>,
	pub request_id: String /* "038c5031-7d1d-4193-8910-7c051764c223" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roundnum: Option<String> /* "2505181000005462607" */,
	pub session_id: String /* "577040c7bf0b4dc18036a41bc4527fb7" */,
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

