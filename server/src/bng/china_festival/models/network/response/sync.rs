use super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::server::{ServerModesEnum, StatusCodeEnum, ServerCommandEnum, };

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
	pub balance: i64 /* 100159 */,
	pub balance_version: i64 /* 49 */,
	pub currency: String /* FUN */,
	pub huid: String /* "demo-3113852a4b1b4d7abfc79389f6c42dfb" */,
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
	pub request_id: String /* "439919d2-e849-4373-9ee6-f4e9d49ab91c" */,
	pub session_id: String /* "62d6181745754641878771fefb937415" */,
	pub status: Status,
	pub user: User,
}

impl From<server::Server> for Sync {
	fn from(obj: server::Server) -> Self {
		Sync {
			command: obj.command,
			modes: obj.modes.unwrap_or_default().into_iter().map(Into::into).collect(),
			request_id: obj.request_id,
			session_id: obj.session_id.unwrap_or_default(),
			status: obj.status.into(),
			user: obj.user.unwrap_or_default().into(),
		}
	}
}

