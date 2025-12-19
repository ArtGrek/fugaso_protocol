use essential_core::error::ServerError;
use fugaso_data::fugaso_action::{self, ActionKind};
use serde::{Deserialize, Serialize};

use crate::protocol::{FreeGame, Promo};

pub trait IPlayResponse {
    fn create_action_default(&self) -> Result<fugaso_action::Model, ServerError>;

    fn free(&self) -> Option<&FreeGame>;

    fn has_bonus(&self) -> bool;

    fn has_respin(&self) -> bool;

    fn has_drop(&self) -> bool;

    fn total(&self) -> i64;

    fn respins(&self) -> i32 {
        0
    }

    fn is_gamble_end(&self, total_bet: i64) -> bool;

    fn stops_on(&self) -> Vec<usize>;

    fn grid_on(&self) -> Vec<Vec<char>>;

    fn promo(&self) -> Promo;

    fn set_next_act(&mut self, kind: ActionKind);
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub bet: i32,
    pub line: usize,
    pub denom: i32,
    #[serde(default)]
    pub bet_index: usize,
    #[serde(default)]
    pub bet_counter: usize,
    #[serde(default)]
    pub reels: usize,
}