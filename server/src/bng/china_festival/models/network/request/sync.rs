use super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::client::{ClientCommandEnum};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Sync {
	pub command: ClientCommandEnum /* sync */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 319 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "83893fce-c22d-40ae-8f91-6de696697669" */,
	pub request_id: String /* "439919d2-e849-4373-9ee6-f4e9d49ab91c" */,
	pub session_id: String /* "62d6181745754641878771fefb937415" */,
}

impl From<client::Client> for Sync {
	fn from(obj: client::Client) -> Self {
		Sync {
			command: obj.command,
			prev_client_command_time: obj.prev_client_command_time,
			prev_request_id: obj.prev_request_id,
			request_id: obj.request_id,
			session_id: obj.session_id.unwrap_or_default(),
		}
	}
}

