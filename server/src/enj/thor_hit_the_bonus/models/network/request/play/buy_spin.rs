use super::super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::super::enums::{BonusModesEnum, CommandsEnum, ActionsEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Params {
	pub bet_factor: i64 /* 10 */,
	pub bet_per_line: i64 /* 20 */,
	pub lines: i64 /* 1 */,
	pub selected_mode: BonusModesEnum /* 1, 2, 3 */,
}

impl From<client::Params> for Params {
	fn from(obj: client::Params) -> Self {
		Params {
			bet_factor: obj.bet_factor.unwrap_or_default(),
			bet_per_line: obj.bet_per_line.unwrap_or_default(),
			lines: obj.lines.unwrap_or_default(),
			selected_mode: obj.selected_mode.unwrap_or_default().into(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Action {
	pub name: ActionsEnum /* buy_spin */,
	pub params: Params,
}

impl From<client::Action> for Action {
	fn from(obj: client::Action) -> Self {
		Action {
			name: obj.name,
			params: obj.params.into(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BuySpin {
	pub action: Action,
	pub autogame: bool /* false */,
	pub command: CommandsEnum /* play */,
	pub mobile: bool /* false */,
	pub portrait: bool /* false */,
	pub prev_client_command_time: i64 /* 29 */,
	pub quick_spin: i64 /* 2 */,
	pub request_id: String /* "84db061b2b5c41b89810510a4c8c283e" */,
	pub session_id: String /* "1753038333859mEuhZklAbJO2lwinJ.RJM4TphE96hbWcWtcHEW" */,
	pub sound: bool /* false */,
}

impl From<client::Client> for BuySpin {
	fn from(obj: client::Client) -> Self {
		BuySpin {
			action: obj.action.unwrap_or_default().into(),
			autogame: obj.autogame.unwrap_or_default(),
			command: obj.command,
			mobile: obj.mobile.unwrap_or_default(),
			portrait: obj.portrait.unwrap_or_default(),
			prev_client_command_time: obj.prev_client_command_time.unwrap_or_default(),
			quick_spin: obj.quick_spin.unwrap_or_default(),
			request_id: obj.request_id,
			session_id: obj.session_id.unwrap_or_default(),
			sound: obj.sound.unwrap_or_default(),
		}
	}
}

