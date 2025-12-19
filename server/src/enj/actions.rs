use actix_web::{web, HttpResponse, Result, Error};
use serde_json::Value;
use uuid::Uuid;
use crate::http_errors::http_responses_error;
use crate::games_list::{LockedList, Games};
use super::thor_hit_the_bonus;

pub async fn execute(a_body: web::Bytes, a_shared_game_list: web::Data<LockedList<Games>>, is_test: bool, a_mock_data: &thor_hit_the_bonus::models::mock::MockData, ) -> Result<HttpResponse, Error> {
    let json: Value = serde_json::from_slice(&a_body).map_err(|e| http_responses_error("BadRequest", &e.to_string(), "ERR_INVALID_BODY"))?;
    let l_command: &str =  json.get("command").and_then(|value| value.as_str()).ok_or(http_responses_error("BadRequest", "Command is missing.", "ERR_COMMAND_MISSING"))?;
    let l_response: Value;
    if l_command == "login" {
        let l_player_id =  json.get("playerguid").and_then(|value| value.as_str()).ok_or(http_responses_error("BadRequest", "Player id is missing.", "ERR_PLAYERID_MISSING"))?.to_string();
        let l_game_id =  json.get("game").and_then(|value| value.as_str()).ok_or(http_responses_error("BadRequest", "Game id is missing.", "ERR_GAMEID_MISSING"))?;
        let l_session_id = if !is_test {Uuid::new_v5(&Uuid::new_v5(&Uuid::parse_str("0797cb7a-1452-4407-a561-bf4f0f4fb2b1").unwrap(),l_game_id.as_bytes()), l_player_id.as_bytes()).to_string()} else {a_mock_data.session_id.clone().unwrap_or_default()};
        if let Some(l_arc_mutex_game) = a_shared_game_list.get_by_player_game_id(&(l_player_id.clone() + l_game_id), l_session_id.clone()).await {
            let mut l_lock_game = l_arc_mutex_game.lock().await;
            l_response = match &mut *l_lock_game {
                Games::ThorHitTheBonus(l_extracted_game) => {let l_request = serde_json::from_slice::<thor_hit_the_bonus::models::client::Client>(&a_body).map_err(|e|  http_responses_error("BadRequest", &e.to_string(), "ERR_INVALID_BODY"))?; 
                    thor_hit_the_bonus::actions::execute(&l_request, &mut *l_extracted_game, is_test, a_mock_data).await?
                },
                _ => {return Err(http_responses_error("ErrorInternalServerError", "Game not implement.", "ERR_NOT_IMPLEMENTED"))}
            };
        } else {
            let l_game: Games = match l_game_id {
                "3_aztec_temples" => {
                    let l_request = serde_json::from_slice::<thor_hit_the_bonus::models::client::Client>(&a_body).map_err(|e|  http_responses_error("BadRequest", &e.to_string(), "ERR_INVALID_BODY"))?; 
                    let mut new_game = thor_hit_the_bonus::models::server::Server::new();
                    new_game.session_id = l_session_id.clone();
                    l_response = thor_hit_the_bonus::actions::execute(&l_request, &mut new_game, is_test, a_mock_data).await?;
                    Games::ThorHitTheBonus(new_game)
                },
                _ => {return Err(http_responses_error("ErrorInternalServerError", "Game not implement.", "ERR_NOT_IMPLEMENTED"))}
            }; 
            a_shared_game_list.insert(l_player_id.clone() + l_game_id, l_session_id, l_game).await;
        }
    } else {
        let l_session_id = json.get("session_id").and_then(|value| value.as_str()).ok_or(http_responses_error("BadRequest", "Session id is missing.", "ERR_SESSIONID_MISSING"))?.to_string();
        if let Some(l_arc_mutex_game) = a_shared_game_list.get_by_session_id(&l_session_id).await {
            let mut l_lock_game = l_arc_mutex_game.lock().await;
            l_response = match &mut *l_lock_game {
                Games::ThorHitTheBonus(l_extracted_game) => {let l_request = serde_json::from_slice::<thor_hit_the_bonus::models::client::Client>(&a_body).map_err(|e|  http_responses_error("BadRequest", &e.to_string(), "ERR_INVALID_BODY"))?; 
                    thor_hit_the_bonus::actions::execute(&l_request, &mut *l_extracted_game, is_test, a_mock_data).await?
                },
                _ => {return Err(http_responses_error("ErrorInternalServerError", "Game not implement.", "ERR_NOT_IMPLEMENTED"))}
            };
        } else {return Err(http_responses_error("ErrorUnauthorized", "Session expired. Please log in again.", "ERR_UNAUTHORIZED"))}
    }
    Ok(HttpResponse::Ok().json(l_response))
}