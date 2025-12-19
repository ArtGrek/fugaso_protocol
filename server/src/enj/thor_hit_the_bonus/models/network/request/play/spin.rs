use super::super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::super::enums::{CommandsEnum, ActionsEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Params {
	pub bet_per_line: i64 /* 20 */,
	pub lines: i64 /* 1 */,
}

impl From<client::Params> for Params {
	fn from(obj: client::Params) -> Self {
		Params {
			bet_per_line: obj.bet_per_line.unwrap_or_default(),
			lines: obj.lines.unwrap_or_default(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Action {
	pub name: ActionsEnum /* spin */,
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
pub struct Spin {
	pub action: Action,
	pub autogame: bool /* false */,
	pub command: CommandsEnum /* play */,
	pub mobile: bool /* false */,
	pub portrait: bool /* false */,
	pub prev_client_command_time: i64 /* 19 */,
	pub quick_spin: i64 /* 2 */,
	pub request_id: String /* "36abc13f56974c1ba4f5957f7e8c98ea" */,
	pub session_id: String /* "17520679221969UH15ouuh3xFUSvXY.EmVaz7x07pImki9byd2v" */,
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
			prev_client_command_time: obj.prev_client_command_time.unwrap_or_default(),
			quick_spin: obj.quick_spin.unwrap_or_default(),
			request_id: obj.request_id,
			session_id: obj.session_id.unwrap_or_default(),
			sound: obj.sound.unwrap_or_default(),
		}
	}
}

