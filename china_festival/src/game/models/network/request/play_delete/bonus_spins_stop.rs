use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BonusSpinStop {
    pub command: String,
    pub request_id: String,
    pub session_id: String,
    pub action: Action,
    pub set_denominator: i32,
    pub quick_spin: i32,
    pub sound: bool,
    pub autogame: bool,
    pub mobile: String,
    pub portrait: bool,
    pub prev_client_command_time: Option<i32>
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Action {
    pub name: String,
    pub params: Params
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Params {}