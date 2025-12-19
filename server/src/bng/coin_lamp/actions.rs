use std::error::Error;
use serde_json::Value;
use crate::http_errors::http_responses_error;
use super::models::{client, server, business, network::request, network::response};

pub async fn execute(a_request: &client::Client, a_game: &mut server::Server, ) -> Result<Value, Box<dyn Error>> {
    match a_request.command {
        client::ClientCommandEnum::Login => {
            business::login::execute(&request::login::Login::from(a_request.clone()), a_game).await?;
            let l_response = response::login::Login::from(a_game.clone());
            Ok(serde_json::to_value(l_response)?)
        }
        client::ClientCommandEnum::Start => {
            business::start::execute(&request::start::Start::from(a_request.clone()), a_game).await?;
            let l_response = response::start::Start::from((a_game).clone());
            Ok(serde_json::to_value(l_response)?)
        }  
        client::ClientCommandEnum::Sync => {
            business::sync::execute(&request::sync::Sync::from(a_request.clone()), a_game).await?;
            let l_response = response::sync::Sync::from((a_game).clone());
            Ok(serde_json::to_value(l_response)?)
        }    
        client::ClientCommandEnum::Play => {
            match a_request.action.clone().ok_or_else(|| {Box::new(http_responses_error("BadRequest", "Missing action.", "ERR_MISSING_ACTION"))})?.name {
                client::ActionNameEnum::Spin => {
                    business::play::spin::execute(&request::play::spin::Spin::from(a_request.clone()), a_game).await?;
                    let l_response = response::play::spin::Spin::from((a_game).clone());
                    Ok(serde_json::to_value(l_response)?)
                }
                client::ActionNameEnum::BuySpin => {
                    business::play::buy_spin::execute(&request::play::buy_spin::BuySpin::from(a_request.clone()), a_game).await?;
                    let l_response = response::play::buy_spin::BuySpin::from((a_game).clone());
                    Ok(serde_json::to_value(l_response)?)
                }
                client::ActionNameEnum::BonusInit => {
                    business::play::bonus_init::execute(&request::play::bonus_init::BonusInit::from(a_request.clone()), a_game).await?;
                    let l_response = response::play::bonus_init::BonusInit::from((a_game).clone());
                    Ok(serde_json::to_value(l_response)?)
                }
                client::ActionNameEnum::Respin => {
                    business::play::respin::execute(&request::play::respin::Respin::from(a_request.clone()), a_game).await?;
                    let l_response = response::play::respin::Respin::from((a_game).clone());
                    Ok(serde_json::to_value(l_response)?)
                }
                client::ActionNameEnum::BonusSpinsStop => {
                    business::play::bonus_spins_stop::execute(&request::play::bonus_spins_stop::BonusSpinsStop::from((a_request).clone()), a_game).await?;
                    let l_response = response::play::bonus_spins_stop::BonusSpinsStop::from((a_game).clone());
                    Ok(serde_json::to_value(l_response)?)
                }
            }
        }
    }
}