use actix_web::Error;
use serde_json::Value;
use rand::Rng;
use crate::utils;
use crate::game::settings::{BOARD_HEIGHT, BOARD_WIDTH, BOARD_LINES_COUNT};
use crate::game::models::{model, network::request};

pub async fn execute(a_request: &request::play::Play, a_game: &mut model::Game, expected_response: Option<Value>) -> Result<(), Error> {
    a_game.command = a_request.command.clone();

    if a_game.context.actions.contains(&a_request.action.name.clone()) {
        let l_bet_per_line = a_request.action.params.bet_per_line.clone().ok_or_else(|| utils::err_http_responses("BadRequest", "Missing bet_per_line.", "ERR_PARAM_REQUIRED"))?;
        let l_lines = a_request.action.params.lines.clone().ok_or_else(|| utils::err_http_responses("BadRequest", "Missing lines.", "ERR_PARAM_REQUIRED"))?;
        if a_game.user.balance > ((l_bet_per_line.as_i64().unwrap_or(0) as i32 * a_game.settings.bet_factor[0]) as i64) {

            if cfg!(test) {
                a_game.context.spins.spin_type = expected_response.as_ref().and_then(|json| json.get("context")).and_then(|context| context.get("spins")).and_then(|spins| spins.get("spin_type")).and_then(|value| value.as_i64()).map(|v| v as i32);
                a_game.context.spins.board = expected_response.as_ref().and_then(|json| json.get("context")).and_then(|context| context.get("spins")).and_then(|spins| spins.get("board")).and_then(|board_value| board_value.as_array()).and_then(|board_array| {if board_array.len() == 5 {let mut board: [[i32; 3]; 5] = [[0; 3]; 5];for (i, row_value) in board_array.iter().enumerate() {if let Some(row) = row_value.as_array() {if row.len() == 3 {for (j, cell) in row.iter().enumerate() {board[i][j] = cell.as_i64()? as i32;}} else {return None;}} else {return None;}} Some(board)} else {None}}).unwrap_or([[0; 3]; 5]);
            } else {
                a_game.context.spins.spin_type = Some(rand::thread_rng().gen_range(0..=16));
                a_game.context.spins.board = generate_board_5x3(a_game.settings.reels.get(&format!("spins_{}", a_game.context.spins.spin_type.unwrap_or(0))).ok_or_else(|| utils::err_http_responses("BadRequest", "ErrorInternalServerError.", "ERR_INTERNAL_ERROR1"))?);
            }

            a_game.context.spins.bac_win = Some(false);
            a_game.context.spins.bonus_mechanic = None;
            let mut l_mechanic_pushed1 = false;
            let mut l_mechanic_pushed2 = false;
            let mut l_mechanic_pushed3 = false;
            let mut l_bonus_mechanic: Option<Vec<i32>> = None;
            for x in 0..BOARD_WIDTH {
                for y in 0..BOARD_HEIGHT {
                    if a_game.context.spins.board[x][y] == 11 {
                        inc_bac(&mut a_game.context.spins.bac.field1);
                        if !l_mechanic_pushed1 {l_bonus_mechanic.get_or_insert_with(Vec::new).push(1); l_mechanic_pushed1 = true;}
                    }
                    else if a_game.context.spins.board[x][y] == 12 {
                        inc_bac(&mut a_game.context.spins.bac.field2);
                        if !l_mechanic_pushed2 {l_bonus_mechanic.get_or_insert_with(Vec::new).push(2); l_mechanic_pushed2 = true;}
                    }
                    else if a_game.context.spins.board[x][y] == 13 {
                        inc_bac(&mut a_game.context.spins.bac.field3);
                        if !l_mechanic_pushed3 {l_bonus_mechanic.get_or_insert_with(Vec::new).push(3); l_mechanic_pushed3 = true;}
                    }
                }
            }
            if l_mechanic_pushed1 || l_mechanic_pushed2 || l_mechanic_pushed3 {
                if cfg!(test) {
                    a_game.context.spins.bac_win = expected_response.as_ref().and_then(|json| json.get("context")).and_then(|context| context.get("spins")).and_then(|spins| spins.get("bac_win")).and_then(|value| value.as_bool());
                    a_game.context.spins.bonus_mechanic = expected_response.as_ref().and_then(|json| json.get("context")).and_then(|context| context.get("spins")).and_then(|spins| spins.get("bonus_mechanic")).and_then(|value| value.as_array()).map(|array| {array.iter().filter_map(|v| v.as_i64().map(|num| num as i32)).collect::<Vec<i32>>()});
                } else {
                    let l_random_bac = rand::thread_rng().gen_range(0..=sum_upto_n(39));
                    if l_random_bac < sum_upto_n(a_game.context.spins.bac.field1[0] + a_game.context.spins.bac.field2[0] + a_game.context.spins.bac.field3[0] as i32) {
                        a_game.context.spins.bac_win = Some(true); 
                        a_game.context.spins.bonus_mechanic = l_bonus_mechanic.clone();
                    }
                }
            }

            a_game.context.spins.winlines = None;
            a_game.context.spins.round_win = 0;
            let mut l_checking_symbol;
            let mut l_checking_lenght;
            let mut l_checking_positions: Vec<[i32; 2]> = Vec::new();
            for l in 0..BOARD_LINES_COUNT {
                l_checking_symbol = a_game.context.spins.board[0][a_game.settings.paylines[l][0] as usize];
                l_checking_lenght = 1;
                l_checking_positions.push([0, a_game.settings.paylines[l][0].clone()]);

                for x in 1..BOARD_WIDTH {
                    if a_game.settings.symbols_wild.contains(&l_checking_symbol) {l_checking_symbol = a_game.context.spins.board[x][a_game.settings.paylines[l][x] as usize]; l_checking_lenght += 1; l_checking_positions.push([x as i32, a_game.settings.paylines[l][x].clone()]); continue;}
                    if l_checking_symbol == a_game.context.spins.board[x][a_game.settings.paylines[l][x] as usize] 
                    || a_game.settings.symbols_wild.contains(&a_game.context.spins.board[x][a_game.settings.paylines[l][x] as usize].clone()) 
                    {l_checking_positions.push([x as i32, a_game.settings.paylines[l][x].clone()]); l_checking_lenght += 1;}
                    else{break;}
                }
                if a_game.settings.symbols_line.contains(&l_checking_symbol) { 
                    let l_paytable_symbol = a_game.settings.paytable.get(&format!("{}", l_checking_symbol)).ok_or_else(|| utils::err_http_responses("BadRequest", "ErrorInternalServerError.", "ERR_INTERNAL_ERROR2"))?;
                    if l_paytable_symbol.len() > 0 {
                        for occ in 0..l_paytable_symbol.len() {
                            if l_paytable_symbol[occ].occurrences == l_checking_lenght {
                                a_game.context.spins.winlines.get_or_insert_with(Vec::new).push(model::Winline {
                                    amount: (l_bet_per_line.as_i64().unwrap_or(0) as i32 * l_paytable_symbol[occ].multiplier) as i64, 
                                    line: (l+1) as i32, 
                                    occurrences: l_paytable_symbol[occ].occurrences, 
                                    positions: l_checking_positions.clone(), 
                                    symbol: l_checking_symbol, 
                                    winline_type: "lb".to_string()
                                });
                                a_game.context.spins.round_win += (l_bet_per_line.as_i64().unwrap_or(0) as i32 * l_paytable_symbol[occ].multiplier) as i64;
                            }
                        }
                    }
                }
                l_checking_positions.clear();
            }
            if let Some(ref mut winlines) = a_game.context.spins.winlines {
                winlines.sort_by(|a, b| b.amount.cmp(&a.amount));
            }
            a_game.context.spins.total_win = Some(a_game.context.spins.round_win);

            a_game.context.spins.bet_per_line = l_bet_per_line.as_i64().unwrap_or(0) as i32;
            a_game.context.spins.lines = l_lines;
            a_game.context.spins.round_bet = (a_game.context.spins.bet_per_line * a_game.settings.bet_factor[0]) as i32;
            a_game.context.spins.is_lucky_spin = false;
            a_game.context.spins.selected_mode = Some("0".to_string());
            //a_game.context.spins.selected_mode = a_request.action.params.selected_mode.clone();

            if a_game.context.spins.bac_win.unwrap_or(false) {
                a_game.context.actions = ["bonus_init".to_string()].into_iter().map(Into::into).collect();
            } else {
                a_game.context.actions = ["spin".to_string(), "buy_spin".to_string()].into_iter().map(Into::into).collect();
            };
            a_game.context.current = "spins".to_string();
            a_game.context.last_action = a_request.action.name.clone();
            a_game.context.last_args.bet_per_line = Some(l_bet_per_line.as_i64().unwrap_or(0) as i32);
            a_game.context.last_args.lines = a_request.action.params.lines;
            if a_game.context.spins.total_win > Some(0) {a_game.context.last_win = a_game.context.spins.total_win};
            a_game.context.round_finished = !a_game.context.spins.bac_win.unwrap_or(false);
            a_game.context.version = 1;
            a_game.origin_data.feature = false;
            a_game.origin_data.autogame = a_request.autogame;
            a_game.origin_data.mobile = a_request.mobile.clone();
            a_game.origin_data.portrait = a_request.portrait;
            a_game.origin_data.quickspin = a_request.quick_spin;
            a_game.origin_data.set_denominator = a_request.set_denominator;
            a_game.origin_data.sound = a_request.sound;
            a_game.request_id = a_request.request_id.clone();
            a_game.status.code = "OK".to_string();
            a_game.status.status_type = None;
            a_game.user.balance -= (a_game.context.spins.bet_per_line * a_game.settings.bet_factor[0]) as i64;
            a_game.user.balance += a_game.context.spins.total_win.unwrap_or(0);
            a_game.user.balance_version += 1;
        } else {
            //a_game.context = None;
            a_game.request_id = a_request.request_id.clone();
            a_game.status.code = "FUNDS_EXCEED".to_string(); /* BET_LIMIT_ERROR, FUNDS_EXCEED */
            a_game.status.status_type = Some("exceed".to_string()); /* crit, exceed */
        }
    } else {
        //a_game.context = None;
        a_game.request_id = a_request.request_id.clone();
        a_game.status.code = "ACTION_ERROR".to_string(); /* BET_LIMIT_ERROR, FUNDS_EXCEED */
        a_game.status.status_type = Some("crit".to_string()); /* crit, exceed */
    };
    Ok(())
}

fn generate_board_5x3(reels: &[Vec<i32>; BOARD_WIDTH]) -> [[i32; BOARD_HEIGHT]; BOARD_WIDTH] {
    let mut rng = rand::thread_rng();
    let mut board: [[i32; BOARD_HEIGHT]; BOARD_WIDTH] = [[0; BOARD_HEIGHT]; BOARD_WIDTH];
    for x in 0..BOARD_WIDTH {
        let reel = &reels[x];
        let reel_len = reel.len();
        if reel_len > 0 {
            let start_pos = rng.gen_range(0..reel_len);
            for y in 0..BOARD_HEIGHT {
                let index = (start_pos + y) % reel_len;
                board[x][y] = reel[index] as i32;
            }
        }
    }
    board
}

fn inc_bac(a_bac: &mut [i32; 2]) { 
    if a_bac[1] != 14 {
        if a_bac[1] < a_bac[0] {a_bac[1] += 1;} 
        else {a_bac[0] += 1; a_bac[1] = 0;}
    }
}

fn sum_upto_n(n: i32) -> i32 {
    n * (n + 1) / 2
}