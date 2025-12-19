use super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::client::{ClientCommandEnum};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Login {
	pub command: ClientCommandEnum /* login */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub game: Option<String> /* china_festival */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<String> /* en */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub platform: Option<String> /* mob */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub playerguid: Option<String> /* "d2975c62-3244-11f0-9fad-ca43f3bffbbc" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub re_enter: Option<bool> /* false */,
	pub request_id: String /* "7df5da8e-afbd-4969-9c85-d67386ade182" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub token: Option<String> /* "e5e1a898-19d2-11f0-87d1-1e2ba00d4d9a" */,
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

