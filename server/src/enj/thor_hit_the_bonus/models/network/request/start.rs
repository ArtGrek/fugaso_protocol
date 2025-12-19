use super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::enums::{CommandsEnum};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Start {
	pub command: CommandsEnum /* start */,
	pub huid: String /* "686e6f522c7c80483b132b54" */,
	pub mode: String /* play */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 139 */,
	pub request_id: String /* "e144a8b71e4b4438a30d71dbbebe6c8e" */,
	pub session_id: String /* "17520679221969UH15ouuh3xFUSvXY.EmVaz7x07pImki9byd2v" */,
}

impl From<client::Client> for Start {
	fn from(obj: client::Client) -> Self {
		Start {
			command: obj.command,
			huid: obj.huid.unwrap_or_default(),
			mode: obj.mode.unwrap_or_default(),
			prev_client_command_time: obj.prev_client_command_time,
			request_id: obj.request_id,
			session_id: obj.session_id.unwrap_or_default(),
		}
	}
}

