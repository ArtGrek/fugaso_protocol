
use serde::{Deserialize, Serialize, };
use essential_core::error::ServerError;
use essential_core::err_on;
use crate::protocol::DatabaseStore;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Lift {
    pub pos: (usize, usize),
    pub mult: i32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangeItem {
    pub p: (usize, usize),
    pub v: i32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grid: Vec<Vec<char>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub collects: Vec<ChangeItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lift_new: Vec<Lift>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mults: Vec<Vec<i32>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mults1: Vec<Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ZeusExtremeLinkInfo {
    #[serde(default)]
    pub total: i64,
    pub respins: i32,
    #[serde(default)]
    pub accum: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<Step>,
}

impl DatabaseStore for ZeusExtremeLinkInfo {
    fn from_db(value: &str) -> Result<Self, ServerError> {serde_json::from_str(&value).map_err(|e| err_on!(e))}
    fn to_db(&self) -> Result<String, ServerError> {serde_json::to_string(self).map_err(|e| err_on!(e))}
    fn respins(&self) -> i32 {self.respins}
}