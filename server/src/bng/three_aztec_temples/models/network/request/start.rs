use super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::enums::{CommandsEnum};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Start {
	pub command: CommandsEnum /* start */,
	pub huid: String /* "demo-106758a99a3346fba872f844aa187a8c" */,
	pub mode: String /* play */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 1217 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "e170bddc-d869-4a89-b09a-fbbb98903167" */,
	pub request_id: String /* "7577bb90-9cbe-4f6e-bfaa-19a58c94e61a" */,
	pub session_id: String /* "04d1923972bc43a9a629302732728d65" */,
}

impl From<client::Client> for Start {
	fn from(obj: client::Client) -> Self {
		Start {
			command: obj.command,
			huid: obj.huid.unwrap_or_default(),
			mode: obj.mode.unwrap_or_default(),
			prev_client_command_time: obj.prev_client_command_time,
			prev_request_id: obj.prev_request_id,
			request_id: obj.request_id,
			session_id: obj.session_id.unwrap_or_default(),
		}
	}
}

