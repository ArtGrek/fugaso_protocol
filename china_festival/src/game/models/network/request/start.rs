use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Start {
    pub command: String,
    pub request_id: String,
    pub session_id: String,
    pub mode: String,
    pub huid: String
}