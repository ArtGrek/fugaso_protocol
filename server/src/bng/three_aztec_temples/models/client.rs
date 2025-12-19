use serde::{Serialize, Deserialize};
/*use strum_macros::Display;


#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum CommandsEnum {
	#[default]
	#[serde(rename = "login")]
	Login,
	#[serde(rename = "start")]
	Start,
	#[serde(rename = "play")]
	Play,
	#[serde(rename = "sync")]
	Sync,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum ActionsEnum {
	#[default]
	#[serde(rename = "spin")]
	Spin,
	#[serde(rename = "buy_spin")]
	BuySpin,
	#[serde(rename = "bonus_init")]
	BonusInit,
	#[serde(rename = "respin")]
	Respin,
	#[serde(rename = "bonus_spins_stop")]
	BonusSpinsStop,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum BonusModesEnum {
	#[default]
	#[serde(rename = "0")]
	Enum0,
	#[serde(rename = "1")]
	Enum1,
	#[serde(rename = "2")]
	Enum2,
}*/

use super::enums::{BonusModesEnum, ActionsEnum, CommandsEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Params {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_factor: Option<i64> /* 20 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_per_line: Option<i64> /* 1 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lines: Option<i64> /* 25 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<BonusModesEnum> /* 1, 2 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Action {
	pub name: ActionsEnum /* bonus_init, bonus_spins_stop, buy_spin, respin, spin */,
	pub params: Params,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Client {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub action: Option<Action>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub autogame: Option<bool> /* true */,
	pub command: CommandsEnum /* login, play, start, sync */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub game: Option<String> /* 3_aztec_temples */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub huid: Option<String> /* "demo-106758a99a3346fba872f844aa187a8c" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<String> /* en */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mobile: Option<String> /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mode: Option<String> /* play */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub platform: Option<String> /* mob */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub playerguid: Option<String> /* "9a170a3a-32b3-11f0-9fad-ca43f3bffbbc" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub portrait: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 282 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "e170bddc-d869-4a89-b09a-fbbb98903167" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub quick_spin: Option<i64> /* 2 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub re_enter: Option<bool> /* false */,
	pub request_id: String /* "28ae5ee1-a824-4634-8358-bcab13e3ce74" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session_id: Option<String> /* "04d1923972bc43a9a629302732728d65" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub set_denominator: Option<i64> /* 1 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sound: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub token: Option<String> /* "f5a31d9c-1f50-11f0-b666-76cd92dc3233" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub wl: Option<String> /* demo */,
}

