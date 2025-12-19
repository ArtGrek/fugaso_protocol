use super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::server::{ServerModesEnum, ServerCommandEnum, StatusCodeEnum};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Status {
	pub code: StatusCodeEnum /* OK */,
}

impl From<server::Status> for Status {
	fn from(obj: server::Status) -> Self {
		Status {
			code: obj.code,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
	pub balance: i64 /* 100000 */,
	pub balance_version: i64 /* 1 */,
	pub currency: String /* FUN */,
	pub huid: String /* "demo-e55b3f9a0a5f4e42ac8114faaf0d413f" */,
	pub show_balance: bool /* true */,
}

impl From<server::User> for User {
	fn from(obj: server::User) -> Self {
		User {
			balance: obj.balance,
			balance_version: obj.balance_version,
			currency: obj.currency,
			huid: obj.huid,
			show_balance: obj.show_balance,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Login {
	pub command: ServerCommandEnum /* login */,
	pub modes: Vec<ServerModesEnum> /* auto, freebet, play */,
	pub request_id: String /* "62c3fd19-2e34-4905-8519-89e7f4236bc7" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub server_ver: Option<String> /* 1.44.11-9348d0f1 */,
	pub session_id: String /* "577040c7bf0b4dc18036a41bc4527fb7" */,
	pub status: Status,
	pub user: User,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<i64> /* -1 */,
}

impl From<server::Server> for Login {
	fn from(obj: server::Server) -> Self {
		Login {
			command: obj.command,
			modes: obj.modes.unwrap_or_default().into_iter().map(Into::into).collect(),
			request_id: obj.request_id,
			server_ver: obj.server_ver,
			session_id: obj.session_id,
			status: obj.status.into(),
			user: obj.user.unwrap_or_default().into(),
			user_id: obj.user_id,
		}
	}
}

