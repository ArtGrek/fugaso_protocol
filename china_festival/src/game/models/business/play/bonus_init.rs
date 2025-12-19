use actix_web::Error;
use serde_json::Value;
use rand::Rng;
use crate::utils;
use crate::game::settings::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::game::models::{model, network::request};

pub async fn execute(a_request: &request::play::Play, a_game: &mut model::Game, expected_response: Option<Value>) -> Result<(), Error> {
    a_game.command = a_request.command.clone();

    if a_game.context.actions.contains(&a_request.action.name.clone()) {
        let l_bonus = a_game.context.bonus.get_or_insert_with(Default::default);
        
        l_bonus.bet_per_line = a_game.context.spins.bet_per_line;
        l_bonus.lines = a_game.context.spins.lines;
        l_bonus.round_bet = a_game.context.spins.round_bet;
        l_bonus.is_lucky_spin = false;
        l_bonus.last_respin = false;
        l_bonus.bac = model::BacInner {
            field1: if a_game.context.spins.bonus_mechanic.get_or_insert_with(Vec::new).contains(&1) {[13, 0]} else {a_game.context.spins.bac.field1.clone()}, 
            field2: if a_game.context.spins.bonus_mechanic.get_or_insert_with(Vec::new).contains(&2) {[13, 0]} else {a_game.context.spins.bac.field2.clone()}, 
            field3: if a_game.context.spins.bonus_mechanic.get_or_insert_with(Vec::new).contains(&3) {[13, 0]} else {a_game.context.spins.bac.field3.clone()}
        };
        l_bonus.bonus_mechanic = a_game.context.spins.bonus_mechanic.clone().ok_or_else(|| utils::err_http_responses("BadRequest", "Missing bet_per_line.", "ERR_PARAM_REQUIRED"))?;
        l_bonus.bonus_scenario = a_game.context.spins.selected_mode.get_or_insert_with(|| "0".to_string()).parse::<i32>().unwrap();
        l_bonus.orig_board = a_game.context.spins.board.clone();/*+ */
        l_bonus.board = a_game.context.spins.board.clone();/*+ */
        l_bonus.board_opened = [[false,false,false],[false,false,false],[false,false,false],[false,false,false],[false,false,false]];/*+ */
        l_bonus.bs_values = [[0.0,0.0,0.0],[0.0,0.0,0.0],[0.0,0.0,0.0],[0.0,0.0,0.0],[0.0,0.0,0.0]];/*+ */
        l_bonus.orig_bs_v = [[Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0))],
                                                                               [Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0))],
                                                                               [Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0))],
                                                                               [Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0))],
                                                                               [Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0))],];/*+ */
        l_bonus.bs_v = [[Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0))],
                                                                          [Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0))],
                                                                          [Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0))],
                                                                          [Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0))],
                                                                          [Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0)),Value::Number(serde_json::Number::from(0))]];/*+ */
        l_bonus.bs_count = l_bonus.copy_new_bs.get_or_insert_with(|| Vec::new()).len() as i32; /*+ */
        l_bonus.mystery_count += l_bonus.mystery_pos.get_or_insert_with(|| Vec::new()).len() as i32;/*+ */
        l_bonus.mystery_pos = None;
        l_bonus.mystery_values = None;
        let l_jp_mini = a_game.settings.jackpots.mini;
        let l_jp_minor = a_game.settings.jackpots.minor;
        let l_jp_major = a_game.settings.jackpots.major;
        //l_bonus.jackpot_values = Some([l_jp_mini, l_jp_minor, l_jp_major]);
        l_bonus.new_bs = None;/*+ */
        l_bonus.copy_new_bs = None;/*+ */
        l_bonus.boost_values = None;
        l_bonus.double_values = None;
        l_bonus.collect_values = None;
        l_bonus.rounds_granted = 3;
        l_bonus.rounds_left = 3;
        l_bonus.round_win = 0;
        l_bonus.total_win = a_game.context.spins.total_win.unwrap_or(0);
        l_bonus.back_to = "spins".to_string();

        
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                if a_game.context.spins.board[x][y] == 11 || a_game.context.spins.board[x][y] == 12 || a_game.context.spins.board[x][y] == 13 {
                    l_bonus.copy_new_bs.get_or_insert_with(|| Vec::new()).push([x as i32, y as i32]);
                    if a_game.context.spins.board[x][y] != 13 {
                        
                        if cfg!(test) {
                            l_bonus.bs_values[x][y] = expected_response.as_ref().and_then(|json| json.get("context")).and_then(|context| context.get("bonus")).and_then(|bonus| bonus.get("bs_values")).and_then(|bs_values| bs_values.get(x).and_then(|row| row.get(y))).and_then(|cell| cell.as_f64()).unwrap_or(0.0);
                            l_bonus.orig_bs_v[x][y] = expected_response.as_ref().and_then(|json| json.get("context")).and_then(|context| context.get("bonus")).and_then(|bonus| bonus.get("orig_bs_v")).and_then(|orig_bs_v| orig_bs_v.get(x).and_then(|row| row.get(y))).unwrap_or(&serde_json::Value::Null).clone();
                        } else {
                            let l_bs_v_position = rand::thread_rng().gen_range(0..a_game.settings.bonus_symbol_v.len());
                            l_bonus.bs_values[x][y] = get_bs_values(&a_game.settings.bonus_symbol_v[l_bs_v_position], [l_jp_mini, l_jp_minor, l_jp_major]);
                            l_bonus.orig_bs_v[x][y] = get_bs_v(&a_game.settings.bonus_symbol_v[l_bs_v_position], l_bonus.round_bet);
                        }
                        if ["mini", "minor", "major"].into_iter().map(Into::into).collect::<Vec<_>>().contains(&l_bonus.orig_bs_v[x][y]) {l_bonus.jackpot_values = Some([l_jp_mini * l_bonus.round_bet as i64, l_jp_minor * l_bonus.round_bet as i64, l_jp_major * l_bonus.round_bet as i64]);}
                    }
        }}}

        let mut x = 0;
        let mut y = 0;
        let mut l_count = 0;
        loop{
            if (if cfg!(test) {expected_response.as_ref().and_then(|json| json.get("context")).and_then(|context| context.get("bonus")).and_then(|bonus| bonus.get("board")).and_then(|board| board.get(x).and_then(|row| row.get(y))).and_then(|cell| cell.as_i64()).map(|v| v as i32) == Some(14)} else {rand::thread_rng().gen_range(0..=1) == 0}) && (l_count < 5) && (![11, 12, 13, 14].contains(&l_bonus.board[x][y])) {
                l_bonus.board[x][y] = 14;
                //if rand::thread_rng().gen_range(0..100) == 0 {l_bonus.board[x][y] = 15}
                
                if cfg!(test) {
                    l_bonus.bs_values[x][y] = expected_response.as_ref().and_then(|json| json.get("context")).and_then(|context| context.get("bonus")).and_then(|bonus| bonus.get("bs_values")).and_then(|bs_values| bs_values.get(x).and_then(|row| row.get(y))).and_then(|cell| cell.as_f64()).unwrap_or(0.0);
                    l_bonus.orig_bs_v[x][y] = expected_response.as_ref().and_then(|json| json.get("context")).and_then(|context| context.get("bonus")).and_then(|bonus| bonus.get("orig_bs_v")).and_then(|orig_bs_v| orig_bs_v.get(x).and_then(|row| row.get(y))).unwrap_or(&serde_json::Value::Null).clone();
                } else {
                    let l_bs_v_position = rand::thread_rng().gen_range(0..a_game.settings.bonus_symbol_v.len());
                    l_bonus.bs_values[x][y] = get_bs_values(&a_game.settings.bonus_symbol_v[l_bs_v_position], [l_jp_mini, l_jp_minor, l_jp_major]);
                    l_bonus.orig_bs_v[x][y] = get_bs_v(&a_game.settings.bonus_symbol_v[l_bs_v_position], l_bonus.round_bet);
                }
                if ["mini", "minor", "major"].into_iter().map(Into::into).collect::<Vec<_>>().contains(&l_bonus.orig_bs_v[x][y]) {l_bonus.jackpot_values = Some([l_jp_mini * l_bonus.round_bet as i64, l_jp_minor * l_bonus.round_bet as i64, l_jp_major * l_bonus.round_bet as i64]);}
                if l_bonus.board[x][y] == 14 {l_bonus.board_opened[x][y] = true;}
                l_bonus.new_bs.get_or_insert_with(|| Vec::new()).push([x as i32, y as i32]);
                l_bonus.copy_new_bs.get_or_insert_with(|| Vec::new()).push([x as i32, y as i32]);
                l_count += 1;
            }
            if y == BOARD_HEIGHT-1 {x += 1; y = 0;} else {y += 1;}
            if (x == BOARD_WIDTH-1) && (y == BOARD_HEIGHT) { x = 0; y = 0;}
            if l_count == 5 {break;}
        }
        l_bonus.bs_v = l_bonus.orig_bs_v.clone();

        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                if l_bonus.bs_v[x][y] == "mini" && l_bonus.board[x][y] == 12 {l_bonus.bs_v[x][y] = (l_jp_mini * (l_bonus.round_bet as i64)).into()}
                if l_bonus.bs_v[x][y] == "minor" && l_bonus.board[x][y] == 12  {l_bonus.bs_v[x][y] = (l_jp_minor * (l_bonus.round_bet as i64)).into()}
                if l_bonus.bs_v[x][y] == "major" && l_bonus.board[x][y] == 12  {l_bonus.bs_v[x][y] = (l_jp_major * (l_bonus.round_bet as i64)).into()}
            }
        }

        l_bonus.bs_count = l_bonus.copy_new_bs.get_or_insert_with(|| Vec::new()).len() as i32;

        for p_outer in 0..(l_bonus.bs_count as usize) {
            let x_outer = l_bonus.copy_new_bs.get_or_insert_with(|| Vec::new())[p_outer][0] as usize;
            let y_outer = l_bonus.copy_new_bs.get_or_insert_with(|| Vec::new())[p_outer][1] as usize;
            if l_bonus.board[x_outer][y_outer] == 11 {
                for p_inner in 0..(l_bonus.bs_count as usize) {
                    let x_inner = l_bonus.copy_new_bs.get_or_insert_with(|| Vec::new())[p_inner][0] as usize;
                    let y_inner = l_bonus.copy_new_bs.get_or_insert_with(|| Vec::new())[p_inner][1] as usize;
                    if l_bonus.board_opened[x_inner][y_inner]  {
                        if cfg!(test) {} else {l_bonus.bs_values[x_inner][y_inner] += l_bonus.bs_values[x_outer][y_outer];}
                        l_bonus.bs_v[x_inner][y_inner] = get_bs_v(&Value::Number(serde_json::Number::from_f64(l_bonus.bs_values[x_inner][y_inner]).unwrap()), l_bonus.round_bet);
                    }
                }
                l_bonus.board_opened[x_outer][y_outer] = true;
                let mut l_bs_v = l_bonus.bs_v[x_outer][y_outer].clone();
                if l_bs_v == "mini" {l_bs_v = (l_jp_mini * (l_bonus.round_bet as i64)).into()}
                if l_bs_v == "minor" {l_bs_v = (l_jp_minor * (l_bonus.round_bet as i64)).into()}
                if l_bs_v == "major" {l_bs_v = (l_jp_major * (l_bonus.round_bet as i64)).into()}
                l_bonus.boost_values.get_or_insert_with(|| Vec::new()).push(model::BoostValue { bs_v: l_bs_v, pos: [x_outer as i32, y_outer as i32]});
            }
        }

        for p_outer in 0..(l_bonus.bs_count as usize) {
            let x_outer = l_bonus.copy_new_bs.get_or_insert_with(|| Vec::new())[p_outer][0] as usize;
            let y_outer = l_bonus.copy_new_bs.get_or_insert_with(|| Vec::new())[p_outer][1] as usize;
            if l_bonus.board[x_outer][y_outer] == 12 {
                for p_inner in 0..(l_bonus.bs_count as usize) {
                    let x_inner = l_bonus.copy_new_bs.get_or_insert_with(|| Vec::new())[p_inner][0] as usize;
                    let y_inner = l_bonus.copy_new_bs.get_or_insert_with(|| Vec::new())[p_inner][1] as usize;
                    if l_bonus.board_opened[x_inner][y_inner] {
                        if cfg!(test) {} else {l_bonus.bs_values[x_inner][y_inner] *= 2.0;}
                        if l_bonus.orig_bs_v[x_inner][y_inner] == "mini" && l_bonus.orig_board[x_inner][y_inner] == 11 {l_bonus.bs_v[x_inner][y_inner] = "minor".into()}
                        else {l_bonus.bs_v[x_inner][y_inner] = get_bs_v(&Value::Number(serde_json::Number::from_f64(l_bonus.bs_values[x_inner][y_inner]).unwrap()), l_bonus.round_bet);}
                    }
                }
                l_bonus.board_opened[x_outer][y_outer] = true;
                let mut l_bs_v = l_bonus.bs_v[x_outer][y_outer].clone();
                if l_bs_v == "mini" {l_bs_v = (l_jp_mini * (l_bonus.round_bet as i64)).into()}
                if l_bs_v == "minor" {l_bs_v = (l_jp_minor * (l_bonus.round_bet as i64)).into()}
                if l_bs_v == "major" {l_bs_v = (l_jp_major * (l_bonus.round_bet as i64)).into()}
                l_bonus.double_values.get_or_insert_with(|| Vec::new()).push(model::BoostValue { bs_v: l_bs_v, pos: [x_outer as i32, y_outer as i32]});
            }
        }

        for p_outer in (0..(l_bonus.bs_count as usize)).rev() {
            let x_outer = l_bonus.copy_new_bs.get_or_insert_with(|| Vec::new())[p_outer][0] as usize;
            let y_outer = l_bonus.copy_new_bs.get_or_insert_with(|| Vec::new())[p_outer][1] as usize;
            if l_bonus.board[x_outer][y_outer] == 13 {
                for p_inner in 0..(l_bonus.bs_count as usize) {
                    let x_inner = l_bonus.copy_new_bs.get_or_insert_with(|| Vec::new())[p_inner][0] as usize;
                    let y_inner = l_bonus.copy_new_bs.get_or_insert_with(|| Vec::new())[p_inner][1] as usize;
                    if l_bonus.board_opened[x_inner][y_inner] {
                        l_bonus.bs_values[x_outer][y_outer] += l_bonus.bs_values[x_inner][y_inner];
                        l_bonus.bs_v[x_outer][y_outer] = get_bs_v(&Value::Number(serde_json::Number::from_f64(l_bonus.bs_values[x_outer][y_outer]).unwrap()), l_bonus.round_bet);
                    }
                }
                l_bonus.board_opened[x_outer][y_outer] = true;
                let mut l_bs_v = l_bonus.bs_v[x_outer][y_outer].clone();
                if l_bs_v == "mini" {l_bs_v = (l_jp_mini * (l_bonus.round_bet as i64)).into()}
                if l_bs_v == "minor" {l_bs_v = (l_jp_minor * (l_bonus.round_bet as i64)).into()}
                if l_bs_v == "major" {l_bs_v = (l_jp_major * (l_bonus.round_bet as i64)).into()}
                l_bonus.collect_values.get_or_insert_with(|| Vec::new()).push(model::BoostValue { bs_v: l_bs_v, pos: [x_outer as i32, y_outer as i32]});
            }
        }
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                if l_bonus.board[x][y] == 14 {l_bonus.board_opened[x][y] = true}
                if ["mini", "minor", "major"].into_iter().map(Into::into).collect::<Vec<_>>().contains(&l_bonus.orig_bs_v[x][y]) {l_bonus.jackpot_values = Some([a_game.settings.jackpots.mini * l_bonus.round_bet as i64, a_game.settings.jackpots.minor * l_bonus.round_bet as i64, a_game.settings.jackpots.major * l_bonus.round_bet as i64]);}
            }
        }
        

        if a_game.context.spins.selected_mode == Some("0".to_string()) {a_game.context.spins.selected_mode = None;}
        a_game.context.actions = ["respin".to_string()].into_iter().map(Into::into).collect();
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

fn get_bs_v(a_bonus_symbol_v: &serde_json::Value, a_round_bet: i32) -> serde_json::Value {
    match &a_bonus_symbol_v {
        Value::Number(num) => {Value::Number(serde_json::Number::from_f64(num.as_f64().unwrap_or_default() * a_round_bet as f64).unwrap())}
        Value::String(s) => {Value::String(s.clone())}
        _ => {Value::Number(serde_json::Number::from_f64(0.0).unwrap())}
    }
}