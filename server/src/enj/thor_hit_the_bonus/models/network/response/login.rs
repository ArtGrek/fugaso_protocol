use super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::enums::{ModesEnum, StatusCodesEnum, CommandsEnum, StatusTypesEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Status {
	pub code: StatusCodesEnum /* FUNDS_EXCEED, OK */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reason: Option<String> /* Insufficient balance */,
	#[serde(rename = "type", skip_serializing_if = "Option::is_none")]
	pub status_type: Option<StatusTypesEnum> /* exceed */,
}

impl From<server::Status> for Status {
	fn from(obj: server::Status) -> Self {
		Status {
			code: obj.code.into(),
			reason: obj.reason,
			status_type: obj.status_type,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
	pub balance: i64 /* 500000 */,
	pub balance_version: i64 /* 1752067922213 */,
	pub currency: String /* FUN */,
	pub huid: String /* "686e6f522c7c80483b132b54" */,
	pub nick: String /* Player 3a9b80b6-e351-4713-9627-3ad37a961139, Player 481f841a-566d-4f5d-99b7-c085c96e378e, Player 5a8c32af-ffda-4247-8eb5-05f73d3148e9, Player c80b1aac-2423-4f93-9058-3392e18805de */,
	pub show_balance: bool /* true */,
}

impl From<server::User> for User {
	fn from(obj: server::User) -> Self {
		User {
			balance: obj.balance,
			balance_version: obj.balance_version,
			currency: obj.currency,
			huid: obj.huid,
			nick: obj.nick.into(),
			show_balance: obj.show_balance,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Login {
	pub command: CommandsEnum /* login */,
	pub modes: Vec<ModesEnum> /* auto, play */,
	pub request_id: String /* "816eb35963c141d684121e4f5d1557e1" */,
	pub session_id: String /* "17520679221969UH15ouuh3xFUSvXY.EmVaz7x07pImki9byd2v" */,
	pub status: Status,
	pub user: User,
}

impl From<server::Server> for Login {
	fn from(obj: server::Server) -> Self {
		Login {
			command: obj.command,
			modes: obj.modes.into_iter().map(Into::into).collect(),
			request_id: obj.request_id,
			session_id: obj.session_id,
			status: obj.status.into(),
			user: obj.user.into(),
		}
	}
}

