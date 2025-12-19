use actix_web::Error;
//use crate::ROUND_ID_COUNTER;
//use super::super::super::super::settings::{BOARD_HEIGHT, BOARD_WIDTH, LINES_COUNT, COIN, BOOST, COLLECT, MULTI, SPECIALS, };
use super::super::super::{server, network::request};
//use super::super::super::enums::{CurrentActionsEnum, ActionsEnum, StatusCodesEnum, MultiValueEnum, BonusModesEnum, };
use super::super::super::mock::MockData;

pub async fn execute(_a_request: &request::play::buy_spin::BuySpin, _a_game: &mut server::Server, _is_test: bool, _a_mock_data: &MockData) -> Result<(), Error> {

Ok(())
}