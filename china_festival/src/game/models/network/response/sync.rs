use serde::{Deserialize, Serialize};
use crate::game::models::model;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Sync {
    pub command: String,
    pub modes: Vec<String>,
    pub request_id: String,
    pub session_id: String,
    pub status: Status,
    pub user: User,
}

impl From<model::Game> for Sync {
    fn from(obj: model::Game) -> Self {
        Sync {
            command: obj.command,
            modes: obj.modes,
            request_id: obj.request_id,
            session_id: obj.session_id,
            status: obj.status.into(),
            user: obj.user.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Status {
    pub code: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub status_type: Option<String>
}

impl From<model::Status> for Status {
    fn from(obj: model::Status) -> Self {
        Status {
            code: obj.code,
            status_type: obj.status_type,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct User {
    pub balance: i64,
    pub balance_version: i32,
    pub currency: String,
    pub huid: String,
    pub show_balance: bool,
}

impl From<model::User> for User {
    fn from(obj: model::User) -> Self {
        User {
            balance: obj.balance,
            balance_version: obj.balance_version,
            currency: obj.currency,
            huid: obj.huid,
            show_balance: obj.show_balance,
        }
    }
}