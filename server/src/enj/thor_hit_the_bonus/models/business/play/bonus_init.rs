use actix_web::Error;
//use crate::ROUND_ID_COUNTER;
//use crate::bng::three_aztec_temples::settings::{BOARD_HEIGHT, BOARD_WIDTH, JACKPOTS_VALUE, JACKPOTS_STR, COIN, BOOST, COLLECT, MULTI, BOOST_STR, MULTI_STR, SPECIALS, };
use super::super::super::{server, network::request};
//use crate::bng::three_aztec_temples::models::server::{BoostValues, CollectValues, MultiValues};
//use super::super::super::enums::{CurrentActionsEnum, ActionsEnum, StatusCodesEnum, MultiValueEnum, BonusModesEnum, };
use super::super::super::mock::MockData;

pub async fn execute(_a_request: &request::play::bonus_init::BonusInit, _a_game: &mut server::Server, _is_test: bool, _a_mock_data: &MockData, ) -> Result<(), Error> {

Ok(())
}