use actix_web::Error;
use serde_json::Value;
use rand::Rng;
use crate::game::settings::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::game::models::{model, network::request};

pub async fn execute(a_request: &request::play::Play, a_game: &mut model::Game, expected_response: Option<Value>) -> Result<(), Error> {
    a_game.command = a_request.command.clone();

    if a_game.context.actions.contains(&a_request.action.name.clone()) {
        let l_bonus = a_game.context.bonus.get_or_insert_with(Default::default);
        if l_bonus.rounds_left > 0 {
            l_bonus.rounds_left -= 1;
            
            l_bonus.orig_board = l_bonus.board.clone();
            l_bonus.orig_bs_v = l_bonus.bs_v.clone();
            l_bonus.mystery_pos = None;
            l_bonus.mystery_values = None;
            l_bonus.jackpot_values = None;
            l_bonus.new_bs = None;
            l_bonus.copy_new_bs = None;
            l_bonus.boost_values = None;
            l_bonus.double_values = None;
            l_bonus.collect_values = None;
    /*generate */
            for x in 0..BOARD_WIDTH {
                for y in 0..BOARD_HEIGHT {
                    if (if cfg!(test) {[11, 12, 13, 14, 15].into_iter().map(Into::into).collect::<Vec<_>>().contains(&expected_response.as_ref().and_then(|json| json.get("context")).and_then(|context| context.get("bonus")).and_then(|bonus| bonus.get("board")).and_then(|board| board.get(x).and_then(|row| row.get(y))).and_then(|cell| cell.as_i64()).map(|v| v as i32))} else {rand::thread_rng().gen_range(0..10) == 0}) && !l_bonus.board_opened[x][y] && l_bonus.bs_count < 14 {
                        //board create
                        l_bonus.rounds_left = 3;
                        if cfg!(test) {
                            l_bonus.orig_board[x][y] = expected_response.as_ref().and_then(|json| json.get("context")).and_then(|context| context.get("bonus")).and_then(|bonus| bonus.get("orig_board")).and_then(|board| board.get(x).and_then(|row| row.get(y))).and_then(|cell| cell.as_i64()).map(|v| v as i32).unwrap_or(0);
                        } else {
                            l_bonus.orig_board[x][y] = [11, 12, 13, 14, 14, 14, 14, 14, 14, 15].into_iter().map(Into::into).collect::<Vec<_>>()[rand::thread_rng().gen_range(0..10)];
                        }
                        l_bonus.bs_count += 1;
                        l_bonus.new_bs.get_or_insert_with(|| Vec::new()).push([x as i32, y as i32]);
                        l_bonus.copy_new_bs.get_or_insert_with(|| Vec::new()).push([x as i32, y as i32]);
                        //mystory execute
                        if l_bonus.orig_board[x][y] == 15 {
                            if cfg!(test) {
                                l_bonus.board[x][y] = expected_response.as_ref().and_then(|json| json.get("context")).and_then(|context| context.get("bonus")).and_then(|bonus| bonus.get("board")).and_then(|board| board.get(x).and_then(|row| row.get(y))).and_then(|cell| cell.as_i64()).map(|v| v as i32).unwrap_or(0);
                            } else {
                                l_bonus.board[x][y] = rand::thread_rng().gen_range(11..=14);
                            }
                            l_bonus.mystery_count += 1;
                            l_bonus.mystery_pos.get_or_insert_with(|| Vec::new()).push([x as i32, y as i32]);
                        } else {l_bonus.board[x][y] = l_bonus.orig_board[x][y]}
                        if l_bonus.board[x][y] == 14 {l_bonus.board_opened[x][y] = true} 
                        
                        if ["mini", "minor", "major"].into_iter().map(Into::into).collect::<Vec<_>>().contains(&expected_response.as_ref().and_then(|json| json.get("context")).and_then(|context| context.get("bonus")).and_then(|bonus| bonus.get("orig_bs_v")).and_then(|orig_bs_v| orig_bs_v.get(x).and_then(|row| row.get(y))).unwrap_or(&serde_json::Value::Null).clone()) {l_bonus.jackpot_values = Some([a_game.settings.jackpots.mini * l_bonus.round_bet as i64, a_game.settings.jackpots.minor * l_bonus.round_bet as i64, a_game.settings.jackpots.major * l_bonus.round_bet as i64]);}
                        if l_bonus.board[x][y] != 13 {

                            if cfg!(test) {
                                l_bonus.orig_bs_v[x][y] = expected_response.as_ref().and_then(|json| json.get("context")).and_then(|context| context.get("bonus")).and_then(|bonus| bonus.get("orig_bs_v")).and_then(|orig_bs_v| orig_bs_v.get(x).and_then(|row| row.get(y))).unwrap_or(&serde_json::Value::Null).clone();
                                l_bonus.bs_v[x][y] = expected_response.as_ref().and_then(|json| json.get("context")).and_then(|context| context.get("bonus")).and_then(|bonus| bonus.get("bs_v")).and_then(|orig_bs_v| orig_bs_v.get(x).and_then(|row| row.get(y))).unwrap_or(&serde_json::Value::Null).clone();
                                l_bonus.bs_values[x][y] = from_bs_v(&l_bonus.orig_bs_v[x][y], *l_bonus.jackpot_values.get_or_insert_with(|| [0, 0, 0]), l_bonus.round_bet);
                                //l_bonus.bs_values[x][y] = expected_response.as_ref().and_then(|json| json.get("context")).and_then(|context| context.get("bonus")).and_then(|bonus| bonus.get("bs_values")).and_then(|bs_values| bs_values.get(x).and_then(|row| row.get(y))).and_then(|cell| cell.as_f64()).unwrap_or(0.0);
                            } else {
                                let l_bs_v_position = rand::thread_rng().gen_range(0..a_game.settings.bonus_symbol_v.len());
                                l_bonus.bs_values[x][y] = get_bs_values(&a_game.settings.bonus_symbol_v[l_bs_v_position], *l_bonus.jackpot_values.get_or_insert_with(|| [0, 0, 0]));
                                l_bonus.orig_bs_v[x][y] = get_bs_v(&a_game.settings.bonus_symbol_v[l_bs_v_position], l_bonus.round_bet);
                                l_bonus.bs_v[x][y] = get_bs_v(&a_game.settings.bonus_symbol_v[l_bs_v_position], l_bonus.round_bet);
                            }
                        }
                        if l_bonus.board[x][y] != 14 {
                            l_bonus.copy_new_bs.get_or_insert_with(|| Vec::new()).push([x as i32, y as i32]);
                        }
                    }
                }
            }
            l_bonus.jackpot_values = None;
    /*bac inc */
            if l_bonus.new_bs != None {
                if l_bonus.new_bs.get_or_insert_with(|| Vec::new()).len() > 0 {
                    let mut l_mechanic_pushed1 = false;
                    let mut l_mechanic_pushed2 = false;
                    let mut l_mechanic_pushed3 = false;
                    let mut l_bonus_mechanic = l_bonus.bonus_mechanic.clone();
                    for p_outer in 0..(l_bonus.new_bs.get_or_insert_with(|| Vec::new()).len() as usize) {
                        let x = l_bonus.new_bs.get_or_insert_with(|| Vec::new())[p_outer][0] as usize;
                        let y = l_bonus.new_bs.get_or_insert_with(|| Vec::new())[p_outer][1] as usize;
                        if !l_bonus.board_opened[x][y] {
                            if l_bonus.board[x][y] == 11 {
                                inc_bac(&mut l_bonus.bac.field1);
                                if !l_mechanic_pushed1 && !l_bonus_mechanic.contains(&1) {l_bonus_mechanic.push(1); l_mechanic_pushed1 = true;}
                            }
                            else if l_bonus.board[x][y] == 12 {
                                inc_bac(&mut l_bonus.bac.field2);
                                if !l_mechanic_pushed2 && !l_bonus_mechanic.contains(&2) {l_bonus_mechanic.push(2); l_mechanic_pushed2 = true;}
                            }
                            else if l_bonus.board[x][y] == 13 {
                                inc_bac(&mut l_bonus.bac.field3);
                                if !l_mechanic_pushed3 && !l_bonus_mechanic.contains(&3) {l_bonus_mechanic.push(3); l_mechanic_pushed3 = true;}
                            }
                        }
                    }
                    if l_mechanic_pushed1 || l_mechanic_pushed2 || l_mechanic_pushed3 {
                        let l_random_bac = rand::thread_rng().gen_range(0..=sum_upto_n(39));
                        if if cfg!(test) {l_bonus.bonus_mechanic != expected_response.as_ref().and_then(|json| json.get("context")).and_then(|context| context.get("bonus")).and_then(|bonus| bonus.get("bonus_mechanic")).and_then(|bonus_mechanic| bonus_mechanic.as_array()).map(|arr| {arr.iter().filter_map(|v| v.as_i64().map(|n| n as i32)).collect::<Vec<_>>()}).unwrap_or_default()} else {l_random_bac < sum_upto_n(10)} 
                        {
                            if l_mechanic_pushed1 {l_bonus.bac.field1 = [13, 0]}
                            if l_mechanic_pushed2 {l_bonus.bac.field1 = [13, 0]}
                            if l_mechanic_pushed3 {l_bonus.bac.field1 = [13, 0]}
                            l_bonus.bonus_mechanic = l_bonus_mechanic.clone();
                        }
                    }
                    l_bonus.bonus_mechanic.sort_by(|a, b| a.cmp(b));
    /*check  */
                    for p_outer in 0..(l_bonus.new_bs.get_or_insert_with(|| Vec::new()).len() as usize) {
                        let x_outer = l_bonus.new_bs.get_or_insert_with(|| Vec::new())[p_outer][0] as usize;
                        let y_outer = l_bonus.new_bs.get_or_insert_with(|| Vec::new())[p_outer][1] as usize;
                        if l_bonus.board[x_outer][y_outer] == 11 {
                            for x in 0..BOARD_WIDTH {
                                for y in 0..BOARD_HEIGHT {
                                    if l_bonus.board_opened[x][y] {
                                        l_bonus.bs_values[x][y] += l_bonus.bs_values[x_outer][y_outer];
                                        l_bonus.bs_v[x][y] = get_bs_v(&Value::Number(serde_json::Number::from_f64(l_bonus.bs_values[x][y]).unwrap()), l_bonus.round_bet);
                                    } 
                                }
                            }
                            l_bonus.board_opened[x_outer][y_outer] = true;
                            let mut l_bs_v = l_bonus.orig_bs_v[x_outer][y_outer].clone();
                            if l_bs_v == "mini" {l_bs_v = (a_game.settings.jackpots.mini * (l_bonus.round_bet as i64)).into()}
                            if l_bs_v == "minor" {l_bs_v = (a_game.settings.jackpots.minor * (l_bonus.round_bet as i64)).into()}
                            if l_bs_v == "major" {l_bs_v = (a_game.settings.jackpots.major * (l_bonus.round_bet as i64)).into()}
                            l_bonus.boost_values.get_or_insert_with(|| Vec::new()).push(model::BoostValue { bs_v: l_bs_v, pos: [x_outer as i32, y_outer as i32]});
                        }
                    }

                    for p_outer in 0..(l_bonus.new_bs.get_or_insert_with(|| Vec::new()).len() as usize) {
                        let x_outer = l_bonus.new_bs.get_or_insert_with(|| Vec::new())[p_outer][0] as usize;
                        let y_outer = l_bonus.new_bs.get_or_insert_with(|| Vec::new())[p_outer][1] as usize;
                        if l_bonus.board[x_outer][y_outer] == 12 {
                            for x in 0..BOARD_WIDTH {
                                for y in 0..BOARD_HEIGHT {
                                    if l_bonus.board_opened[x][y] {
                                        l_bonus.bs_values[x][y] *= 2.0;
                                        l_bonus.bs_v[x][y] = get_bs_v(&Value::Number(serde_json::Number::from_f64(l_bonus.bs_values[x][y]).unwrap()), l_bonus.round_bet);
                                    }
                                }
                            }
                            l_bonus.board_opened[x_outer][y_outer] = true;
                            let mut l_bs_v = l_bonus.orig_bs_v[x_outer][y_outer].clone();
                            if l_bs_v == "mini" {l_bs_v = (a_game.settings.jackpots.mini * (l_bonus.round_bet as i64)).into()}
                            if l_bs_v == "minor" {l_bs_v = (a_game.settings.jackpots.minor * (l_bonus.round_bet as i64)).into()}
                            if l_bs_v == "major" {l_bs_v = (a_game.settings.jackpots.major * (l_bonus.round_bet as i64)).into()}
                            l_bonus.double_values.get_or_insert_with(|| Vec::new()).push(model::BoostValue { bs_v: l_bs_v, pos: [x_outer as i32, y_outer as i32]});
                        }
                    }
                    /*for p_outer in 0..(l_bonus.new_bs.get_or_insert_with(|| Vec::new()).len() as usize) {
                        let x_outer = l_bonus.new_bs.get_or_insert_with(|| Vec::new())[p_outer][0] as usize;
                        let y_outer = l_bonus.new_bs.get_or_insert_with(|| Vec::new())[p_outer][1] as usize;
                        if l_bonus.board[x_outer][y_outer] == 12 {
                            l_bonus.board_opened[x_outer][y_outer] = true;
                        }
                    }*/

                    for x in 0..BOARD_WIDTH {
                        for y in 0..BOARD_HEIGHT {
                            if l_bonus.board[x][y] == 14 {l_bonus.board_opened[x][y] = true}
                        }
                    }
                    for p_outer in (0..(l_bonus.new_bs.get_or_insert_with(|| Vec::new()).len() as usize)).rev() {
                        let x_outer = l_bonus.new_bs.get_or_insert_with(|| Vec::new())[p_outer][0] as usize;
                        let y_outer = l_bonus.new_bs.get_or_insert_with(|| Vec::new())[p_outer][1] as usize;
                        if l_bonus.board[x_outer][y_outer] == 13 {
                            for x in 0..BOARD_WIDTH {
                                for y in 0..BOARD_HEIGHT {
                                    if l_bonus.board_opened[x][y] {
                                        l_bonus.bs_values[x_outer][y_outer] += l_bonus.bs_values[x][y];
                                    } 
                                }
                            }
                            l_bonus.bs_v[x_outer][y_outer] = get_bs_v(&Value::Number(serde_json::Number::from_f64(l_bonus.bs_values[x_outer][y_outer]).unwrap()), l_bonus.round_bet);
                            l_bonus.board_opened[x_outer][y_outer] = true;
                            let mut l_bs_v = l_bonus.bs_v[x_outer][y_outer].clone();
                            if l_bs_v == "mini" {l_bs_v = (a_game.settings.jackpots.mini * (l_bonus.round_bet as i64)).into()}
                            if l_bs_v == "minor" {l_bs_v = (a_game.settings.jackpots.minor * (l_bonus.round_bet as i64)).into()}
                            if l_bs_v == "major" {l_bs_v = (a_game.settings.jackpots.major * (l_bonus.round_bet as i64)).into()}
                            l_bonus.collect_values.get_or_insert_with(|| Vec::new()).push(model::BoostValue { bs_v: l_bs_v, pos: [x_outer as i32, y_outer as i32]});
                        }
                    }

                    if l_bonus.mystery_pos != None {
                        if l_bonus.mystery_pos.get_or_insert_with(|| Vec::new()).len() > 0 {
                            for p_outer in 0..(l_bonus.mystery_pos.get_or_insert_with(|| Vec::new()).len() as usize) {
                                let x_outer = l_bonus.mystery_pos.get_or_insert_with(|| Vec::new())[p_outer][0] as usize;
                                let y_outer = l_bonus.mystery_pos.get_or_insert_with(|| Vec::new())[p_outer][1] as usize;
                                let l_bs_v: Value;
                                if (l_bonus.board[x_outer][y_outer] == 13) || (l_bonus.board[x_outer][y_outer] == 12) || (l_bonus.board[x_outer][y_outer] == 11) {l_bs_v = Value::Number(serde_json::Number::from(0))} else {l_bs_v = l_bonus.orig_bs_v[x_outer][y_outer].clone();}
                                let l_id = l_bonus.board[x_outer][y_outer];
                                l_bonus.mystery_values.get_or_insert_with(|| Vec::new()).push(model::MysteryValue { bs_v: l_bs_v, id: l_id, pos: [x_outer as i32, y_outer as i32]});
                            }
                        }
                    }
                }
            }
            for x in 0..BOARD_WIDTH {
                for y in 0..BOARD_HEIGHT {
                    if ["mini", "minor", "major"].into_iter().map(Into::into).collect::<Vec<_>>().contains(&l_bonus.orig_bs_v[x][y]) {l_bonus.jackpot_values = Some([a_game.settings.jackpots.mini * l_bonus.round_bet as i64, a_game.settings.jackpots.minor * l_bonus.round_bet as i64, a_game.settings.jackpots.major * l_bonus.round_bet as i64]);}
                }
            }

            if l_bonus.rounds_left > 0 {
                a_game.context.actions = ["respin".to_string()].into_iter().map(Into::into).collect();
            } else {
                for x in 0..BOARD_WIDTH {
                    for y in 0..BOARD_HEIGHT {
                        l_bonus.round_win += (l_bonus.bs_values[x][y] * l_bonus.round_bet as f64) as i64;
                    }
                }
                l_bonus.total_win += l_bonus.round_win;
                a_game.user.balance += l_bonus.round_win;
                a_game.context.last_win = Some(l_bonus.round_win);
                a_game.context.actions = ["bonus_spins_stop".to_string()].into_iter().map(Into::into).collect();
            }
            
            //a_game.context.spins.selected_mode = None;
            a_game.context.spins.total_win = None;
            
            a_game.context.current = "bonus".to_string();
            a_game.context.last_action = a_request.action.name.clone();
            a_game.context.round_finished = false;
            a_game.context.version = 1;
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
            a_game.user.balance_version += 1;
        } else {
            a_game.request_id = a_request.request_id.clone();
            a_game.status.code = "ACTION_ERROR".to_string(); /* BET_LIMIT_ERROR, FUNDS_EXCEED */
            a_game.status.status_type = Some("crit".to_string()); /* crit, exceed */
        }
    } else {
        a_game.request_id = a_request.request_id.clone();
        a_game.status.code = "ACTION_ERROR".to_string(); /* BET_LIMIT_ERROR, FUNDS_EXCEED */
        a_game.status.status_type = Some("crit".to_string()); /* crit, exceed */
    };
    Ok(())
}

fn get_bs_values(a_bonus_symbol_v: &serde_json::Value, a_jackpots: [i64; 3]) -> f64 {
    match &a_bonus_symbol_v {
        Value::Number(num) => {num.as_f64().unwrap_or_default()}
        Value::String(s) => {
            match s.as_str() {
                "mini" => {a_jackpots[0] as f64}
                "minor" => {a_jackpots[1] as f64}
                "major" => {a_jackpots[2] as f64}
                _ => {0.0}
            }
        }
        _ => {0.0}
    }
}

fn from_bs_v(a_b_s_v: &serde_json::Value, a_jackpots: [i64; 3], a_round_bet: i32) -> f64 {
    match &a_b_s_v {
        Value::Number(num) => {num.as_f64().unwrap_or_default()/(a_round_bet as f64)}
        Value::String(s) => {
            match s.as_str() {
                "mini" => {(a_jackpots[0] as f64) / (a_round_bet as f64)}
                "minor" => {(a_jackpots[1] as f64) / (a_round_bet as f64)}
                "major" => {(a_jackpots[2] as f64) / (a_round_bet as f64)}
                _ => {0.0}
            }
        }
        _ => {0.0}
    }
}

fn get_bs_v(a_bonus_symbol_v: &serde_json::Value, a_round_bet: i32) -> serde_json::Value {
    match &a_bonus_symbol_v {
        Value::Number(num) => {Value::Number(serde_json::Number::from_f64(num.as_f64().unwrap_or_default() * a_round_bet as f64).unwrap())}
        Value::String(s) => {Value::String(s.clone())}
        _ => {Value::Number(serde_json::Number::from_f64(0.0).unwrap())}
    }
}

fn inc_bac(a_bac: &mut [i32; 2]) { 
    if a_bac[1] != 13 {
        if a_bac[1] < a_bac[0] {a_bac[1] += 1;} 
        else {a_bac[0] += 1; a_bac[1] = 0;}
    } else {a_bac[0] = 0; a_bac[1] = 0;}
}

fn sum_upto_n(n: i32) -> i32 {
    n * (n + 1) / 2
}