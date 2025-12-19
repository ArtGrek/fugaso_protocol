use super::super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::super::enums::{ActionsEnum, MultiValueEnum, BonusModesEnum, ModesEnum, CommandsEnum, StatusCodesEnum, CurrentActionsEnum, CurrenciesEnum, };

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
pub struct BoostValues {
	pub bs_v: MultiValueEnum /* 140 */,
	pub pos: Vec<i64> /* [0,2] */,
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
	pub bs_v: MultiValueEnum /* 360.0 */,
	pub pos: Vec<i64> /* [0,1] */,
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
	pub bs_v: MultiValueEnum /* 40 */,
	pub mult_value: i64 /* 2, 3, 5 */,
	pub pos: Vec<i64> /* [2,2] */,
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
	pub id: i64 /* 10, 11, 12, 13 */,
	pub pos: Vec<i64> /* [0,1] */,
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
	pub board: Vec<Vec<i64>> /* [[10,7,7],[7,7,4],[5,10,10],[4,12,10],[10,10,1]] */,
	pub bonus_game_type: i64 /* 1, 2, 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus_mechanic: Option<Vec<i64>> /* [1,2,3], [1,2], [1,3], [1], [2,3], [2], [3] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub boost_values: Option<Vec<BoostValues>>,
	pub bs_count: i64 /* 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 */,
	pub bs_pos: Vec<Vec<i64>> /* [[0,0],[2,1],[2,2],[3,1],[4,0],[4,1]] */,
	pub bs_v: Vec<Vec<MultiValueEnum>> /* [[10.0,0,0],[0,0,0],[0,20,50.0],[0,120.0,10.0],[30.0,10.0,0]] */,
	pub bs_values: Vec<Vec<MultiValueEnum>> /* [[0.5,0,0],[0,0,0],[0,1,2.5],[0,6.0,0.5],[1.5,0.5,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub collect_values: Option<Vec<CollectValues>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub init_bs_count: Option<bool> /* true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jackpot_positions: Option<Vec<Vec<MultiValueEnum>>> /* [[0,0,"mini"],[0,0,0],[0,0,0],[0,0,0],[0,0,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jackpot_values: Option<Vec<i64>> /* [200,400,2000] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jackpots_boost_values: Option<Vec<Vec<i64>>> /* [[0,0,0],[0,0,0],[100,0,0],[0,0,0],[0,0,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jackpots_multiplier_values: Option<Vec<Vec<i64>>> /* [[0,0,2],[0,0,0],[0,0,0],[0,0,0],[0,0,0]] */,
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
	pub origin_board: Option<Vec<Vec<i64>>> /* [[5,14,10],[4,4,13],[10,4,4],[10,4,4],[10,10,4]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_bs_v: Option<Vec<Vec<MultiValueEnum>>> /* [[0,0,40],[0,0,60],[60.0,0,0],[20.0,0,0],[80,100.0,0]] */,
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
			bs_pos: obj.bs_pos.unwrap_or_default(),
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
}

impl From<server::LastArgs> for LastArgs {
	fn from(_obj: server::LastArgs) -> Self {
		LastArgs {
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
	pub bet_per_line: i64 /* 1 */,
	pub board: Vec<Vec<i64>> /* [[10,7,7],[7,7,4],[5,10,10],[4,12,4],[10,10,1]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus_mechanic: Option<Vec<i64>> /* [1,2,3], [1,2], [1,3], [1], [2,3], [2], [3] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bs_count: Option<i64> /* 6 */,
	pub bs_v: Vec<Vec<MultiValueEnum>> /* [[10.0,0,0],[0,0,0],[0,20,50.0],[0,120.0,0],[30.0,10.0,0]] */,
	pub bs_values: Vec<Vec<MultiValueEnum>> /* [[0.5,0,0],[0,0,0],[0,1,2.5],[0,6.0,0],[1.5,0.5,0]] */,
	pub lines: i64 /* 25 */,
	pub lucky_spin_win: bool /* false, true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_board: Option<Vec<Vec<i64>>> /* [[5,5,1],[4,4,13],[9,4,4],[4,4,4],[6,4,4]] */,
	pub round_bet: i64 /* 20 */,
	pub round_win: i64 /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<BonusModesEnum> /* 1, 2 */,
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
			selected_mode: obj.selected_mode.map(Into::into),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Context {
	pub actions: Vec<ActionsEnum> /* bonus_spins_stop, respin */,
	pub bonus: Bonus,
	pub current: CurrentActionsEnum /* bonus */,
	pub last_action: ActionsEnum /* respin */,
	pub last_args: LastArgs,
	pub last_win: i64 /* 40 */,
	pub round_finished: bool /* false */,
	pub spins: Spins,
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
	pub prev_request_id: Option<String> /* "c6a22a27-a9f4-4b6b-bc5e-7ddf8a9679f1" */,
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
	pub code: StatusCodesEnum /* OK */,
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
	pub balance: i64 /* 99735 */,
	pub balance_version: i64 /* 29 */,
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
pub struct Respin {
	pub command: CommandsEnum /* play */,
	pub context: Context,
	pub modes: Vec<ModesEnum> /* auto, freebet, play */,
	pub origin_data: OriginData,
	pub request_id: String /* "bcba8f61-359f-4dc8-a609-a810f768260d" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roundnum: Option<String> /* "2505171000004313117" */,
	pub session_id: String /* "04d1923972bc43a9a629302732728d65" */,
	pub status: Status,
	pub user: User,
}

impl From<server::Server> for Respin {
	fn from(obj: server::Server) -> Self {
		Respin {
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

