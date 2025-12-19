use super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::client::{ClientCommandEnum};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Start {
	pub command: ClientCommandEnum /* start */,
	pub huid: String /* "demo-e55b3f9a0a5f4e42ac8114faaf0d413f" */,
	pub mode: String /* play */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 1508 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "3a108dbf-03ac-4575-b08f-a4e2b102e4b8" */,
	pub request_id: String /* "85e5a638-611a-467e-a7b1-9c9d104b0041" */,
	pub session_id: String /* "577040c7bf0b4dc18036a41bc4527fb7" */,
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

