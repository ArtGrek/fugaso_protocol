use actix_web::Error;
use super::super::{server, network::request};

use super::super::enums::{CommandsEnum, StatusCodesEnum, };
use super::super::mock::MockData;

pub async fn execute(a_request: &request::sync::Sync, a_game: &mut server::Server, _is_test: bool, _a_mock_data: &MockData, ) -> Result<(), Error> {
    a_game.command = CommandsEnum::Sync;
    //a_game.context
    //a_game.modes
    a_game.request_id = a_request.request_id.clone();
    // a_game.roundnum
    // a_game.server_ver
    // a_game.session_id
    // a_game.settings
    a_game.status.code = StatusCodesEnum::Ok;
    a_game.status.status_type = None;
    a_game.status.traceback = None;
    a_game.status.user_id = None;
    // a_game.user
    // a_game.user_id

    Ok(())
}