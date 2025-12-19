
use std::error::Error;
use std::fs;
use serde_json::Value;
use crate::utils;
use crate::api;
use crate::game::models::{model, business, network::request, network::response};

pub async fn execute(a_command: Option<&str>, a_body: String, a_game: &mut model::Game, expected_response: Option<Value>) -> Result<Value, Box<dyn Error>> {
    match a_command {
        Some("login") => {
            let l_request = serde_json::from_slice::<request::login::Login>(a_body.as_bytes()).map_err(|e|  utils::err_http_responses("BadRequest", &e.to_string(), "ERR_INVALID_BODY"))?; 
            let l_user = api::get_account().await.map_err(|e| utils::err_http_responses("ErrorServiceUnavailable", &e.to_string(), "ERR_SERVICE_UNAVAILABLE"))?;  
            a_game.command = l_request.command.clone();
            a_game.request_id = l_request.request_id.clone();
            a_game.status.code = "OK".to_string();
            a_game.user = l_user;
            let l_response = response::login::Login::from(a_game.clone());
            Ok(serde_json::to_value(l_response)?)
        }
        Some("start") => {
            let l_request = serde_json::from_slice::<request::start::Start>(a_body.as_bytes()).map_err(|e|  utils::err_http_responses("BadRequest", &e.to_string(), "ERR_INVALID_BODY"))?; 
                a_game.command = l_request.command.clone();
                /*a_game.context = */
                a_game.context.actions = ["spin".to_string(), "buy_spin".to_string()].into_iter().map(Into::into).collect();
                a_game.context.current = "spins".to_string();
                a_game.context.last_action = "init".to_string();
                a_game.context.bonus = None;
                a_game.context.spins.round_bet = 300;
                a_game.context.spins.bet_per_line = 15;
                a_game.context.spins.lines = 25;
                a_game.context.spins.bac.field1 = [3, 0];
                a_game.context.spins.bac.field2 = [3, 0];
                a_game.context.spins.bac.field3 = [3, 0];
                a_game.context.spins.board = [[2,2,1],[3,6,6],[9,9,9],[2,8,8],[5,2,2]];
                a_game.context.spins.total_win = Some(0);
                a_game.context.version = 1;
                /* */
                a_game.request_id = l_request.request_id.clone();
                a_game.settings = serde_json::from_str(&fs::read_to_string("../data/games/settings/china_festival.json").map_err(|e| utils::err_http_responses("ErrorInternalServerError", &e.to_string(), "ERR_INTERNAL_ERROR"))?).map_err(|e| utils::err_http_responses("ErrorInternalServerError", &e.to_string(), "ERR_INTERNAL_ERROR"))?;
                /* */
                a_game.settings.jackpots.mini = 15;
                a_game.settings.jackpots.minor = 30;
                a_game.settings.jackpots.major = 100;
                a_game.settings.jackpots.grand = 5000;
                /* */
                a_game.status.code = "OK".to_string();
                a_game.user.balance_version += 1;
                let l_response = response::start::Start::from((a_game).clone());
                
                a_game.context.last_win = Some(0);
                Ok(serde_json::to_value(l_response)?)
        }  
        Some("sync") => {
            let l_request = serde_json::from_slice::<request::sync::Sync>(a_body.as_bytes()).map_err(|e|  utils::err_http_responses("BadRequest", &e.to_string(), "ERR_INVALID_BODY"))?;
                a_game.command = l_request.command.clone();
                a_game.request_id = l_request.request_id.clone();
                a_game.status.code = "OK".to_string();
                let l_response = response::sync::Sync::from((a_game).clone());
                Ok(serde_json::to_value(l_response)?)
        }    
        Some("play") => {
            let l_request = serde_json::from_slice::<request::play::Play>(a_body.as_bytes()).map_err(|e|  utils::err_http_responses("BadRequest", &e.to_string(), "ERR_INVALID_BODY"))?;
                
                match l_request.action.name.as_str() {
                    "spin" => {
                        business::play::spin::execute(&l_request, a_game, expected_response).await.map_err(|e| utils::err_http_responses("ErrorInternalServerError", &e.to_string(), "ERR_INTERNAL_ERROR"))?;
                        let l_response = response::play::spin::Spin::from((a_game).clone());
                        Ok(serde_json::to_value(l_response)?)
                    }
                    "buy_spin" => {
                        business::play::buy_spin::execute(&l_request, a_game, expected_response).await.map_err(|e| utils::err_http_responses("ErrorInternalServerError", &e.to_string(), "ERR_INTERNAL_ERROR"))?;
                        let l_response = response::play::buy_spin::BuySpin::from((a_game).clone());
                        Ok(serde_json::to_value(l_response)?)
                    }
                    "bonus_init" => {
                        business::play::bonus_init::execute(&l_request, a_game, expected_response).await.map_err(|e| utils::err_http_responses("ErrorInternalServerError", &e.to_string(), "ERR_INTERNAL_ERROR"))?;
                        let l_response = response::play::bonus_init::BonusInit::from((a_game).clone());
                        Ok(serde_json::to_value(l_response)?)
                    }
                    "respin" => {
                        business::play::respin::execute(&l_request, a_game, expected_response).await.map_err(|e| utils::err_http_responses("ErrorInternalServerError", &e.to_string(), "ERR_INTERNAL_ERROR"))?;
                        let l_response = response::play::respin::Respin::from((a_game).clone());
                        Ok(serde_json::to_value(l_response)?)
                    }
                    "bonus_spins_stop" => {
                        business::play::bonus_spins_stop::execute(&l_request, a_game, expected_response).await.map_err(|e| utils::err_http_responses("ErrorInternalServerError", &e.to_string(), "ERR_INTERNAL_ERROR"))?;
                        let l_response = response::play::bonus_spins_stop::BonusSpinStop::from((a_game).clone());
                        Ok(serde_json::to_value(l_response)?)
                    }
                    _ => {Err(Box::new(utils::err_http_responses("BadRequest", "Unknown action.", "ERR_UNKNOWN_ACTION")))}
                }
        }  
        Some(_) => {Err(Box::new(utils::err_http_responses("BadRequest", "Invalid command.", "ERR_INVALID_CMD")))}
        None => {Err(Box::new(utils::err_http_responses("BadRequest", "Missing command.", "ERR_MISSING_CMD")))}
    }
}