use actix_web::Error;
use super::super::super::{server, network::request};

//use super::super::super::enums::{CurrentActionsEnum, ActionsEnum, StatusCodesEnum, MultiValueEnum, };
use super::super::super::mock::MockData;

pub async fn execute(_a_request: &request::play::bonus_spins_stop::BonusSpinsStop, _a_game: &mut server::Server, _is_test: bool, _a_mock_data: &MockData, ) -> Result<(), Error> {

Ok(())
}