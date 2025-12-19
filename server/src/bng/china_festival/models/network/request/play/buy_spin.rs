use super::super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::super::client::{ParamsSelectedModeEnum, ClientCommandEnum, ActionNameEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Params {
	pub bet_factor: i64 /* 20 */,
	pub bet_per_line: i64 /* 1, 150 */,
	pub lines: i64 /* 25 */,
	pub selected_mode: ParamsSelectedModeEnum /* 1, 2 */,
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
	pub name: ActionNameEnum /* buy_spin */,
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
	pub autogame: bool /* true */,
	pub command: ClientCommandEnum /* play */,
	pub mobile: String /* 0 */,
	pub portrait: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 295 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "cba5c16b-0ff4-4af2-9c38-a1092bec7128" */,
	pub quick_spin: i64 /* 2 */,
	pub request_id: String /* "a30dde91-5145-4a04-a233-11772e89cf8f" */,
	pub session_id: String /* "54d657fdddea4c76800b216371ea868e" */,
	pub set_denominator: i64 /* 1 */,
	pub sound: bool /* false, true */,
}

impl From<client::Client> for BuySpin {
	fn from(obj: client::Client) -> Self {
		BuySpin {
			action: obj.action.unwrap_or_default().into(),
			autogame: obj.autogame.unwrap_or_default(),
			command: obj.command,
			mobile: obj.mobile.unwrap_or_default(),
			portrait: obj.portrait.unwrap_or_default(),
			prev_client_command_time: obj.prev_client_command_time,
			prev_request_id: obj.prev_request_id,
			quick_spin: obj.quick_spin.unwrap_or_default(),
			request_id: obj.request_id,
			session_id: obj.session_id.unwrap_or_default(),
			set_denominator: obj.set_denominator.unwrap_or_default(),
			sound: obj.sound.unwrap_or_default(),
		}
	}
}

