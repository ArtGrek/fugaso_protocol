use super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::enums::{ModesEnum, CommandsEnum, StatusCodesEnum, CurrenciesEnum, StatusTypesEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Status {
	pub code: StatusCodesEnum /* FUNDS_EXCEED, GAME_REOPENED, OK, PLAYER_DISCONNECTED */,
	#[serde(rename = "type", skip_serializing_if = "Option::is_none")]
	pub status_type: Option<StatusTypesEnum> /* crit, exceed */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub traceback: Option<String> /* NOT_ENOUGH_MONEY */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<i64> /* -1 */,
}

impl From<server::Status> for Status {
	fn from(obj: server::Status) -> Self {
		Status {
			code: obj.code,
			status_type: obj.status_type,
			traceback: obj.traceback,
			user_id: obj.user_id,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
	pub balance: i64 /* 100000 */,
	pub balance_version: i64 /* 1 */,
	pub currency: CurrenciesEnum /* FUN */,
	pub huid: String /* "demo-106758a99a3346fba872f844aa187a8c" */,
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
	pub command: CommandsEnum /* login */,
	pub modes: Vec<ModesEnum> /* auto, freebet, play */,
	pub request_id: String /* "28ae5ee1-a824-4634-8358-bcab13e3ce74" */,
	#[serde(rename = "server-ver")]
	pub server_ver: String /* 1.44.11-9348d0f1 */,
	pub session_id: String /* "04d1923972bc43a9a629302732728d65" */,
	pub status: Status,
	pub user: User,
	#[serde(rename = "user-id")]
	pub user_id: i64 /* -1 */,
}

impl From<server::Server> for Login {
	fn from(obj: server::Server) -> Self {
		Login {
			command: obj.command,
			modes: obj.modes.unwrap_or_default().into_iter().map(Into::into).collect(),
			request_id: obj.request_id,
			server_ver: obj.server_ver.unwrap_or_default(),
			session_id: obj.session_id,
			status: obj.status.into(),
			user: obj.user.unwrap_or_default().into(),
			user_id: obj.user_id.unwrap_or_default(),
		}
	}
}

