use actix_web::Error;
use crate::ROUND_ID_COUNTER;
use crate::bng::three_aztec_temples::settings::{BOARD_HEIGHT, BOARD_WIDTH, JACKPOTS_VALUE, JACKPOTS_STR, COIN, BOOST, COLLECT, MULTI, MYSTERY, BONUS_SYMBOLS, SPECIALS, };
use super::super::super::{server, network::request};
use crate::bng::three_aztec_temples::models::server::{BoostValues, CollectValues, MultiValues, MysteryValues, };
use super::super::super::enums::{CurrentActionsEnum, ActionsEnum, StatusCodesEnum, StatusTypesEnum, MultiValueEnum, BonusModesEnum, };
use super::super::super::mock::MockData;

pub async fn execute(a_request: &request::play::respin::Respin, a_game: &mut server::Server, is_test: bool, a_mock_data: &MockData, ) -> Result<(), Error> {
    a_game.command = a_request.command.clone();
    a_game.request_id = a_request.request_id.clone();
    if let Some(ref mut l_context) = a_game.context {
        if l_context.actions.contains(&a_request.action.name) {
            if let Some(ref mut l_bonus) = l_context.bonus {
            // set start values
            l_bonus.jackpot_values = None;
            l_bonus.boost_values = None;
            l_bonus.collect_values = None;
            l_bonus.multi_values = None;
            l_bonus.mystery_values = None;
            l_bonus.bs_pos = None;
            l_bonus.new_bs = None;
            l_bonus.origin_board = None;
            l_bonus.origin_bs_v = None;
            l_bonus.origin_bs_values = l_bonus.bs_values.clone();
            let mechanic_id = l_bonus.bonus_mechanic.as_ref().map(|v| v.iter().map(|n| n.to_string()).collect::<String>()).unwrap_or("0".to_string());
            // round_id
            a_game.roundnum = if !is_test {Some(ROUND_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst).to_string())} else {a_mock_data.roundnum.clone()};
            // select reels
            let current_reels = match l_bonus.selected_mode {
                Some(BonusModesEnum::Enum1) => {&a_game.reels_buy1},
                Some(BonusModesEnum::Enum2) => {&a_game.reels_buy2},
                _ => {&a_game.reels}
            };
            // generate board
            let mut new_bs_count = 0;
            // mysteries appear and value
            let mut _mystery_appearance = false;
            for col_num in 0..BOARD_WIDTH {
                for row_num in 0..BOARD_HEIGHT {
                    if !l_bonus.board_is_executed[col_num][row_num] {
                        let (a, i, v, m) = (
                            match a_mock_data.bonus_mystery_appearances.as_ref() {None => None,Some(rows) => rows.get(col_num).and_then(|row| row.get(row_num)).copied().or(Some(10000)),},
                            match a_mock_data.bonus_mystery_ids.as_ref() {None => None,Some(rows) => rows.get(col_num).and_then(|row| row.get(row_num)).copied().or(Some(10000)),},
                            match a_mock_data.bonus_mystery_values.as_ref() {None => None,Some(rows) => rows.get(col_num).and_then(|row| row.get(row_num)).copied().or(Some(10000)),},
                            match a_mock_data.bonus_mystery_mults.as_ref() {None => None,Some(rows) => rows.get(col_num).and_then(|row| row.get(row_num)).copied().or(Some(10000)),}
                        );
                        let mystery = current_reels.pick_bonus_respin_mystery(l_bonus.bonus_game_type as usize, &mechanic_id, &(l_bonus.bs_count + new_bs_count).to_string(), a, i, v, m);
                        if let Some((id, value, mult)) = mystery {
                            if SPECIALS.contains(&id) {
                                l_bonus.origin_bs_v.get_or_insert_with(|| vec![vec![MultiValueEnum::Int(0); BOARD_HEIGHT]; BOARD_WIDTH]);
                            }
                            l_bonus.origin_board.get_or_insert_with(|| {l_bonus.board.clone()})[col_num][row_num] = MYSTERY;
                            l_bonus.board[col_num][row_num] = id;
                            match id {
                                COIN => {
                                    if let MultiValueEnum::String(s) = &value {
                                        if JACKPOTS_STR.contains(&s.as_str()) {
                                            l_bonus.jackpot_values = Some(JACKPOTS_VALUE.iter().map(|v| v * l_bonus.round_bet).collect());
                                            l_bonus.jackpot_positions.get_or_insert_with(|| vec![vec![MultiValueEnum::Int(0); BOARD_HEIGHT]; BOARD_WIDTH])[col_num][row_num] = MultiValueEnum::String(s.clone());
                                        }
                                    }
                                    l_bonus.bs_values[col_num][row_num] = value.to_num();
                                    l_bonus.mystery_values.get_or_insert_with(Default::default).push(MysteryValues { bs_v: value.to_num_value_by_coast(l_bonus.round_bet as u64), id: id, pos: vec![col_num as i64, row_num as i64] });
                                }
                                BOOST => {
                                    l_bonus.bs_values[col_num][row_num] = value.to_num();
                                    l_bonus.boost_values.get_or_insert_with(Default::default).push(BoostValues { bs_v: value.to_num_value_by_coast(l_bonus.round_bet as u64), pos: vec![col_num as i64, row_num as i64] });
                                    l_bonus.mystery_values.get_or_insert_with(Default::default).push(MysteryValues { bs_v: MultiValueEnum::Int(0), id: id, pos: vec![col_num as i64, row_num as i64] });
                                }
                                COLLECT => {
                                    l_bonus.mystery_values.get_or_insert_with(Default::default).push(MysteryValues { bs_v: MultiValueEnum::Int(0), id: id, pos: vec![col_num as i64, row_num as i64] });
                                }
                                MULTI => {
                                    l_bonus.bs_values[col_num][row_num] = value.to_num();
                                    l_bonus.bs_mults[col_num][row_num] = mult.clone();
                                    l_bonus.multi_values.get_or_insert_with(Default::default).push(MultiValues { bs_v: value.to_num_value_by_coast(l_bonus.round_bet as u64), mult_value: mult, pos: vec![col_num as i64, row_num as i64] });
                                    l_bonus.mystery_values.get_or_insert_with(Default::default).push(MysteryValues { bs_v: MultiValueEnum::Int(0), id: id, pos: vec![col_num as i64, row_num as i64] });
                                }
                                _ => {}
                            };
                            _mystery_appearance = true;
                            new_bs_count += 1;
                        }
                    }
                }
            }
            // specials appear and value
            let mut _special_appearance = false;
            for col_num in 0..BOARD_WIDTH {
                for row_num in 0..BOARD_HEIGHT {
                    if !l_bonus.board_is_executed[col_num][row_num] && l_bonus.board[col_num][row_num] != MYSTERY {
                        let (a, v, m) = (
                                match a_mock_data.bonus_specials_appearances.as_ref() {None => None,Some(rows) => rows.get(col_num).and_then(|row| row.get(row_num)).copied().or(Some(10000)),},
                                match a_mock_data.bonus_specials_values.as_ref() {None => None,Some(rows) => rows.get(col_num).and_then(|row| row.get(row_num)).copied().or(Some(10000)),},
                                match a_mock_data.bonus_specials_mults.as_ref() {None => None,Some(rows) => rows.get(col_num).and_then(|row| row.get(row_num)).copied().or(Some(10000)),}
                        );
                        let special = current_reels.pick_bonus_respin_special(l_bonus.bonus_game_type as usize, &mechanic_id, &(l_bonus.bs_count + new_bs_count).to_string(), a, v, m);
                        if let Some((id, value, mult)) = special {
                            l_bonus.origin_bs_v.get_or_insert_with(|| vec![vec![MultiValueEnum::Int(0); BOARD_HEIGHT]; BOARD_WIDTH]);
                            if let Some(l_origin_board) = l_bonus.origin_board.as_mut() {l_origin_board[col_num][row_num] = id;}
                            l_bonus.board[col_num][row_num] = id;
                            match id {
                                BOOST => {
                                    l_bonus.bs_values[col_num][row_num] = value.to_num();
                                    l_bonus.boost_values.get_or_insert_with(Default::default).push(BoostValues { bs_v: value.to_num_value_by_coast(l_bonus.round_bet as u64), pos: vec![col_num as i64, row_num as i64] });
                                    
                                }
                                COLLECT => {}
                                MULTI => {
                                    l_bonus.bs_values[col_num][row_num] = value.to_num();
                                    l_bonus.bs_mults[col_num][row_num] = mult.clone();
                                    l_bonus.multi_values.get_or_insert_with(Default::default).push(MultiValues { bs_v: value.to_num_value_by_coast(l_bonus.round_bet as u64), mult_value: mult, pos: vec![col_num as i64, row_num as i64] });
                                }
                                _ => {}
                            }
                            _special_appearance = true;
                            new_bs_count += 1;
                        }
                    }
                }
            }
            // coins appear and value
            let mut _coin_appearance = false;
            for col_num in 0..BOARD_WIDTH {
                for row_num in 0..BOARD_HEIGHT {
                    if !l_bonus.board_is_executed[col_num][row_num] && l_bonus.board[col_num][row_num] != MYSTERY && !SPECIALS.contains(&l_bonus.board[col_num][row_num]) {
                        let (a, v,) = (
                                match a_mock_data.bonus_coins_appearances.as_ref() {None => None,Some(rows) => rows.get(col_num).and_then(|row| row.get(row_num)).copied().or(Some(10000)),},
                                match a_mock_data.bonus_coins_values.as_ref() {None => None,Some(rows) => rows.get(col_num).and_then(|row| row.get(row_num)).copied().or(Some(10000)),}
                        );
                        let coin = current_reels.pick_bonus_respin_coin(l_bonus.bonus_game_type as usize, &mechanic_id, &(l_bonus.bs_count + new_bs_count).to_string(), a, v);
                        if let Some((id, value)) = coin {
                            if let MultiValueEnum::String(s) = &value {
                                if JACKPOTS_STR.contains(&s.as_str()) {
                                    l_bonus.jackpot_values = Some(JACKPOTS_VALUE.iter().map(|v| v * l_bonus.round_bet).collect());
                                    l_bonus.jackpot_positions.get_or_insert_with(|| vec![vec![MultiValueEnum::Int(0); BOARD_HEIGHT]; BOARD_WIDTH])[col_num][row_num] = MultiValueEnum::String(s.clone());
                                }
                            }
                            if let Some(l_origin_board) = l_bonus.origin_board.as_mut() {l_origin_board[col_num][row_num] = id;}
                            l_bonus.board[col_num][row_num] = id;
                            l_bonus.origin_bs_values[col_num][row_num] = value.to_num();
                            l_bonus.bs_values[col_num][row_num] = value.to_num();
                            _coin_appearance = true;
                            new_bs_count += 1;
                        }
                    }
                }
            }
            // execute coins
            for col_num in 0..BOARD_WIDTH {
                for row_num in 0..BOARD_HEIGHT {
                    if l_bonus.board_is_executed[col_num][row_num] && BONUS_SYMBOLS.contains(&l_bonus.board[col_num][row_num]) {l_bonus.bs_pos.get_or_insert_with(Default::default).push(vec![col_num as i64, row_num as i64]);}
                    if !l_bonus.board_is_executed[col_num][row_num] {
                        if BONUS_SYMBOLS.contains(&l_bonus.board[col_num][row_num]) {l_bonus.new_bs.get_or_insert_with(Default::default).push(vec![col_num as i64, row_num as i64])};
                        match l_bonus.board[col_num][row_num] {
                            COIN => {l_bonus.board_is_executed[col_num][row_num] = true;}
                            BOOST => {l_bonus.bonus_mechanic.get_or_insert_with(Default::default).push(1);}
                            COLLECT => {l_bonus.bonus_mechanic.get_or_insert_with(Default::default).push(2);}
                            MULTI => {l_bonus.bonus_mechanic.get_or_insert_with(Default::default).push(3);}
                            MYSTERY => {
                                    if let Some(l_origin_board) = l_bonus.origin_board.as_mut() {
                                        match l_origin_board[col_num][row_num] {
                                            COIN => {l_bonus.board_is_executed[col_num][row_num] = true;}
                                            BOOST => {l_bonus.bonus_mechanic.get_or_insert_with(Default::default).push(1);}
                                            COLLECT => {l_bonus.bonus_mechanic.get_or_insert_with(Default::default).push(2);}
                                            MULTI => {l_bonus.bonus_mechanic.get_or_insert_with(Default::default).push(3);}
                                            _ => {}
                                        }
                                    }
                                }
                            _ => {}
                        }
                        if let Some(bonus_mechanic) =  &mut l_bonus.bonus_mechanic {bonus_mechanic.sort_unstable(); bonus_mechanic.dedup();}
                    }
                }
            }
            // execute specials and mysteiys
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
                                        if l_bonus.board_is_executed[col_num_target][row_num_target] {let target_value = l_bonus.bs_values[col_num_target][row_num_target].clone(); l_bonus.bs_values[col_num_current][row_num_current] += target_value;}
                                    }
                                }
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
                            MYSTERY => {
                                if let Some(l_origin_board) = l_bonus.origin_board.as_mut() {
                                    match l_origin_board[col_num_current][row_num_current] {
                                        COIN => {}
                                        BOOST => {
                                            for col_num_target in 0..BOARD_WIDTH {
                                                for row_num_target in 0..BOARD_HEIGHT {
                                                    if l_bonus.board_is_executed[col_num_target][row_num_target] {
                                                        l_bonus.bs_values[col_num_target][row_num_target] = l_bonus.bs_values[col_num_target][row_num_target].clone() + l_bonus.bs_values[col_num_current][row_num_current].clone();
                                                        if let Some(l_jackpot_positions) = l_bonus.jackpot_positions.as_ref() {
                                                            if l_jackpot_positions[col_num_target][row_num_target] != MultiValueEnum::Int(0) {
                                                                let l_jackpots_boost_values = l_bonus.jackpots_boost_values.get_or_insert_with(|| vec![vec![0; BOARD_HEIGHT]; BOARD_WIDTH]);
                                                                let l_jackpots_boost_value = MultiValueEnum::Int(l_jackpots_boost_values[col_num_target][row_num_target]) + l_bonus.bs_values[col_num_current][row_num_current].clone();
                                                                l_jackpots_boost_values[col_num_target][row_num_target] = l_jackpots_boost_value.to_num_value_by_coast(l_bonus.round_bet as u64).as_f64() as i64;
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
                                                    if l_bonus.board_is_executed[col_num_target][row_num_target] {let target_value = l_bonus.bs_values[col_num_target][row_num_target].clone(); l_bonus.bs_values[col_num_current][row_num_current] += target_value;}
                                                }
                                            }
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
                        if let Some(l_origin_bs_v) = l_bonus.origin_bs_v.as_mut() {
                            l_origin_bs_v[col_num][row_num] = l_bonus.origin_bs_values[col_num][row_num].to_multi_value_by_coast(l_bonus.round_bet as u64);
                            if let Some(l_jackpot_positions) = l_bonus.jackpot_positions.as_ref() {if l_jackpot_positions[col_num][row_num] != MultiValueEnum::Int(0) {l_origin_bs_v[col_num][row_num] = l_jackpot_positions[col_num][row_num].clone()}};
                        };
                    }
                }
            }

            //l_bonus.jackpot_positions = None;
            //l_bonus.jackpots_boost_values = None;
            //l_bonus.jackpots_multiplier_values = None;
            l_bonus.last_respin = false;
            l_bonus.rounds_granted = 3;
            l_bonus.round_win = 0;
            l_bonus.total_win = 0;

            l_bonus.bs_count += new_bs_count;
            if new_bs_count > 0 && l_bonus.bs_count < 15 {
                l_bonus.init_bs_count = None;
                l_bonus.rounds_left = 3;
                l_context.actions = vec![ActionsEnum::Respin];
            } else {
                l_bonus.rounds_left -= 1;
                if l_bonus.rounds_left > 0 && l_bonus.bs_count < 15 {l_context.actions = vec![ActionsEnum::Respin]}
                else {
                    l_context.actions = vec![ActionsEnum::BonusSpinsStop];
                    if l_bonus.bs_count == 15 {
                        l_bonus.rounds_left = 0;
                        l_bonus.round_win = a_game.settings.jackpots.grand * l_bonus.round_bet;
                    }
                    for x in 0..BOARD_WIDTH {
                        for y in 0..BOARD_HEIGHT {
                            l_bonus.round_win += (l_bonus.bs_values[x][y].as_f64() * l_bonus.round_bet as f64) as i64;
                        }
                    }
                    l_bonus.total_win += l_bonus.round_win;
                    if let Some(ref mut l_user) = a_game.user {l_user.balance += l_bonus.round_win};
                    l_context.last_win = Some(l_bonus.round_win);
                }
            }

                
            // static data
            l_bonus.back_to = CurrentActionsEnum::Spins;
            l_context.round_finished = false;
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
            } else {a_game.status.set(StatusCodesEnum::InternalServerError, Some(StatusTypesEnum::Crit), Some("BONUS_IN_RESPIN_IS_NONE".to_string()), None);}
        } else {a_game.status.set(StatusCodesEnum::BadRequest, Some(StatusTypesEnum::Crit), Some("EXPECTED_ACTIONS:".to_owned() + &l_context.actions.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(",") + " ACTUAL_ACTION:" + &a_request.action.name.to_string()), None);}
    } else {a_game.status.set(StatusCodesEnum::InternalServerError, Some(StatusTypesEnum::Crit), Some("CONTEXT_IN_RESPIN_IS_NONE".to_string()), None);}

    Ok(())
}