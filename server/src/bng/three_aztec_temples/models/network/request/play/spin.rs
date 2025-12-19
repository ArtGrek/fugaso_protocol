use super::super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::super::enums::{CommandsEnum, ActionsEnum, BonusModesEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Params {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_factor: Option<i64> /* 20 */,
	pub bet_per_line: i64 /* 1 */,
	pub lines: i64 /* 25 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<BonusModesEnum> /* 1, 2 */,
}

impl From<client::Params> for Params {
	fn from(obj: client::Params) -> Self {
		Params {
			bet_factor: obj.bet_factor,
			bet_per_line: obj.bet_per_line.unwrap_or_default(),
			lines: obj.lines.unwrap_or_default(),
			selected_mode: obj.selected_mode.map(Into::into),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Action {
	pub name: ActionsEnum /* buy_spin, spin */,
	pub params: Params,
}

impl From<client::Action> for Action {
	fn from(obj: client::Action) -> Self {
		Action {
			name: obj.name.into(),
			params: obj.params.into(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Spin {
	pub action: Action,
	pub autogame: bool /* true */,
	pub command: CommandsEnum /* play */,
	pub mobile: String /* 0 */,
	pub portrait: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 282 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "ae4106cc-dca2-4910-b2d6-a05f38d71f07" */,
	pub quick_spin: i64 /* 2 */,
	pub request_id: String /* "c63ecae9-2b5e-4e2d-b2d4-324e6a81add5" */,
	pub session_id: String /* "04d1923972bc43a9a629302732728d65" */,
	pub set_denominator: i64 /* 1 */,
	pub sound: bool /* false */,
}

impl From<client::Client> for Spin {
	fn from(obj: client::Client) -> Self {
		Spin {
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

