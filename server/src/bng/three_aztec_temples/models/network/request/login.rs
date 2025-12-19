use super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::enums::{CommandsEnum};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Login {
	pub command: CommandsEnum /* login */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub game: Option<String> /* 3_aztec_temples */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<String> /* en */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub platform: Option<String> /* mob */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub playerguid: Option<String> /* "9a170a3a-32b3-11f0-9fad-ca43f3bffbbc" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub re_enter: Option<bool> /* false */,
	pub request_id: String /* "28ae5ee1-a824-4634-8358-bcab13e3ce74" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub token: Option<String> /* "f5a31d9c-1f50-11f0-b666-76cd92dc3233" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub wl: Option<String> /* demo */,
}

impl From<client::Client> for Login {
	fn from(obj: client::Client) -> Self {
		Login {
			command: obj.command,
			game: obj.game,
			language: obj.language,
			platform: obj.platform,
			playerguid: obj.playerguid,
			re_enter: obj.re_enter,
			request_id: obj.request_id,
			token: obj.token,
			wl: obj.wl,
		}
	}
}

