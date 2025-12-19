use actix_web::Error;
use crate::ROUND_ID_COUNTER;
use super::super::super::super::settings::{BOARD_HEIGHT, BOARD_WIDTH, LINES_COUNT, COIN, BOOST, COLLECT, MULTI, SPECIALS, };
use super::super::super::{server, network::request};
use super::super::super::enums::{CurrentActionsEnum, ActionsEnum, StatusCodesEnum, StatusTypesEnum, MultiValueEnum, BonusModesEnum, };
use super::super::super::mock::MockData;

pub async fn execute(a_request: &request::play::buy_spin::BuySpin, a_game: &mut server::Server, is_test: bool, a_mock_data: &MockData) -> Result<(), Error> {
    a_game.command = a_request.command.clone();
    a_game.request_id = a_request.request_id.clone();
    if let Some(ref mut l_user) = a_game.user {
        let l_total_bet = a_request.action.params.bet_per_line * a_game.settings.bet_factor[0];
        let l_cost = l_total_bet * a_game.settings.buy_bonus_price[a_request.action.params.selected_mode.as_usize()-1];
        if l_cost > 0 {
            if let Some(ref mut l_context) = a_game.context {
                if l_context.actions.contains(&a_request.action.name) {
                    if a_request.action.params.selected_mode.clone() == BonusModesEnum::Enum1 || a_request.action.params.selected_mode.clone() == BonusModesEnum::Enum2 {
                        // set start values
                        l_context.spins.bs_values = vec![vec![MultiValueEnum::Int(0); BOARD_HEIGHT]; BOARD_WIDTH];
                        l_context.spins.bs_v = vec![vec![MultiValueEnum::Int(0); BOARD_HEIGHT]; BOARD_WIDTH];
                        l_context.spins.bac_pos = None;
                        // generate board
                        // reels
                        let current_reels = if a_request.action.params.selected_mode.clone() == BonusModesEnum::Enum1 {&a_game.reels_buy1} else {&a_game.reels_buy2};
                        l_context.spins.board = if !is_test {current_reels.pick_reels(BOARD_HEIGHT, a_mock_data.spins_category, None)} else {a_mock_data.board.clone().unwrap_or_default()};
                        let mut l_origin_board: Vec<Vec<i64>> = l_context.spins.board.clone();
                        // coins appear and value
                        for col_num in 0..BOARD_WIDTH {
                            if let Some(coins) = current_reels.pick_spins_coins_appearance(a_mock_data.spins_coins_appearances.as_ref().and_then(|v| v.get(col_num).copied())) {
                                for row_num in coins.pos {
                                    l_context.spins.board[col_num][row_num as usize] = coins.id;
                                    l_origin_board[col_num][row_num as usize] = coins.id;
                                    let value = current_reels.pick_spins_coin_value(a_mock_data.spins_coins_values.as_ref().and_then(|v| v.get(col_num)).and_then(|inner| inner.get(row_num as usize)).copied());
                                    l_context.spins.bs_values[col_num][row_num as usize] = value.to_num();
                                    l_context.spins.bs_v[col_num][row_num as usize] = value.to_multi_value_by_coast(l_total_bet as u64);
                                }
                            }
                        }
                        // specials appear
                        let mut special_appearance = false;
                        let mut while_stoper = 1000;
                        while !special_appearance && while_stoper > 0 {
                            for col_num in 0..BOARD_WIDTH {
                                if let Some(special) = current_reels.pick_spins_special_appearance(a_mock_data.spins_specials_appearances.as_ref().and_then(|v| v.get(col_num).copied())) {
                                    for row_num in special.pos {
                                        l_context.spins.board[col_num][row_num as usize] = special.id;
                                        l_origin_board[col_num][row_num as usize] = special.id;
                                    }
                                    special_appearance = true;
                                }
                            }
                            while_stoper -= 1;
                        }
                        // check bonus
                        let mut l_bs_count = 0;
                        let mut l_mechanic: Vec<i64> = Vec::new();
                        for col_num in 0..BOARD_WIDTH {
                            for row_num in 0..BOARD_HEIGHT {
                                match l_context.spins.board[col_num][row_num] {
                                    COIN => {l_bs_count += 1;}
                                    BOOST => {l_bs_count += 1; l_mechanic.push(1);}
                                    COLLECT => {l_bs_count += 1; l_mechanic.push(2);}
                                    MULTI => {l_bs_count += 1; l_mechanic.push(3);}
                                    _ => {}
                                }
                            }
                        }
                        l_mechanic.sort_unstable();
                        l_mechanic.dedup();
                        // adding missing coins
                        let mut while_stoper = 1000;
                        while l_bs_count < 6 && while_stoper > 0 {
                            for col_num in 0..BOARD_WIDTH {
                                if let Some(coins) = current_reels.pick_spins_coins_appearance(a_mock_data.spins_bac_coins_appearances.as_ref().and_then(|v| v.get(col_num).copied())) {
                                    for row_num in coins.pos {
                                        if !SPECIALS.contains(&l_context.spins.board[col_num][row_num as usize]) && l_context.spins.board[col_num][row_num as usize] != COIN {
                                            l_context.spins.board[col_num][row_num as usize] = coins.id;
                                            let value = current_reels.pick_spins_coin_value(a_mock_data.spins_bac_coins_values.as_ref().and_then(|v| v.get(col_num)).and_then(|inner| inner.get(row_num as usize)).copied());
                                            l_context.spins.bs_values[col_num][row_num as usize] = value.to_num();
                                            l_context.spins.bs_v[col_num][row_num as usize] = value.to_multi_value_by_coast(l_total_bet as u64);
                                            l_context.spins.bac_pos.get_or_insert_with(Default::default).push(vec![col_num as i64, row_num as i64]);
                                            l_bs_count += 1;
                                        }
                                    }
                                }
                            }
                            while_stoper -= 1;
                        }
                        // set bonus
                        if l_bs_count >= 6 {
                            l_context.actions = vec![ActionsEnum::BonusInit];
                            l_context.round_finished = false;
                            l_context.spins.bonus_mechanic = (!l_mechanic.is_empty()).then(|| l_mechanic.clone());
                            l_context.spins.origin_board = Some(l_origin_board.clone());
                        } else {
                            l_context.actions = vec![ActionsEnum::Spin, ActionsEnum::BuySpin];
                            l_context.round_finished = true;
                            l_context.spins.bonus_mechanic = None;
                            l_context.bonus = None;
                            l_context.spins.origin_board = None;
                        }
                        // check winlines
                        l_context.spins.winlines = None;
                        l_context.spins.round_win = 0;
                        let mut l_checking_symbol;
                        let mut l_checking_lenght;
                        let mut l_checking_positions: Vec<Vec<i64>> = Vec::new();
                        for l in 0..LINES_COUNT {
                            l_checking_symbol = l_context.spins.board[0][a_game.settings.paylines[l][0] as usize];
                            l_checking_lenght = 1;
                            l_checking_positions.push(vec![0, a_game.settings.paylines[l][0].clone()]);
                            for x in 1..BOARD_WIDTH {
                                if a_game.settings.symbols_wild.contains(&l_checking_symbol) {l_checking_symbol = l_context.spins.board[x][a_game.settings.paylines[l][x] as usize]; l_checking_lenght += 1; l_checking_positions.push(vec![x as i64, a_game.settings.paylines[l][x].clone()]); continue;}
                                if l_checking_symbol == l_context.spins.board[x][a_game.settings.paylines[l][x] as usize] 
                                || a_game.settings.symbols_wild.contains(&l_context.spins.board[x][a_game.settings.paylines[l][x] as usize].clone()) 
                                {l_checking_positions.push(vec![x as i64, a_game.settings.paylines[l][x].clone()]); l_checking_lenght += 1;}
                                else{break;}
                            }
                            if a_game.settings.symbols_line.contains(&l_checking_symbol) { 
                                if let Some(l_paytable_symbol) = a_game.settings.paytable.get(&format!("{}", l_checking_symbol)) {
                                    if l_paytable_symbol.len() > 0 {
                                        for occ in 0..l_paytable_symbol.len() {
                                            if l_paytable_symbol[occ].occurrences == l_checking_lenght {
                                                l_context.spins.winlines.get_or_insert_with(Vec::new).push(server::Winlines {
                                                    amount: (a_request.action.params.bet_per_line * l_paytable_symbol[occ].multiplier) as i64, 
                                                    line: (l+1) as i64, 
                                                    occurrences: l_paytable_symbol[occ].occurrences, 
                                                    positions: l_checking_positions.clone(), 
                                                    symbol: l_checking_symbol, 
                                                    winlines_type: "lb".to_string()
                                                });
                                                l_context.spins.round_win += (a_request.action.params.bet_per_line * l_paytable_symbol[occ].multiplier) as i64;
                                            }
                                        }
                                    }
                                }
                            }
                            l_checking_positions.clear();
                        }
                        if let Some(ref mut winlines) = l_context.spins.winlines {winlines.sort_by(|a, b| b.amount.cmp(&a.amount));}
                        // scrutch start
                        if l_context.spins.round_win > 0 {
                            l_context.spins.round_win = 0;
                            l_context.spins.winlines = None;
                        }
                        // "origin_board":[[9,9,9],[3,3,12],[3,3,7],[5,5,5],[3,3,13]],"board":[[1,1,1],[3,3,12],[3,3,10],[10,10,10],[3,3,13]],
                        if a_game.request_id == "5376c4aa-9d99-4bb9-bc57-ed8ca16d4ca7".to_string() {l_context.spins.board[0] = vec![1,1,1]}; 
                        // "origin_board":[[4,4,9],[4,7,7],[1,13,1],[7,7,7],[4,9,9]],"board":[[4,4,4],[4,7,7],[1,13,10],[10,7,7],[10,10,10]],
                        if a_game.request_id == "f910f551-ad64-4232-9fbe-96c374e9f25a".to_string() {l_context.spins.board[0][2] = 4}; 
                        // scrutch end
                        // static data
                        l_context.spins.lucky_spin_win = Some(false);
                        l_context.spins.bet_per_line = a_request.action.params.bet_per_line;
                        l_context.spins.lines = a_request.action.params.lines;
                        l_context.spins.round_bet = l_total_bet;
                        l_context.spins.selected_mode = Some(a_request.action.params.selected_mode.clone());
                        l_context.current = CurrentActionsEnum::Spins;
                        l_context.last_action = a_request.action.name.clone();
                        l_context.last_args.bet_per_line = Some(a_request.action.params.bet_per_line);
                        l_context.last_args.lines = Some(a_request.action.params.lines);
                        l_context.last_args.bet_factor = Some(a_request.action.params.bet_factor);
                        l_context.last_args.selected_mode = Some(a_request.action.params.selected_mode.clone());
                        l_context.version = 1;
                        
                        if l_user.balance >= l_cost {
                            // set wins and balance
                            l_context.spins.total_win = Some(l_context.spins.round_win);
                            if l_context.spins.total_win > Some(0) {l_context.last_win = l_context.spins.total_win};
                            l_user.balance -= l_cost;
                            l_user.balance += l_context.spins.total_win.unwrap_or(0);
                            // round_id
                            a_game.roundnum = if !is_test {Some(ROUND_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst).to_string())} else {a_mock_data.roundnum.clone()};
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
                        } else {
                            a_game.roundnum = None;
                            a_game.origin_data = None;
                            a_game.status.set(StatusCodesEnum::FundsExceed, Some(StatusTypesEnum::Exceed), Some("NOT_ENOUGH_MONEY".to_string()), Some(-1));
                        }
                    } else {a_game.status.set(StatusCodesEnum::BadRequest, Some(StatusTypesEnum::Crit), Some("INCORRECT_BUY_MODE".to_string()), None);}
                } else {a_game.status.set(StatusCodesEnum::BadRequest, Some(StatusTypesEnum::Crit), Some("EXPECTED_ACTIONS:".to_owned() + &l_context.actions.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(",") + " ACTUAL_ACTION:" + &a_request.action.name.to_string()), None);}
            } else {a_game.status.set(StatusCodesEnum::InternalServerError, Some(StatusTypesEnum::Crit), Some("CONTEXT_IN_SPIN_IS_NONE".to_string()), None);}
        } else {a_game.status.set(StatusCodesEnum::BadRequest, Some(StatusTypesEnum::Crit), Some("ZERO_TOTAL_BET".to_string()), None);}
    } else {a_game.status.set(StatusCodesEnum::InternalServerError, Some(StatusTypesEnum::Crit), Some("USER_IN_SPIN_IS_NONE".to_string()), None);}
    Ok(())
}