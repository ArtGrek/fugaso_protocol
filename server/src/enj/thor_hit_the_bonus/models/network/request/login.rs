use super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::enums::{CommandsEnum};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Login {
	pub command: CommandsEnum /* login */,
	pub language: String /* en */,
	pub request_id: String /* "816eb35963c141d684121e4f5d1557e1" */,
	pub token: String /* "537c5edd-428c-4a00-8e3c-41e899ce4166" */,
}

impl From<client::Client> for Login {
	fn from(obj: client::Client) -> Self {
		Login {
			command: obj.command,
			language: obj.language.unwrap_or_default(),
			request_id: obj.request_id,
			token: obj.token.unwrap_or_default(),
		}
	}
}

