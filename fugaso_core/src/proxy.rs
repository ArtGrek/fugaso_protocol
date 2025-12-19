use std::sync::Arc;

use rust_decimal::Decimal;

use crate::tournament::TournamentPlace;

#[derive(Debug)]
pub struct TournamentTransfer {
    pub balance: i64,
    pub points: Decimal,
    pub place: i32,
    pub name: String,
    pub amount: Decimal,
    pub winners: Arc<Vec<TournamentPlace>>,
}