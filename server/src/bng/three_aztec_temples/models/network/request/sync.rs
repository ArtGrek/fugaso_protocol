use super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::enums::{CommandsEnum};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Sync {
	pub command: CommandsEnum /* sync */,
	pub request_id: String /* "41d4764e-070d-41ce-bcc3-489b55afad15" */,
	pub session_id: String /* "04d1923972bc43a9a629302732728d65" */,
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

