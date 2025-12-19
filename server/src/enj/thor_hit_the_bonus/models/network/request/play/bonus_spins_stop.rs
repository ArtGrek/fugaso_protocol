use super::super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::super::enums::{ActionsEnum, CommandsEnum, BonusModesEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Params {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_factor: Option<i64> /* 10 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_per_line: Option<i64> /* 20 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lines: Option<i64> /* 1 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<BonusModesEnum> /* 1, 2, 3 */,
}

impl From<client::Params> for Params {
	fn from(obj: client::Params) -> Self {
		Params {
			bet_factor: obj.bet_factor,
			bet_per_line: obj.bet_per_line,
			lines: obj.lines,
			selected_mode: obj.selected_mode,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Action {
	pub name: ActionsEnum /* bonus_spins_stop, buy_spin */,
	pub params: Params,
}

impl From<client::Action> for Action {
	fn from(obj: client::Action) -> Self {
		Action {
			name: obj.name.into(),
			params: obj.params.into(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BonusSpinsStop {
	pub action: Action,
	pub autogame: bool /* false */,
	pub command: CommandsEnum /* play */,
	pub mobile: bool /* false */,
	pub portrait: bool /* false */,
	pub prev_client_command_time: i64 /* 155 */,
	pub quick_spin: i64 /* 2 */,
	pub request_id: String /* "de70abbd38cf4a678bd313fe15383683" */,
	pub session_id: String /* "17520679221969UH15ouuh3xFUSvXY.EmVaz7x07pImki9byd2v" */,
	pub sound: bool /* false */,
}

impl From<client::Client> for BonusSpinsStop {
	fn from(obj: client::Client) -> Self {
		BonusSpinsStop {
			action: obj.action.unwrap_or_default().into(),
			autogame: obj.autogame.unwrap_or_default(),
			command: obj.command,
			mobile: obj.mobile.unwrap_or_default(),
			portrait: obj.portrait.unwrap_or_default(),
			prev_client_command_time: obj.prev_client_command_time.unwrap_or_default(),
			quick_spin: obj.quick_spin.unwrap_or_default(),
			request_id: obj.request_id,
			session_id: obj.session_id.unwrap_or_default(),
			sound: obj.sound.unwrap_or_default(),
		}
	}
}

