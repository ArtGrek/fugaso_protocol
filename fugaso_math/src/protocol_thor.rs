
use serde::{Deserialize, Serialize, };
use essential_core::error::ServerError;
use essential_core::err_on;
use crate::protocol::DatabaseStore;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Lift {
    pub pos: (usize, usize),
    pub mult: i32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RandItem {
    pub p: (usize, usize),
    pub s: char,
    pub v: i32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangeItem {
    pub p: (usize, usize),
    pub v: i32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WrathStuck {
    pub p: (usize, usize),
    pub c: i32,
    pub s: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OlympusManiaInfo {
    #[serde(default)]
    pub total: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mults1: Vec<Vec<i32>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mults0: Vec<Vec<i32>>,
    pub respins: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay: Option<Vec<Vec<char>>>,
    #[serde(default)]
    pub accum: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lift: Vec<Vec<i32>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lift_new: Vec<Lift>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grid0: Vec<Vec<char>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rands: Vec<RandItem>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub stuck_accums: HashMap<i32, Vec<WrathStuck>>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub stuck_accums_buy: HashMap<i32, Vec<WrathStuck>>,
     #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub collects: Vec<ChangeItem>,
}

impl DatabaseStore for OlympusManiaInfo {
    fn from_db(value: &str) -> Result<Self, ServerError> {
        serde_json::from_str(&value).map_err(|e| err_on!(e))
    }

    fn to_db(&self) -> Result<String, ServerError> {
        serde_json::to_string(self).map_err(|e| err_on!(e))
    }

    fn respins(&self) -> i32 {
        self.respins
    }
}
