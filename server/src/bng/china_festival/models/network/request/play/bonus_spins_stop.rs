use super::super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::super::client::{ActionNameEnum, ClientCommandEnum, };

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
	pub name: ActionNameEnum /* bonus_spins_stop */,
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
pub struct BonusSpinsStop {
	pub action: Action,
	pub autogame: bool /* false, true */,
	pub command: ClientCommandEnum /* play */,
	pub mobile: String /* 0 */,
	pub portrait: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 299 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "4aa94234-c494-4974-9345-5f1bee389809" */,
	pub quick_spin: i64 /* 0, 2 */,
	pub request_id: String /* "07cb604b-d222-4c3d-b898-46b5aee013b3" */,
	pub session_id: String /* "54d657fdddea4c76800b216371ea868e" */,
	pub set_denominator: i64 /* 1 */,
	pub sound: bool /* false, true */,
}

impl From<client::Client> for BonusSpinsStop {
	fn from(obj: client::Client) -> Self {
		BonusSpinsStop {
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

