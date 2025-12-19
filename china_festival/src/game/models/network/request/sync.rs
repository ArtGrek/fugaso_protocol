use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Sync {
    pub command: String,
    pub request_id: String,
    pub session_id: String,
    pub prev_client_command_time: Option<i32>
}