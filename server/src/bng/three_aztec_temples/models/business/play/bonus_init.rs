use actix_web::Error;
use crate::ROUND_ID_COUNTER;
use crate::bng::three_aztec_temples::settings::{BOARD_HEIGHT, BOARD_WIDTH, JACKPOTS_VALUE, JACKPOTS_STR, COIN, BOOST, COLLECT, MULTI, BOOST_STR, MULTI_STR, SPECIALS, };
use super::super::super::{server, network::request};
use crate::bng::three_aztec_temples::models::server::{BoostValues, CollectValues, MultiValues};
use super::super::super::enums::{CurrentActionsEnum, ActionsEnum, StatusCodesEnum, StatusTypesEnum, MultiValueEnum, BonusModesEnum, };
use super::super::super::mock::MockData;

pub async fn execute(a_request: &request::play::bonus_init::BonusInit, a_game: &mut server::Server, is_test: bool, a_mock_data: &MockData, ) -> Result<(), Error> {
    a_game.command = a_request.command.clone();
    a_game.request_id = a_request.request_id.clone();
    if let Some(ref mut l_context) = a_game.context {
        if l_context.actions.contains(&a_request.action.name) {
            let l_bonus = l_context.bonus.get_or_insert_with(Default::default);
            l_bonus.bet_per_line = l_context.spins.bet_per_line.clone();
            l_bonus.lines = l_context.spins.lines.clone();
            l_bonus.round_bet = l_context.spins.round_bet.clone();
            l_bonus.selected_mode = match l_context.spins.selected_mode {
                Some(BonusModesEnum::Enum1) => Some(BonusModesEnum::Enum1),
                Some(BonusModesEnum::Enum2) => Some(BonusModesEnum::Enum2),
                _ => None
            };
            l_bonus.lucky_spin_win = l_context.spins.lucky_spin_win.unwrap_or_default();
            l_bonus.bac = l_context.spins.bac.clone();
            l_bonus.bonus_mechanic = l_context.spins.bonus_mechanic.clone();
            l_bonus.bs_count = 0;
            l_bonus.board = l_context.spins.board.clone();
            l_bonus.bs_values = l_context.spins.bs_values.clone();
            l_bonus.origin_bs_v = if l_context.spins.bonus_mechanic.is_some() {Some(l_context.spins.bs_v.clone())} else {None};
            l_bonus.bs_v = vec![vec![MultiValueEnum::Int(0); BOARD_HEIGHT]; BOARD_WIDTH];
            l_bonus.bs_mults = vec![vec![0; BOARD_HEIGHT]; BOARD_WIDTH];
            l_bonus.board_is_executed = vec![vec![false; BOARD_HEIGHT]; BOARD_WIDTH];
            let mechanic_id = l_bonus.bonus_mechanic.as_ref().map(|v| v.iter().map(|n| n.to_string()).collect::<String>()).unwrap_or_default();
            // round_id
            a_game.roundnum = if !is_test {Some(ROUND_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst).to_string())} else {a_mock_data.roundnum.clone()};
            // select reels
            let current_reels = match l_bonus.selected_mode {
                Some(BonusModesEnum::Enum1) => {&a_game.reels_buy1},
                Some(BonusModesEnum::Enum2) => {&a_game.reels_buy2},
                _ => {&a_game.reels}
            };
            // select bonus category
            l_bonus.bonus_game_type = (current_reels.pick_bonus_type(a_mock_data.bonus_category) + 1) as i64;
            // generate board
            // set specials values
            for col_num in 0..BOARD_WIDTH {
                for row_num in 0..BOARD_HEIGHT {
                    if !l_bonus.board_is_executed[col_num][row_num] {
                        let special_value_mock_rand_num = match a_mock_data.bonus_specials_init_values.as_ref() {None => None, Some(rows) => rows.get(col_num).and_then(|row| row.get(row_num)).copied().or(Some(10000)),};
                        let special_mult_mock_rand_num = match a_mock_data.bonus_specials_init_mults.as_ref() {None => None, Some(rows) => rows.get(col_num).and_then(|row| row.get(row_num)).copied().or(Some(10000)),};
                        match l_bonus.board[col_num][row_num] {
                            COIN => {
                                if let MultiValueEnum::String(s) = &l_context.spins.bs_v[col_num][row_num].clone() {
                                    if JACKPOTS_STR.contains(&s.as_str()) {
                                        l_bonus.jackpot_values = Some(JACKPOTS_VALUE.iter().map(|v| v * l_bonus.round_bet).collect());
                                        l_bonus.jackpot_positions.get_or_insert_with(|| vec![vec![MultiValueEnum::Int(0); BOARD_HEIGHT]; BOARD_WIDTH])[col_num][row_num] = MultiValueEnum::String(s.clone());
                                    }
                                }
                                l_bonus.board_is_executed[col_num][row_num] = true;
                            }
                            BOOST => {
                                let value = current_reels.pick_bonus_init_value(&mechanic_id, BOOST_STR, special_value_mock_rand_num);
                                l_bonus.bs_values[col_num][row_num] = value.to_num();
                                l_bonus.boost_values.get_or_insert_with(Default::default).push(BoostValues { bs_v: value.to_num_value_by_coast(l_bonus.round_bet as u64), pos: vec![col_num as i64, row_num as i64] });
                            }
                            COLLECT => {
                                //l_bonus.collect_values.get_or_insert_with(Default::default).push(CollectValues { bs_v: MultiValueEnum::Int(0), pos: vec![col_num as i64, row_num as i64] });
                            }
                            MULTI => {
                                let value = current_reels.pick_bonus_init_value(&mechanic_id, MULTI_STR, special_value_mock_rand_num);
                                l_bonus.bs_values[col_num][row_num] = value.to_num();
                                let mult = current_reels.pick_bonus_init_multiplayer(&mechanic_id, MULTI_STR, special_mult_mock_rand_num);
                                l_bonus.bs_mults[col_num][row_num] = mult.clone();
                                l_bonus.multi_values.get_or_insert_with(Default::default).push(MultiValues { bs_v: value.to_num_value_by_coast(l_bonus.round_bet as u64), mult_value: mult, pos: vec![col_num as i64, row_num as i64] });
                            }
                            _ => {}
                        }
                    }
                }
            }
            // execute specials
            for col_num_current in 0..BOARD_WIDTH {
                for row_num_current in 0..BOARD_HEIGHT {
                    if !l_bonus.board_is_executed[col_num_current][row_num_current] {
                        match l_bonus.board[col_num_current][row_num_current] {
                            COIN => {}
                            BOOST => {
                                for col_num_target in 0..BOARD_WIDTH {
                                    for row_num_target in 0..BOARD_HEIGHT {
                                        if l_bonus.board_is_executed[col_num_target][row_num_target] {
                                            l_bonus.bs_values[col_num_target][row_num_target] = l_bonus.bs_values[col_num_target][row_num_target].clone() + l_bonus.bs_values[col_num_current][row_num_current].clone();
                                            if let Some(l_jackpot_positions) = l_bonus.jackpot_positions.as_ref() {
                                                if l_jackpot_positions[col_num_target][row_num_target] != MultiValueEnum::Int(0) {
                                                    let l_jackpots_boost_values = l_bonus.jackpots_boost_values.get_or_insert_with(|| vec![vec![0; BOARD_HEIGHT]; BOARD_WIDTH]);
                                                    l_jackpots_boost_values[col_num_target][row_num_target] = (MultiValueEnum::Int(l_jackpots_boost_values[col_num_target][row_num_target]) + l_bonus.bs_values[col_num_current][row_num_current].to_num_value_by_coast(l_bonus.round_bet as u64)).as_f64() as i64;
                                                };
                                            };
                                        }
                                    }
                                }
                                l_bonus.board_is_executed[col_num_current][row_num_current] = true;
                            }
                            COLLECT => {
                                for col_num_target in 0..BOARD_WIDTH {
                                    for row_num_target in 0..BOARD_HEIGHT {
                                        if l_bonus.board_is_executed[col_num_target][row_num_target] {
                                            let target_value = l_bonus.bs_values[col_num_target][row_num_target].clone(); 
                                            l_bonus.bs_values[col_num_current][row_num_current] += target_value;
                                        }
                                    }
                                }
                                // crutch start
                                if l_context.spins.board.iter().flatten().find(|&&symbol| SPECIALS.contains(&symbol)).map_or(false, |&symbol| symbol == COLLECT) {
                                    l_context.spins.bs_values[col_num_current][row_num_current] = l_bonus.bs_values[col_num_current][row_num_current].clone();
                                    l_context.spins.bs_v[col_num_current][row_num_current] = l_context.spins.bs_values[col_num_current][row_num_current].to_num_value_by_coast(l_bonus.round_bet as u64);
                                };
                                // crutch end
                                l_bonus.board_is_executed[col_num_current][row_num_current] = true;
                                l_bonus.collect_values.get_or_insert_with(Default::default).push(CollectValues { bs_v: l_bonus.bs_values[col_num_current][row_num_current].to_num_value_by_coast(l_bonus.round_bet as u64), pos: vec![col_num_current as i64, row_num_current as i64] });
                            }
                            MULTI => {
                                for col_num_target in 0..BOARD_WIDTH {
                                    for row_num_target in 0..BOARD_HEIGHT {
                                        if l_bonus.board_is_executed[col_num_target][row_num_target] {
                                            l_bonus.bs_values[col_num_target][row_num_target] = l_bonus.bs_values[col_num_target][row_num_target].clone() * MultiValueEnum::Int(l_bonus.bs_mults[col_num_current][row_num_current].clone());
                                            if let Some(l_jackpot_positions) = l_bonus.jackpot_positions.as_ref() {
                                                if l_jackpot_positions[col_num_target][row_num_target] != MultiValueEnum::Int(0) {
                                                    let l_jackpots_multiplier_values = l_bonus.jackpots_multiplier_values.get_or_insert_with(|| vec![vec![0; BOARD_HEIGHT]; BOARD_WIDTH]);
                                                    l_jackpots_multiplier_values[col_num_target][row_num_target] = l_jackpots_multiplier_values[col_num_target][row_num_target] + l_bonus.bs_mults[col_num_current][row_num_current];
                                                    if let Some(l_jackpots_boost_values) = &mut l_bonus.jackpots_boost_values {
                                                        l_jackpots_boost_values[col_num_target][row_num_target] = (MultiValueEnum::Int(l_jackpots_boost_values[col_num_target][row_num_target]) * MultiValueEnum::Int(l_bonus.bs_mults[col_num_current][row_num_current].clone())).as_f64() as i64;
                                                    }
                                                };
                                            };
                                        }
                                    }
                                }
                                l_bonus.board_is_executed[col_num_current][row_num_current] = true;
                            }
                            _ => {}
                        }
                    }
                }
            }
            // set bs_v amounts
            for col_num in 0..BOARD_WIDTH {
                for row_num in 0..BOARD_HEIGHT {
                    if l_bonus.board_is_executed[col_num][row_num] {
                        l_bonus.bs_v[col_num][row_num] = l_bonus.bs_values[col_num][row_num].to_multi_value_by_coast(l_bonus.round_bet as u64);
                        if let Some(l_jackpot_positions) = l_bonus.jackpot_positions.as_ref() {if l_jackpot_positions[col_num][row_num] != MultiValueEnum::Int(0) {l_bonus.bs_v[col_num][row_num] = l_jackpot_positions[col_num][row_num].clone()}};
                        l_bonus.bs_count += 1;
                    }
                }
            }
            l_bonus.new_bs = None;
            l_bonus.mystery_values = None;
            //l_bonus.jackpot_positions = None;
            //l_bonus.jackpot_values = None;
            //l_bonus.jackpots_boost_values = None;
            //l_bonus.jackpots_multiplier_values = None;
            
            // static data
            l_bonus.init_bs_count = Some(true);
            l_bonus.last_respin = false;
            l_bonus.rounds_granted = 3;
            l_bonus.rounds_left = 3;
            l_bonus.round_win = 0;
            l_bonus.total_win = 0;
            l_bonus.back_to = CurrentActionsEnum::Spins;
            l_context.spins.total_win = None;
            l_context.round_finished = false;
            l_context.actions = vec![ActionsEnum::Respin];
            l_context.current = CurrentActionsEnum::Bonus;
            l_context.last_action = a_request.action.name.clone();
            l_context.last_args.bet_per_line = None;
            l_context.last_args.lines = None;
            l_context.last_args.bet_factor = None;
            l_context.last_args.selected_mode = None;
            l_context.version = 1;
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
    } else {a_game.status.set(StatusCodesEnum::InternalServerError, Some(StatusTypesEnum::Crit), Some("CONTEXT_IN_BONUS_INIT_IS_NONE".to_string()), None);}

    Ok(())
}