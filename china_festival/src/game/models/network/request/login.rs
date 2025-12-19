use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Login {
    pub command: String,
    pub request_id: String,
    pub token: String,
    pub language: String
}