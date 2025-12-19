use actix_web::Error;
use serde_json::Value;
use crate::game::models::{model, network::request};

pub async fn execute(a_request: &request::play::Play, a_game: &mut model::Game, _expected_response: Option<Value>) -> Result<(), Error> {
    a_game.command = a_request.command.clone();

    if a_game.context.actions.contains(&a_request.action.name.clone()) {

        a_game.context.spins.paid = Some(false);
        if a_game.context.spins.bonus_mechanic.get_or_insert_with(Vec::new).contains(&1) {a_game.context.spins.bac.field1 = [0, 0]}
        if a_game.context.spins.bonus_mechanic.get_or_insert_with(Vec::new).contains(&2) {a_game.context.spins.bac.field2 = [0, 0]}
        if a_game.context.spins.bonus_mechanic.get_or_insert_with(Vec::new).contains(&3) {a_game.context.spins.bac.field3 = [0, 0]}
        a_game.context.spins.bac_win = Some(false);
        a_game.context.spins.bonus_mechanic = None;
        a_game.context.spins.total_win = Some(a_game.context.bonus.get_or_insert_with(Default::default).total_win);
        //a_game.context.spins.selected_mode = Some("0".to_string());
        
        if a_game.context.spins.paid == Some(true) {a_game.context.spins.paid = Some(false);}
        a_game.context.spins.winlines = None;
        a_game.context.spins.round_win = 0;
        a_game.context.last_win = Some(a_game.context.bonus.get_or_insert_with(Default::default).total_win);
        a_game.context.spins.selected_mode = None;

        a_game.context.actions = ["spin".to_string(), "buy_spin".to_string()].into_iter().map(Into::into).collect();
        a_game.context.current = "spins".to_string();
        a_game.context.last_action = a_request.action.name.clone();
        a_game.context.round_finished = true;
        a_game.origin_data.feature = true;
        a_game.origin_data.autogame = a_request.autogame;
        a_game.origin_data.mobile = a_request.mobile.clone();
        a_game.origin_data.portrait = a_request.portrait;
        a_game.origin_data.quickspin = a_request.quick_spin;
        a_game.origin_data.set_denominator = a_request.set_denominator;
        a_game.origin_data.sound = a_request.sound;
        a_game.request_id = a_request.request_id.clone();
        a_game.status.code = "OK".to_string();
        a_game.status.status_type = None;
        //a_game.user.balance += a_game.context.spins.total_win.unwrap_or(0);
        a_game.user.balance_version += 1;
        a_game.context.bonus = None;
    } else {
        a_game.request_id = a_request.request_id.clone();
        a_game.status.code = "ACTION_ERROR".to_string(); /* BET_LIMIT_ERROR, FUNDS_EXCEED */
        a_game.status.status_type = Some("crit".to_string()); /* crit, exceed */
    };
    Ok(())
}