use serde::{Serialize, Deserialize};
use strum_macros::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum ActionNameEnum {
	#[default]
	#[serde(rename = "bonus_init")]
	BonusInit,
	#[serde(rename = "bonus_spins_stop")]
	BonusSpinsStop,
	#[serde(rename = "buy_spin")]
	BuySpin,
	#[serde(rename = "respin")]
	Respin,
	#[serde(rename = "spin")]
	Spin,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum ParamsSelectedModeEnum {
	#[default]
	#[serde(rename = "1")]
	Enum1,
	#[serde(rename = "2")]
	Enum2,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum ClientCommandEnum {
	#[default]
	#[serde(rename = "login")]
	Login,
	#[serde(rename = "play")]
	Play,
	#[serde(rename = "start")]
	Start,
	#[serde(rename = "sync")]
	Sync,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Params {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_factor: Option<i64> /* 20 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_per_line: Option<i64> /* 1, 150 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lines: Option<i64> /* 25 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<ParamsSelectedModeEnum> /* 1, 2 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Action {
	pub name: ActionNameEnum /* bonus_init, bonus_spins_stop, buy_spin, respin, spin */,
	pub params: Params,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Client {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub action: Option<Action>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub autogame: Option<bool> /* false, true */,
	pub command: ClientCommandEnum /* login, play, start, sync */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub game: Option<String> /* china_festival */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub huid: Option<String> /* "demo-88fb2ae9a9fb434c9407a6322c941377" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<String> /* en */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mobile: Option<String> /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mode: Option<String> /* play */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub platform: Option<String> /* mob */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub playerguid: Option<String> /* "d2975c62-3244-11f0-9fad-ca43f3bffbbc" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub portrait: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 303 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "f466e2ec-87c3-4397-8255-b16191d4d88e" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub quick_spin: Option<i64> /* 0, 2 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub re_enter: Option<bool> /* false */,
	pub request_id: String /* "7df5da8e-afbd-4969-9c85-d67386ade182" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session_id: Option<String> /* "54d657fdddea4c76800b216371ea868e" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub set_denominator: Option<i64> /* 1 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sound: Option<bool> /* false, true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub token: Option<String> /* "e5e1a898-19d2-11f0-87d1-1e2ba00d4d9a" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub wl: Option<String> /* demo */,
}

