use actix_web::Error;
use std::fs;
use crate::http_errors::http_responses_error;
use super::super::{server, network::request};
use super::super::enums::{ModesEnum, CommandsEnum, StatusCodesEnum, CurrenciesEnum, };
use super::super::mock::MockData;

pub async fn execute(a_request: &request::login::Login, a_game: &mut server::Server, is_test: bool, a_mock_data: &MockData, ) -> Result<(), Error> {
    a_game.command = CommandsEnum::Login;
    a_game.context = None;
    a_game.roundnum = None;
    a_game.server_ver = Some("1.44.11-9348d0f1".to_string());
    a_game.modes = Some(vec![ModesEnum::Auto, ModesEnum::Play, ModesEnum::Freebet]);
    a_game.settings = serde_json::from_str(&fs::read_to_string("../data/three_aztec_temples/settings/settings.json").map_err(|e| http_responses_error("ErrorInternalServerError", &e.to_string(), "ERR_INTERNAL_ERROR"))?).map_err(|e| http_responses_error("ErrorInternalServerError", &e.to_string(), "ERR_INTERNAL_ERROR"))?;
    // a_game.session_id
    a_game.request_id = a_request.request_id.clone();
    a_game.status.code = StatusCodesEnum::Ok;
    a_game.status.status_type = None;
    a_game.status.traceback = None;
    a_game.status.user_id = None;
    let l_user = a_game.user.get_or_insert_with(Default::default);
    l_user.balance = 100_000;
    l_user.balance_version = 2;
    l_user.currency = CurrenciesEnum::Fun;
    l_user.huid = if !is_test {a_request.playerguid.clone().unwrap_or_default()} else {a_mock_data.huid.clone().unwrap_or_default()};
    l_user.show_balance = true;
    a_game.user_id = Some(-1);
    a_game.reels = serde_json::from_str(&fs::read_to_string("../data/three_aztec_temples/reels/reels.json").map_err(|e| http_responses_error("ErrorInternalServerError", &e.to_string(), "ERR_INTERNAL_ERROR"))?).map_err(|e| http_responses_error("ErrorInternalServerError", &e.to_string(), "ERR_INTERNAL_ERROR"))?;
    a_game.reels_buy1 = serde_json::from_str(&fs::read_to_string("../data/three_aztec_temples/reels/reels_buy1.json").map_err(|e| http_responses_error("ErrorInternalServerError", &e.to_string(), "ERR_INTERNAL_ERROR"))?).map_err(|e| http_responses_error("ErrorInternalServerError", &e.to_string(), "ERR_INTERNAL_ERROR"))?;
    a_game.reels_buy2 = serde_json::from_str(&fs::read_to_string("../data/three_aztec_temples/reels/reels_buy2.json").map_err(|e| http_responses_error("ErrorInternalServerError", &e.to_string(), "ERR_INTERNAL_ERROR"))?).map_err(|e| http_responses_error("ErrorInternalServerError", &e.to_string(), "ERR_INTERNAL_ERROR"))?;
    

    Ok(())
}