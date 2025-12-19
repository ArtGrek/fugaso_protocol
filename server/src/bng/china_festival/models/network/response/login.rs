use super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::server::{ServerModesEnum, StatusCodeEnum, ServerCommandEnum, StatusTypeEnum, StatusTracebackEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Status {
	pub code: StatusCodeEnum /* OK, OTHER_ERROR */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub traceback: Option<StatusTracebackEnum> /* crit (0) PlayerGUID is empty on connect [parsePlayerGUID(/opt/source/server/modules/common_server/cm/TCmConnect.cpp:452)] */,
	#[serde(rename = "type", skip_serializing_if = "Option::is_none")]
	pub status_type: Option<StatusTypeEnum> /* crit */,
}

impl From<server::Status> for Status {
	fn from(obj: server::Status) -> Self {
		Status {
			code: obj.code.into(),
			traceback: obj.traceback,
			status_type: obj.status_type,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
	pub balance: i64 /* 100000 */,
	pub balance_version: i64 /* 1 */,
	pub currency: String /* FUN */,
	pub huid: String /* "demo-88fb2ae9a9fb434c9407a6322c941377" */,
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
	pub request_id: String /* "7df5da8e-afbd-4969-9c85-d67386ade182" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub server_ver: Option<String> /* 1.44.11-9348d0f1 */,
	pub session_id: String /* "54d657fdddea4c76800b216371ea868e" */,
	pub status: Status,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<User>,
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
			session_id: obj.session_id.unwrap_or_default(),
			status: obj.status.into(),
			user: obj.user.map(Into::into),
			user_id: obj.user_id,
		}
	}
}

