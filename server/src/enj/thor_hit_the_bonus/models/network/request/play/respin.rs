use super::super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::super::enums::{ActionsEnum, CommandsEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Params {
}

impl From<client::Params> for Params {
	fn from(_obj: client::Params) -> Self {
		Params {
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Action {
	pub name: ActionsEnum /* respin */,
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
pub struct Respin {
	pub action: Action,
	pub autogame: bool /* false */,
	pub command: CommandsEnum /* play */,
	pub mobile: bool /* false */,
	pub portrait: bool /* false */,
	pub prev_client_command_time: i64 /* 147 */,
	pub quick_spin: i64 /* 2 */,
	pub request_id: String /* "3c259adc1e3744bdb6dccdaab7c43db4" */,
	pub session_id: String /* "17520679221969UH15ouuh3xFUSvXY.EmVaz7x07pImki9byd2v" */,
	pub sound: bool /* false */,
}

impl From<client::Client> for Respin {
	fn from(obj: client::Client) -> Self {
		Respin {
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

