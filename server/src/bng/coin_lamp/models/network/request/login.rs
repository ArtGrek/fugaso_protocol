use super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::client::{ClientCommandEnum};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Login {
	pub command: ClientCommandEnum /* login */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub game: Option<String> /* coin_lamp */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<String> /* en */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub platform: Option<String> /* mob */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub playerguid: Option<String> /* "4734e5ce-3391-11f0-9fad-ca43f3bffbbc" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub re_enter: Option<bool> /* false */,
	pub request_id: String /* "62c3fd19-2e34-4905-8519-89e7f4236bc7" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub token: Option<String> /* "2e3a948e-1fea-11f0-b666-76cd92dc3233" */,
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

