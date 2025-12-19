use super::super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::super::server::{ContextActionsEnum, BonusBsVEnum, ServerModesEnum, ServerCommandEnum, StatusCodeEnum, ContextCurrentEnum, ContextLastActionEnum};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CollectValues {
	pub bs_v: f64 /* 60.0 */,
	pub pos: Vec<i64> /* [2,4] */,
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
pub struct Mults {
	pub mult: i64 /* 2, 3, 4, 5, 6, 7, 8, 9, 10 */,
	pub pos: Vec<i64> /* [1,5] */,
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
pub struct Bonus {
	pub bac: Vec<i64> /* [1,0], [1,1], [2,0], [2,1], [2,2], [2,3], [2,4], [3,0], [3,1], [3,2], [3,3], [3,4], [3,5], [3,6], [3,7], [3,8], [3,9], [4,0], [4,10], [4,11], [4,12], [4,13], [4,14], [4,1], [4,2], [4,3], [4,4], [4,5], [4,6], [4,7], [4,8], [4,9], [5,0], [5,10], [5,11], [5,12], [5,13], [5,14], [5,15], [5,16], [5,17], [5,18], [5,19], [5,1], [5,2], [5,3], [5,4], [5,5], [5,6], [5,7], [5,8], [5,9], [6,0], [6,10], [6,11], [6,12], [6,13], [6,14], [6,15], [6,16], [6,17], [6,18], [6,19], [6,1], [6,20], [6,21], [6,22], [6,23], [6,24], [6,25], [6,26], [6,27], [6,28], [6,29], [6,2], [6,3], [6,4], [6,5], [6,6], [6,7], [6,8], [6,9], [7,0] */,
	pub back_to: String /* spins */,
	pub bet_per_line: i64 /* 1 */,
	pub board: Vec<Vec<i64>> /* [[0,0,0,0,1,0],[0,0,0,0,1,0],[0,0,0,0,1,0],[0,0,0,0,1,0]] */,
	pub bonus_game_type: i64 /* 1, 2, 3 */,
	pub bs_count: i64 /* 4, 5, 6, 7 */,
	pub bs_v: Vec<Vec<BonusBsVEnum>> /* [[0,0,0,0,20,0],[0,0,0,0,10.0,0],[0,0,0,0,20,0],[0,0,0,0,80,0]] */,
	pub bs_values: Vec<Vec<f64>> /* [[0,0,0,0,1,0],[0,0,0,0,0.5,0],[0,0,0,0,1,0],[0,0,0,0,4,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub collect_values: Option<Vec<CollectValues>>,
	pub current_win: i64 /* 0 */,
	pub lines: i64 /* 1 */,
	pub mps_: Vec<Vec<i64>> /* [[1,1,1,1,1,1],[1,1,1,1,1,1],[1,1,1,1,1,1],[1,1,1,1,1,1]] */,
	pub mults_: Vec<Mults>,
	pub orig_board: Vec<Vec<i64>> /* [[0,0,0,0,1,0],[0,0,0,0,1,0],[0,0,0,0,1,0],[0,0,0,0,1,0]] */,
	pub round_bet: i64 /* 20 */,
	pub round_win: i64 /* 0 */,
	pub rounds_count: i64 /* 0 */,
	pub rounds_granted: i64 /* 3 */,
	pub rounds_left: i64 /* 3 */,
	pub total_win: i64 /* 0 */,
	pub unlock_row_idx: i64 /* 0, 3 */,
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
			bs_v: obj.bs_v.into_iter().map(|inner_vec| {inner_vec.into_iter().map(Into::into).collect()}).collect(),
			bs_values: obj.bs_values,
			collect_values: obj.collect_values.map(|vec| vec.into_iter().map(Into::into).collect()),
			current_win: obj.current_win,
			lines: obj.lines,
			mps_: obj.mps_,
			mults_: obj.mults_.into_iter().map(Into::into).collect(),
			orig_board: obj.orig_board.unwrap_or_default(),
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
}

impl From<server::LastArgs> for LastArgs {
	fn from(_obj: server::LastArgs) -> Self {
		LastArgs {
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Context {
	pub actions: Vec<ContextActionsEnum> /* respin */,
	pub bonus: Bonus,
	pub current: ContextCurrentEnum /* bonus */,
	pub last_action: ContextLastActionEnum /* bonus_init */,
	pub last_args: LastArgs,
	pub last_win: i64 /* 0 */,
	pub round_finished: bool /* false */,
	pub version: i64 /* 1 */,
}

impl From<server::Context> for Context {
	fn from(obj: server::Context) -> Self {
		Context {
			actions: obj.actions.into_iter().map(Into::into).collect(),
			bonus: obj.bonus.unwrap_or_default().into(),
			current: obj.current,
			last_action: obj.last_action,
			last_args: obj.last_args.into(),
			last_win: obj.last_win.unwrap_or_default(),
			round_finished: obj.round_finished,
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
	pub prev_request_id: Option<String> /* "01c6c84b-2b58-4d3a-85ca-a14faa0411c6" */,
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
	pub balance: i64 /* 99760 */,
	pub balance_version: i64 /* 16 */,
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
pub struct BonusInit {
	pub command: ServerCommandEnum /* play */,
	pub context: Context,
	pub modes: Vec<ServerModesEnum> /* auto, freebet, play */,
	pub origin_data: OriginData,
	pub request_id: String /* "5513d70b-f0b0-41e8-bc03-907809dfa116" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roundnum: Option<String> /* "2505181000005471265" */,
	pub session_id: String /* "577040c7bf0b4dc18036a41bc4527fb7" */,
	pub status: Status,
	pub user: User,
}

impl From<server::Server> for BonusInit {
	fn from(obj: server::Server) -> Self {
		BonusInit {
			command: obj.command,
			context: obj.context.unwrap_or_default().into(),
			modes: obj.modes.unwrap_or_default().into_iter().map(Into::into).collect(),
			origin_data: obj.origin_data.unwrap_or_default().into(),
			request_id: obj.request_id,
			roundnum: obj.roundnum,
			session_id: obj.session_id,
			status: obj.status.into(),
			user: obj.user.unwrap_or_default().into(),
		}
	}
}

