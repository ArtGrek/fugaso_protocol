use actix_web::Error;
use super::super::{server, network::request};

//use super::super::enums::{CommandsEnum, StatusCodesEnum, };
use super::super::mock::MockData;

pub async fn execute(_a_request: &request::sync::Sync, _a_game: &mut server::Server, _is_test: bool, _a_mock_data: &MockData, ) -> Result<(), Error> {

Ok(())
}