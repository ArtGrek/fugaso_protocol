
use serde::{Deserialize, Serialize, };
use std::collections::HashMap;
use essential_core::error::ServerError;
use essential_core::err_on;
use crate::protocol::DatabaseStore;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewGameNameLinkInfo {
    #[serde(default)]
    pub total: i64,
    pub respins: i32,
    #[serde(default)]
    pub accum: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay: Option<Vec<Vec<char>>>,
}

impl DatabaseStore for NewGameNameLinkInfo {
    fn from_db(value: &str) -> Result<Self, ServerError> {serde_json::from_str(&value).map_err(|e| err_on!(e))}
    fn to_db(&self) -> Result<String, ServerError> {serde_json::to_string(self).map_err(|e| err_on!(e))}
    fn respins(&self) -> i32 {self.respins}
}

use strum_macros::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default, PartialEq)]
pub enum CommandEnum {
	#[default]
	#[serde(rename = "login")]
	Login,
	#[serde(rename = "play")]
	Play,
	#[serde(rename = "start")]
	Start,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PaytableEntry {
	pub multiplier: i64 /* 10, 30, 100 */,
	pub occurrences: i64 /* 3, 4, 5 */,
	#[serde(rename = "type")]
	pub paytable_elem_type: String /* lb */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Settings {
	pub bet_factor: Vec<i64> /* [10] */,
	pub bets: Vec<i64> /* [1,2,3,4,5,8,10,15,20,30,40,50,75,100,200,300,500] */,
	pub cols: i64 /* 5 */,
	pub lines: Vec<i64> /* [5] */,
	pub paylines: Vec<Vec<i64>> /* [[1,1,1,1,1],[0,0,0,0,0],[2,2,2,2,2],[0,1,2,1,0],[2,1,0,1,2]] */,
    pub paytable: HashMap<String, Vec<PaytableEntry>>,
	pub rows: i64 /* 3 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct OriginalGameNameOut {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub settings: Option<Settings>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct OriginalGameNameIn {
	pub command: CommandEnum /* login, play, start */,
}