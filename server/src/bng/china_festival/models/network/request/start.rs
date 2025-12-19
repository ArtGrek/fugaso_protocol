use super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::client::{ClientCommandEnum};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Start {
	pub command: ClientCommandEnum /* start */,
	pub huid: String /* "demo-88fb2ae9a9fb434c9407a6322c941377" */,
	pub mode: String /* play */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 1244 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "f466e2ec-87c3-4397-8255-b16191d4d88e" */,
	pub request_id: String /* "b6d6863b-8c46-45bf-b000-1c45f35a6a3c" */,
	pub session_id: String /* "54d657fdddea4c76800b216371ea868e" */,
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

