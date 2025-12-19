use actix_web::Error;
use crate::bng::three_aztec_temples::settings::{ COLLECT, SPECIALS, };
use super::super::super::{server, network::request};

use super::super::super::enums::{CurrentActionsEnum, ActionsEnum, StatusCodesEnum, StatusTypesEnum, MultiValueEnum, };
use super::super::super::mock::MockData;

pub async fn execute(a_request: &request::play::bonus_spins_stop::BonusSpinsStop, a_game: &mut server::Server, _is_test: bool, _a_mock_data: &MockData, ) -> Result<(), Error> {
    a_game.command = a_request.command.clone();
    a_game.request_id = a_request.request_id.clone();
    if let Some(ref mut l_context) = a_game.context {
        if l_context.actions.contains(&a_request.action.name) {
            // crutch start
            if let Some((x, y, symbol)) = l_context.spins.board.iter().enumerate().flat_map(|(x, row)| row.iter().enumerate().map(move |(y, &symbol)| (x, y, symbol)))
                .find(|&(_, _, symbol)| SPECIALS.contains(&symbol)) 
            {
                if symbol == COLLECT {
                    l_context.spins.bs_values[x][y] = MultiValueEnum::Int(0);
                    l_context.spins.bs_v[x][y] = MultiValueEnum::Int(0);
                }
            }
            // crutch end
            l_context.actions = vec![ActionsEnum::Spin, ActionsEnum::BuySpin];
            l_context.round_finished = true;
            l_context.spins.total_win = Some(l_context.bonus.get_or_insert_with(Default::default).total_win);
            l_context.spins.lucky_spin_win = Some(false);
            if l_context.spins.bac_win == Some(true) {
                if let Some(mechanic) = &l_context.spins.bonus_mechanic {
                    if mechanic.contains(&1) {l_context.spins.bac.set_initial_bac1();}
                    if mechanic.contains(&2) {l_context.spins.bac.set_initial_bac2();}
                    if mechanic.contains(&3) {l_context.spins.bac.set_initial_bac3();}
                }
            }
            l_context.spins.bac_win = Some(false);
            l_context.spins.bonus_mechanic = None;
            l_context.spins.bs_count = None;
            l_context.spins.bac_pos = None;
            l_context.spins.origin_board = None;

            l_context.bonus = None;

            l_context.current = CurrentActionsEnum::Spins;
            l_context.last_action = a_request.action.name.clone();
            let l_origin_data = a_game.origin_data.get_or_insert_with(Default::default);
            l_origin_data.feature = true;
            l_origin_data.set_denominator = a_request.set_denominator;
            l_origin_data.prev_request_id = a_request.prev_request_id.clone();
            l_origin_data.command = a_request.command.clone();
            l_origin_data.quickspin = a_request.quick_spin;
            l_origin_data.autogame = a_request.autogame;
            l_origin_data.sound = a_request.sound;
            l_origin_data.mobile = a_request.mobile.clone();
            l_origin_data.portrait = a_request.portrait;
            a_game.status.set(StatusCodesEnum::Ok, None, None, None);
        } else {a_game.status.set(StatusCodesEnum::BadRequest, Some(StatusTypesEnum::Crit), Some("EXPECTED_ACTIONS:".to_owned() + &l_context.actions.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(",") + " ACTUAL_ACTION:" + &a_request.action.name.to_string()), None);}
    } else {a_game.status.set(StatusCodesEnum::InternalServerError, Some(StatusTypesEnum::Crit), Some("CONTEXT_IN_BONUS_SPINS_STOP_IS_NONE".to_string()), None);}


    Ok(())
}