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
	#[serde(rename = "3")]
	Enum3,
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
	pub bet_per_line: Option<i64> /* 1 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lines: Option<i64> /* 1 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<ParamsSelectedModeEnum> /* 1, 2, 3 */,
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
	pub autogame: Option<bool> /* true */,
	pub command: ClientCommandEnum /* login, play, start, sync */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub game: Option<String> /* coin_lamp */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub huid: Option<String> /* "demo-e55b3f9a0a5f4e42ac8114faaf0d413f" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<String> /* en */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mobile: Option<String> /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mode: Option<String> /* play */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub platform: Option<String> /* mob */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub playerguid: Option<String> /* "4734e5ce-3391-11f0-9fad-ca43f3bffbbc" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub portrait: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 72440 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "3a108dbf-03ac-4575-b08f-a4e2b102e4b8" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub quick_spin: Option<i64> /* 2 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub re_enter: Option<bool> /* false */,
	pub request_id: String /* "62c3fd19-2e34-4905-8519-89e7f4236bc7" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session_id: Option<String> /* "577040c7bf0b4dc18036a41bc4527fb7" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub set_denominator: Option<i64> /* 1 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sound: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub token: Option<String> /* "2e3a948e-1fea-11f0-b666-76cd92dc3233" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub wl: Option<String> /* demo */,
}

