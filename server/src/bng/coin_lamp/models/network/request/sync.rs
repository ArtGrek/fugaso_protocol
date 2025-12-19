use super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::client::{ClientCommandEnum};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Sync {
	pub command: ClientCommandEnum /* sync */,
	pub request_id: String /* "f54ce91b-0bf3-4dc1-9272-6ce999ad1f34" */,
	pub session_id: String /* "577040c7bf0b4dc18036a41bc4527fb7" */,
}

impl From<client::Client> for Sync {
	fn from(obj: client::Client) -> Self {
		Sync {
			command: obj.command,
			request_id: obj.request_id,
			session_id: obj.session_id.unwrap_or_default(),
		}
	}
}

