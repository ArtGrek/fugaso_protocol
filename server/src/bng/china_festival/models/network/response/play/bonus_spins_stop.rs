use super::super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::super::server::{ContextActionsEnum, ServerModesEnum, ServerCommandEnum, StatusCodeEnum, ContextLastActionEnum, ContextCurrentEnum, };

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
pub struct Bac {
	#[serde(rename = "1")]
	pub bac_1: Vec<i64> /* [0,0], [1,0], [1,1], [10,0], [10,10], [10,11], [10,12], [10,13], [10,14], [10,15], [10,16], [10,17], [10,18], [10,19], [10,1], [10,2], [10,3], [10,4], [10,5], [10,6], [10,7], [10,8], [10,9], [11,0], [11,10], [11,11], [11,12], [11,13], [11,14], [11,15], [11,16], [11,17], [11,18], [11,19], [11,1], [11,20], [11,21], [11,22], [11,23], [11,24], [11,2], [11,3], [11,4], [11,5], [11,6], [11,7], [11,8], [11,9], [12,0], [12,10], [12,11], [12,12], [12,13], [12,14], [12,15], [12,16], [12,17], [12,18], [12,19], [12,1], [12,20], [12,21], [12,22], [12,23], [12,24], [12,25], [12,26], [12,27], [12,28], [12,29], [12,2], [12,3], [12,4], [12,5], [12,6], [12,7], [12,8], [12,9], [13,0], [2,0], [2,1], [2,2], [3,0], [3,1], [3,2], [3,3], [4,0], [4,1], [4,2], [4,3], [5,0], [5,1], [5,2], [5,3], [5,4], [6,0], [6,1], [6,2], [6,3], [6,4], [6,5], [7,0], [7,1], [7,2], [7,3], [7,4], [7,5], [7,6], [7,7], [8,0], [8,1], [8,2], [8,3], [8,4], [8,5], [8,6], [8,7], [8,8], [8,9], [9,0], [9,10], [9,11], [9,12], [9,13], [9,14], [9,1], [9,2], [9,3], [9,4], [9,5], [9,6], [9,7], [9,8], [9,9] */,
	#[serde(rename = "2")]
	pub bac_2: Vec<i64> /* [0,0], [1,0], [1,1], [10,0], [10,10], [10,11], [10,12], [10,13], [10,14], [10,15], [10,16], [10,17], [10,18], [10,19], [10,1], [10,2], [10,3], [10,4], [10,5], [10,6], [10,7], [10,8], [10,9], [11,0], [11,10], [11,11], [11,12], [11,13], [11,14], [11,15], [11,16], [11,17], [11,18], [11,19], [11,1], [11,20], [11,21], [11,22], [11,23], [11,24], [11,2], [11,3], [11,4], [11,5], [11,6], [11,7], [11,8], [11,9], [12,0], [12,10], [12,11], [12,12], [12,13], [12,14], [12,15], [12,16], [12,17], [12,18], [12,19], [12,1], [12,20], [12,21], [12,22], [12,23], [12,24], [12,25], [12,26], [12,27], [12,28], [12,29], [12,2], [12,3], [12,4], [12,5], [12,6], [12,7], [12,8], [12,9], [13,0], [2,0], [2,1], [2,2], [3,0], [3,1], [3,2], [3,3], [4,0], [4,1], [4,2], [4,3], [5,0], [5,1], [5,2], [5,3], [5,4], [6,0], [6,1], [6,2], [6,3], [6,4], [6,5], [7,0], [7,1], [7,2], [7,3], [7,4], [7,5], [7,6], [7,7], [8,0], [8,1], [8,2], [8,3], [8,4], [8,5], [8,6], [8,7], [8,8], [8,9], [9,0], [9,10], [9,11], [9,12], [9,13], [9,14], [9,1], [9,2], [9,3], [9,4], [9,5], [9,6], [9,7], [9,8], [9,9] */,
	#[serde(rename = "3")]
	pub bac_3: Vec<i64> /* [0,0], [1,0], [1,1], [10,0], [10,10], [10,11], [10,12], [10,13], [10,14], [10,15], [10,16], [10,17], [10,18], [10,19], [10,1], [10,2], [10,3], [10,4], [10,5], [10,6], [10,7], [10,8], [10,9], [11,0], [11,10], [11,11], [11,12], [11,13], [11,14], [11,15], [11,16], [11,17], [11,18], [11,19], [11,1], [11,20], [11,21], [11,22], [11,23], [11,24], [11,2], [11,3], [11,4], [11,5], [11,6], [11,7], [11,8], [11,9], [12,0], [12,10], [12,11], [12,12], [12,13], [12,14], [12,15], [12,16], [12,17], [12,18], [12,19], [12,1], [12,20], [12,21], [12,22], [12,23], [12,24], [12,25], [12,26], [12,27], [12,28], [12,29], [12,2], [12,3], [12,4], [12,5], [12,6], [12,7], [12,8], [12,9], [13,0], [2,0], [2,1], [2,2], [3,0], [3,1], [3,2], [3,3], [4,0], [4,1], [4,2], [4,3], [5,0], [5,1], [5,2], [5,3], [5,4], [6,0], [6,1], [6,2], [6,3], [6,4], [6,5], [7,0], [7,1], [7,2], [7,3], [7,4], [7,5], [7,6], [7,7], [8,0], [8,1], [8,2], [8,3], [8,4], [8,5], [8,6], [8,7], [8,8], [8,9], [9,0], [9,10], [9,11], [9,12], [9,13], [9,14], [9,1], [9,2], [9,3], [9,4], [9,5], [9,6], [9,7], [9,8], [9,9] */,
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
	pub bac_win: bool /* false */,
	pub bet_per_line: i64 /* 1, 150 */,
	pub board: Vec<Vec<i64>> /* [[3,3,3],[1,1,1],[2,11,3],[5,8,8],[4,13,5]] */,
	pub bs_count: i64 /* 0 */,
	pub is_lucky_spin: bool /* false */,
	pub lines: i64 /* 25 */,
	pub paid: bool /* false */,
	pub round_bet: i64 /* 20, 3000 */,
	pub round_win: i64 /* 0 */,
	pub spin_type: i64 /* 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 */,
	pub total_win: i64 /* 682500 */,
}

impl From<server::Spins> for Spins {
	fn from(obj: server::Spins) -> Self {
		Spins {
			bac: obj.bac.into(),
			bac_win: obj.bac_win.unwrap_or_default(),
			bet_per_line: obj.bet_per_line,
			board: obj.board,
			bs_count: obj.bs_count,
			is_lucky_spin: obj.is_lucky_spin,
			lines: obj.lines,
			paid: obj.paid.unwrap_or_default(),
			round_bet: obj.round_bet,
			round_win: obj.round_win,
			spin_type: obj.spin_type.unwrap_or_default(),
			total_win: obj.total_win.unwrap_or_default(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Context {
	pub actions: Vec<ContextActionsEnum> /* buy_spin, spin */,
	pub current: ContextCurrentEnum /* spins */,
	pub last_action: ContextLastActionEnum /* bonus_spins_stop */,
	pub last_args: LastArgs,
	pub last_win: i64 /* 682500 */,
	pub round_finished: bool /* true */,
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
	pub autogame: bool /* false, true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub command: Option<String> /* play */,
	pub feature: bool /* true */,
	pub mobile: String /* 0 */,
	pub portrait: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "4aa94234-c494-4974-9345-5f1bee389809" */,
	pub quickspin: i64 /* 0, 2 */,
	pub set_denominator: i64 /* 1 */,
	pub sound: bool /* false, true */,
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
	pub balance: i64 /* 770350 */,
	pub balance_version: i64 /* 57 */,
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
pub struct BonusSpinsStop {
	pub command: ServerCommandEnum /* play */,
	pub context: Context,
	pub modes: Vec<ServerModesEnum> /* auto, freebet, play */,
	pub origin_data: OriginData,
	pub request_id: String /* "07cb604b-d222-4c3d-b898-46b5aee013b3" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roundnum: Option<String> /* "2505161000003729258" */,
	pub session_id: String /* "54d657fdddea4c76800b216371ea868e" */,
	pub status: Status,
	pub user: User,
}

impl From<server::Server> for BonusSpinsStop {
	fn from(obj: server::Server) -> Self {
		BonusSpinsStop {
			command: obj.command,
			context: obj.context.unwrap_or_default().into(),
			modes: obj.modes.unwrap_or_default().into_iter().map(Into::into).collect(),
			origin_data: obj.origin_data.unwrap_or_default().into(),
			request_id: obj.request_id,
			roundnum: obj.roundnum,
			session_id: obj.session_id.unwrap_or_default(),
			status: obj.status.into(),
			user: obj.user.unwrap_or_default().into(),
		}
	}
}

