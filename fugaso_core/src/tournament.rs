use std::sync::Arc;
use sea_orm::prelude::DateTimeWithTimeZone;
use essential_async::channel::OneShotSender;
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentInfo {
    pub current: Option<TournamentState>,
    pub pending_wins: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentState {
    pub name: String,
    #[serde(
        deserialize_with = "crate::protocol::from_milliseconds_str",
        serialize_with = "crate::protocol::serialize_date_time_ms"
    )]
    pub date_start: DateTimeWithTimeZone,
    #[serde(
        deserialize_with = "crate::protocol::from_milliseconds_str",
        serialize_with = "crate::protocol::serialize_date_time_ms"
    )]
    pub date_end: DateTimeWithTimeZone,
    #[serde(with = "rust_decimal::serde::float")]
    pub min_bet: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub min_bet_euro: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub rate: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub share: Decimal,
    #[serde(default = "Vec::new")]
    pub places: Vec<TournamentPlace>,
    #[serde(serialize_with = "crate::protocol::serialize_vec_float")]
    pub rewards: Vec<Decimal>,
    pub position: Option<TournamentPosition>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentPlace {
    pub name: String,
    #[serde(with = "rust_decimal::serde::float")]
    pub balance: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentPosition {
    index: i32,
    balance: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TournamentConfig {
    pub url: String,
    pub ip: Arc<String>,
    pub name: Arc<String>,
    pub password: Arc<String>,
    pub logged: bool,
    pub server: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentIncreaseRequest {
    pub amount: Decimal,
    pub currency: String,
    pub ip: Arc<String>,
    pub stake: Decimal,
    pub tours: Arc<Vec<String>>,
    pub user_name: Arc<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TournamentAuthRequest {
    pub username: Arc<String>,
    pub password: Arc<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct JwtAuthentication {
    pub username: String,
    pub roles: Vec<String>,
    pub expires_in: i64,
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
}

pub enum TournamentEvent {
    Auth(OneShotSender<String>),
    Login,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentCreateAct {
    pub outbound_id: Uuid,
    pub remote_code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentRemoteAct {
    pub id: i64,
    pub remote_code: i32,
}