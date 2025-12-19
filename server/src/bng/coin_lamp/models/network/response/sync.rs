use super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::server::{ServerModesEnum, ServerCommandEnum, StatusCodeEnum, };

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
	pub balance: i64 /* 95320 */,
	pub balance_version: i64 /* 242 */,
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
pub struct Sync {
	pub command: ServerCommandEnum /* sync */,
	pub modes: Vec<ServerModesEnum> /* auto, freebet, play */,
	pub request_id: String /* "f54ce91b-0bf3-4dc1-9272-6ce999ad1f34" */,
	pub session_id: String /* "577040c7bf0b4dc18036a41bc4527fb7" */,
	pub status: Status,
	pub user: User,
}

impl From<server::Server> for Sync {
	fn from(obj: server::Server) -> Self {
		Sync {
			command: obj.command,
			modes: obj.modes.unwrap_or_default().into_iter().map(Into::into).collect(),
			request_id: obj.request_id,
			session_id: obj.session_id,
			status: obj.status.into(),
			user: obj.user.unwrap_or_default().into(),
		}
	}
}

