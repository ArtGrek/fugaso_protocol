use server::{bng, games_list};
use actix_web::{web, HttpResponse, body::to_bytes};
use serde_json; 
use serde_json::Value;
use serde_json::json;
use std::fs::File;
use std::io::BufReader;
use pretty_assertions::assert_eq;
use indicatif::ProgressBar;
use std::collections::HashMap;

use crate::bng::three_aztec_temples::reels::{Reels, BonusRespinItem, };
use crate::bng::three_aztec_temples::settings::{BOARD_HEIGHT, BOARD_WIDTH, MINI_STR, MINOR_STR, MAJOR_STR, MINI_VALUE, MINOR_VALUE, MAJOR_VALUE};
use crate::bng::three_aztec_temples::models::enums::MultiValueEnum;
use crate::bng::three_aztec_temples::models::mock::MockData;
use crate::bng::three_aztec_temples::models::server::{MultiValues};

#[actix_web::test]
async fn app() {
    let reels: Reels = serde_json::from_str(&fs::read_to_string("../data/three_aztec_temples/reels/reels.json").unwrap()).unwrap();
    let reels_buy1: Reels = serde_json::from_str(&fs::read_to_string("../data/three_aztec_temples/reels/reels_buy1.json").unwrap()).unwrap();
    let reels_buy2: Reels = serde_json::from_str(&fs::read_to_string("../data/three_aztec_temples/reels/reels_buy2.json").unwrap()).unwrap();
    let shared_game_list: games_list::LockedList<games_list::Games> = games_list::LockedList::<games_list::Games>::new();
    use std::fs;
    let data_dir = "../data/three_aztec_temples/tests";
    for entry_result in fs::read_dir(data_dir).unwrap() {
        let entry = entry_result.unwrap();
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            let mut log_path = path.clone();
            log_path.set_extension("log");
            println!("test: {:?}", path.file_name().unwrap());
            let file = File::open(path).unwrap();
            let mut reader = BufReader::new(file);
            let mut text = String::new();
            use std::io::Read;
            reader.read_to_string(&mut text).unwrap();
            text.pop();text.pop();
            let fixed_text = format!("[{}]", text);
            let json: Value = serde_json::from_str(&fixed_text).unwrap();
            let transactions = json.as_array().unwrap();
            let pb = ProgressBar::new((transactions.len()) as u64);
            let mut l_position = 0;
            for transaction in transactions {
                l_position += 1;
                pb.set_position(l_position);
                let request = transaction.get("in").unwrap().clone();
                let mut expected_response: Value = transaction.get("out").unwrap().clone();
                let mock_data = fill_mock_data(&expected_response, &reels, &reels_buy1, &reels_buy2);
                //println!("{mock_data:?}");
                let http_response: HttpResponse = bng::actions::execute(web::Bytes::from(serde_json::to_string(&request).unwrap()), web::Data::new(shared_game_list.clone()), true, &mock_data).await.map_err(|e|  e).expect("REASON");
                let body_bytes = to_bytes(http_response.into_body()).await.unwrap();
                let mut actual_response: Value = serde_json::from_slice(&body_bytes).unwrap();
                crutches(&mut actual_response, &mut expected_response);
                assert_eq!(actual_response, expected_response);
            }
            pb.finish();
        }
    }
}

fn fill_mock_data(expected_response: &Value, reels: &Reels, reels_buy1: &Reels, reels_buy2: &Reels, ) -> MockData {
    // start collect mock data
    let session_id = Some(expected_response.get("session_id").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or("mock".to_string()));
    let huid = Some(expected_response.get("user").and_then(|user| user.get("huid")).and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or("mock".to_string()));
    let roundnum = Some(expected_response.get("roundnum").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or("mock".to_string()));
    let mut board = Some(vec![vec![10000; BOARD_HEIGHT]; BOARD_WIDTH]);
    let mut spins_category = Some(10000);
    let mut spins_board_positions = Some(vec![10000; BOARD_WIDTH]);
    let mut spins_coins_appearances = Some(vec![10000; BOARD_WIDTH]);
    let mut spins_coins_values = Some(vec![vec![10000; BOARD_HEIGHT]; BOARD_WIDTH]);
    let mut spins_specials_appearances = Some(vec![10000; BOARD_WIDTH]);
    let mut spins_bonus_win = Some(10000);
    let mut spins_bac_coins_appearances = Some(vec![10000; BOARD_WIDTH]);
    let mut spins_bac_coins_values = Some(vec![vec![10000; BOARD_HEIGHT]; BOARD_WIDTH]);
    let mut bonus_category = Some(10000);
    let mut bonus_specials_init_values = Some(vec![vec![10000; BOARD_HEIGHT]; BOARD_WIDTH]);
    let mut bonus_specials_init_mults = Some(vec![vec![10000; BOARD_HEIGHT]; BOARD_WIDTH]);
    let mut bonus_coins_appearances = Some(vec![vec![10000; BOARD_HEIGHT]; BOARD_WIDTH]);
    let mut bonus_coins_values = Some(vec![vec![10000; BOARD_HEIGHT]; BOARD_WIDTH]);
    let mut bonus_specials_appearances = Some(vec![vec![10000; BOARD_HEIGHT]; BOARD_WIDTH]);
    let mut bonus_specials_values = Some(vec![vec![10000; BOARD_HEIGHT]; BOARD_WIDTH]);
    let mut bonus_specials_mults = Some(vec![vec![10000; BOARD_HEIGHT]; BOARD_WIDTH]);
    let mut bonus_mystery_appearances = Some(vec![vec![10000; BOARD_HEIGHT]; BOARD_WIDTH]);
    let mut bonus_mystery_ids = Some(vec![vec![10000; BOARD_HEIGHT]; BOARD_WIDTH]);
    let mut bonus_mystery_values = Some(vec![vec![10000; BOARD_HEIGHT]; BOARD_WIDTH]);
    let mut bonus_mystery_mults = Some(vec![vec![10000; BOARD_HEIGHT]; BOARD_WIDTH]);
    
    
    let selected_mode = Some(expected_response.get("context").and_then(|ctx| ctx.get("spins")).and_then(|spins| spins.get("selected_mode")).and_then(|v| v.as_str()).unwrap_or("0"));
    let current_reels = match selected_mode {
        Some("1") => {reels_buy1},
        Some("2") => {reels_buy2},
        _ => {reels},
    };
    let current_action = expected_response.get("context").and_then(|context| context.get("current")).and_then(|v| v.as_str()).map(|s| s).unwrap_or_default();
    if current_action == "spins" {
        let _spin_type = Some(expected_response.get("context").and_then(|ctx| ctx.get("spins")).and_then(|spins| spins.get("spin_type")).and_then(|v| v.as_i64()).unwrap_or(0) as u32);
        let spin_board = expected_response.get("context").and_then(|ctx| ctx.get("spins")).and_then(|spins| spins.get("board")).and_then(|b| serde_json::from_value::<Vec<Vec<i64>>>(b.clone()).ok());
        let spin_origin_board = expected_response.get("context").and_then(|ctx| ctx.get("spins")).and_then(|spins| spins.get("origin_board")).and_then(|b| serde_json::from_value::<Vec<Vec<i64>>>(b.clone()).ok());
        let spin_bs_values = expected_response.get("context").and_then(|ctx| ctx.get("spins")).and_then(|spins| spins.get("bs_values")).and_then(|v| serde_json::from_value::<Vec<Vec<f64>>>(v.clone()).ok());
        let spin_bac_win = expected_response.get("context").and_then(|ctx| ctx.get("spins")).and_then(|spins| spins.get("bac_win")).and_then(|v| v.as_bool()).unwrap_or(false);
        let spin_mechanic = expected_response.get("context").and_then(|ctx| ctx.get("spins")).and_then(|bonus| bonus.get("bonus_mechanic")).and_then(|v| serde_json::from_value::<Vec<i64>>(v.clone()).ok());
        let spin_mechanic_id = spin_mechanic.as_ref().map(|v| v.iter().map(|x| x.to_string()).collect::<String>()).unwrap_or("0".to_string());
        board = if let Some(orig_board) = &spin_origin_board {Some(orig_board.clone())} else {Some(spin_board.clone().map(|rows|rows.into_iter().map(|row| row.into_iter().map(|v| if vec![10, 11, 12, 13].contains(&v) { 0 } else { v }).collect()).collect()).unwrap_or(vec![vec![10000; BOARD_HEIGHT]; BOARD_WIDTH]))};
        spins_category = Some(0);
        spins_board_positions = None;
        // spins_coins_appearances
        spins_coins_appearances = if let Some(orig_board) = &spin_origin_board {
            spins_bac_coins_appearances = Some(spin_board.as_ref().map(|cols| {
                    let arr: Vec<u32> = cols.iter().enumerate().map(|(x, rows)| {
                        let mut chain = vec![];
                        for (y, &symbol) in rows.iter().enumerate() {if symbol == 10 && orig_board[x][y] != 10 {chain.push(y as i64)}}
                        current_reels.coins_appearance.iter().find_map(|(key, appearance)| {if appearance.pos == chain {key.parse::<u32>().ok().map(|k| k - 1)} else {None}}).unwrap_or(10000)
                    }).collect();
                    if arr.is_empty() {vec![10000; BOARD_WIDTH]} else {arr}
            }).unwrap_or(vec![10000; BOARD_WIDTH]));
            Some({
                let arr: Vec<u32> = orig_board.iter().map(|rows| {
                    let mut chain = vec![];
                    for (y, &symbol) in rows.iter().enumerate() {if symbol == 10 {chain.push(y as i64)}}
                    current_reels.coins_appearance.iter().find_map(|(key, appearance)| {if appearance.pos == chain {key.parse::<u32>().ok().map(|k| k - 1)} else {None}}).unwrap_or(10000)
                }).collect();
                if arr.is_empty() {vec![10000; BOARD_WIDTH]} else {arr}
            })
        } else {
            Some(spin_board.as_ref().map(|cols| {
                let arr: Vec<u32> = cols.iter().map(|rows| {
                    let mut chain = vec![];
                    let mut coin_exist = false;
                    for (y, &symbol) in rows.iter().enumerate() {if [11, 12, 13].contains(&symbol) || symbol == 10 {chain.push(y as i64); if symbol == 10 {coin_exist = true}} else {if !coin_exist {chain.clear()};}}
                    current_reels.coins_appearance.iter().find_map(|(key, appearance)| {if appearance.pos == chain && coin_exist {key.parse::<u32>().ok().map(|k| k - 1)} else {None}}).unwrap_or(10000)
                }).collect();
                if arr.is_empty() {vec![10000; BOARD_WIDTH]} else {arr}
            }).unwrap_or(vec![10000; BOARD_WIDTH]))
        };
        // spins_coins_values
        spins_coins_values = if let Some(orig_board) = &spin_origin_board {
            spins_bac_coins_values = Some(spin_board.as_ref().map(|bd| {
                bd.iter().enumerate().map(|(x, col)| {
                    col.iter().enumerate().map(|(y, &val)| {
                        if val == 10 && orig_board[x][y] != 10 {
                            let bs_v = spin_bs_values.as_ref().and_then(|rows| rows.get(x)).and_then(|row| row.get(y)).copied().unwrap_or(0.0);
                            current_reels.coin_value.iter().find_map(|(key, enum_val)| {
                                match enum_val {
                                    MultiValueEnum::Int(i) if (*i as f64) == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                    MultiValueEnum::Float(f) if *f == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                    MultiValueEnum::String(s) => match s.as_str() {
                                        MINI_STR if (MINI_VALUE as f64) == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                        MINOR_STR if (MINOR_VALUE as f64) == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                        MAJOR_STR if (MAJOR_VALUE as f64) == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                        _ => None,
                                    },
                                    _ => None
                                }
                            }).unwrap_or(10000)
                        } else {10000}
                    }).collect::<Vec<u32>>()
                }).collect::<Vec<Vec<u32>>>()
            }).unwrap_or(vec![vec![10000; BOARD_HEIGHT]; BOARD_WIDTH]));
            Some({
                orig_board.iter().enumerate().map(|(x, col)| {
                    col.iter().enumerate().map(|(y, &val)| {
                        if val == 10 {
                            let bs_v = spin_bs_values.as_ref().and_then(|rows| rows.get(x)).and_then(|row| row.get(y)).copied().unwrap_or(0.0);
                            current_reels.coin_value.iter().find_map(|(key, enum_val)| {
                                match enum_val {
                                    MultiValueEnum::Int(i) if (*i as f64) == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                    MultiValueEnum::Float(f) if *f == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                    MultiValueEnum::String(s) => match s.as_str() {
                                        MINI_STR if (MINI_VALUE as f64) == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                        MINOR_STR if (MINOR_VALUE as f64) == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                        MAJOR_STR if (MAJOR_VALUE as f64) == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                        _ => None,
                                    },
                                    _ => None
                                }
                            }).unwrap_or(10000)
                        } else {10000}
                    }).collect::<Vec<u32>>()
                }).collect::<Vec<Vec<u32>>>()
            })
        } else {
            Some(spin_board.as_ref().map(|bd| {
                bd.iter().enumerate().map(|(x, col)| {
                    col.iter().enumerate().map(|(y, &val)| {
                        if val == 10 {
                            let bs_v = spin_bs_values.as_ref().and_then(|rows| rows.get(x)).and_then(|row| row.get(y)).copied().unwrap_or(0.0);
                            current_reels.coin_value.iter().find_map(|(key, enum_val)| {
                                match enum_val {
                                    MultiValueEnum::Int(i) if (*i as f64) == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                    MultiValueEnum::Float(f) if *f == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                    MultiValueEnum::String(s) => match s.as_str() {
                                        MINI_STR if (MINI_VALUE as f64) == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                        MINOR_STR if (MINOR_VALUE as f64) == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                        MAJOR_STR if (MAJOR_VALUE as f64) == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                        _ => None,
                                    },
                                    _ => None
                                }
                            }).unwrap_or(10000)
                        } else {10000}
                    }).collect::<Vec<u32>>()
                }).collect::<Vec<Vec<u32>>>()
            }).unwrap_or(vec![vec![10000; BOARD_HEIGHT]; BOARD_WIDTH]))
        };
        // spins_specials_appearances
        spins_specials_appearances = Some(spin_board.as_ref().map(|bd| {
            bd.iter().map(|col_data| {
                current_reels.special_appearance.iter().find_map(|(key, app)| {
                    let target_id = app.id;
                    let matched = app.pos.iter().any(|&row| {matches!(col_data.get(row as usize), Some(val) if *val == target_id)});
                    if matched {key.parse::<u32>().ok().map(|k| k - 1)} else {None}
                }).unwrap_or(10000)
            }).collect()
        }).unwrap_or(vec![10000; BOARD_WIDTH]));
        spins_bonus_win = if !spin_bac_win {Some(10000)} else {Some(current_reels.bonus_win.get(&spin_mechanic_id).map(|v| v - 1).unwrap_or(10000))};
    } else if current_action == "bonus" {
        let bonus_game_type = expected_response.get("context").and_then(|ctx| ctx.get("bonus")).and_then(|bonus| bonus.get("bonus_game_type")).and_then(|v| v.as_i64()).unwrap_or(1);
        let bonus_board = expected_response.get("context").and_then(|ctx| ctx.get("bonus")).and_then(|bonus| bonus.get("board")).and_then(|b| serde_json::from_value::<Vec<Vec<i64>>>(b.clone()).ok());
        let bonus_origin_board = expected_response.get("context").and_then(|ctx| ctx.get("bonus")).and_then(|bonus| bonus.get("origin_board")).and_then(|b| serde_json::from_value::<Vec<Vec<i64>>>(b.clone()).ok());
        let bonus_bs_values = expected_response.get("context").and_then(|ctx| ctx.get("bonus")).and_then(|bonus| bonus.get("bs_values")).and_then(|v| serde_json::from_value::<Vec<Vec<f64>>>(v.clone()).ok());
        let bonus_multi_values = expected_response.get("context").and_then(|ctx| ctx.get("bonus")).and_then(|bonus| bonus.get("multi_values")).and_then(|v| serde_json::from_value::<Vec<MultiValues>>(v.clone()).ok());
        let bonus_mechanic = expected_response.get("context").and_then(|ctx| ctx.get("bonus")).and_then(|bonus| bonus.get("bonus_mechanic")).and_then(|v| serde_json::from_value::<Vec<i64>>(v.clone()).ok());
        let mechanic_id = bonus_mechanic.as_ref().map(|v| v.iter().map(|x| x.to_string()).collect::<String>()).unwrap_or("0".to_string());
        // get bonus category with data
        let mut keys_values: Vec<(u32, &HashMap<String, HashMap<String, BonusRespinItem>>)> = current_reels.bonus_respin.iter().filter_map(|(k, v)| k.parse::<u32>().ok().map(|key| (key, v))).collect();
        keys_values.sort_by_key(|(k, _)| *k);
        let (bonus_category_id, bonus_category_data) = keys_values.get((bonus_game_type - 1) as usize).map(|(k, v)| (Some(*k - 1), Some(*v))).unwrap_or((Some(1), None));
        bonus_category = bonus_category_id;
        let last_action = expected_response.get("context").and_then(|context| context.get("last_action")).and_then(|v| v.as_str()).map(|s| s).unwrap_or_default();
        if last_action == "bonus_init" {
            // decrease bonus_bs_values to the init state
            let mut init_bs_values = bonus_bs_values.clone();
            if let Some(temp_board) = bonus_board.as_ref() {
                if let Some(bs_values) = init_bs_values.as_mut() {
                    let mut board_is_executed = vec![vec![true; BOARD_HEIGHT]; BOARD_WIDTH];
                    for col_num_current in (0..BOARD_WIDTH).rev() {
                        for row_num_current in (0..BOARD_HEIGHT).rev() {
                            match temp_board[col_num_current][row_num_current] {
                                11 => {
                                    board_is_executed[col_num_current][row_num_current] = false;
                                    for col_num_target in 0..BOARD_WIDTH {
                                        for row_num_target in 0..BOARD_HEIGHT {
                                            if board_is_executed[col_num_target][row_num_target] && bs_values[col_num_target][row_num_target] > 0.0 {
                                                bs_values[col_num_target][row_num_target] = bs_values[col_num_target][row_num_target].clone() - bs_values[col_num_current][row_num_current].clone();
                                            }
                                        }
                                    }
                                }
                                12 => {bs_values[col_num_current][row_num_current] = 0.0}
                                13 => {
                                    board_is_executed[col_num_current][row_num_current] = false;
                                    let mult = bonus_multi_values.as_ref().and_then(|arr| arr.iter().find_map(|item| {if item.pos[0] == col_num_current as i64 && item.pos[1] == row_num_current as i64 {Some(item.mult_value)} else {None}})).unwrap_or(0);
                                    for col_num_target in (0..BOARD_WIDTH).rev() {
                                        for row_num_target in (0..BOARD_HEIGHT).rev() {
                                            if board_is_executed[col_num_target][row_num_target] && bs_values[col_num_target][row_num_target] > 0.0 {
                                                bs_values[col_num_target][row_num_target] = bs_values[col_num_target][row_num_target].clone() / mult as f64;
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }  
                        }
                    }
                }
            } 
            // bonus_specials_init_values
            bonus_specials_init_values = Some(bonus_board.as_ref().map(|bd| {
                bd.iter().enumerate().map(|(x, col)| {
                    col.iter().enumerate().map(|(y, &val)| {
                        if vec![11, 12, 13].contains(&val) {
                            let bs_v = init_bs_values.as_ref().and_then(|rows| rows.get(x)).and_then(|row| row.get(y)).copied().unwrap_or(0.0);
                            let values = current_reels.bonus_init.get(&mechanic_id).and_then(|specials| specials.get(&val.to_string()).cloned()).unwrap_or_default().value;
                            values.iter().find_map(|(key, enum_val)| {
                                match enum_val {
                                    MultiValueEnum::Int(i) if (*i as f64) == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                    MultiValueEnum::Float(f) if *f == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                    MultiValueEnum::String(s) => match s.as_str() {
                                        MINI_STR if (MINI_VALUE as f64) == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                        MINOR_STR if (MINOR_VALUE as f64) == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                        MAJOR_STR if (MAJOR_VALUE as f64) == bs_v => key.parse::<u32>().ok().map(|k| k - 1),
                                        _ => None,
                                    },
                                    _ => None
                                }
                            }).unwrap_or(10000)
                        } else {10000}
                    }).collect::<Vec<u32>>()
                }).collect::<Vec<Vec<u32>>>()
            }).unwrap_or(vec![vec![10000; BOARD_HEIGHT]; BOARD_WIDTH]));
            // bonus_specials_init_mults
            bonus_specials_init_mults = Some(bonus_board.as_ref().map(|bd| {
                bd.iter().enumerate().map(|(x, col)| {
                    col.iter().enumerate().map(|(y, &val)| {
                        if val == 13 {
                            let mult = bonus_multi_values.as_ref().and_then(|arr| arr.iter().find_map(|item| {
                                if item.pos[0] == x as i64 && item.pos[1] == y as i64 {Some(item.mult_value)} else {None}
                            })).unwrap_or(0);
                            let mults = current_reels.bonus_init.get(&mechanic_id).and_then(|specials| specials.get(&val.to_string()).cloned()).unwrap_or_default().mult;
                            if let Some(mults_map) = mults {
                                mults_map.iter().find_map(|(key, &i)| {
                                    if i == mult {key.parse::<u32>().ok().map(|k| k - 1)} else {None}
                                }).unwrap_or(10000)
                            } else {10000}
                        } else {10000}
                    }).collect::<Vec<u32>>()
                }).collect::<Vec<Vec<u32>>>()
            }).unwrap_or(vec![vec![10000; BOARD_HEIGHT]; BOARD_WIDTH]));
        } else if last_action == "respin" {
            let mut bonus_new_bs = expected_response.get("context").and_then(|ctx| ctx.get("bonus")).and_then(|bonus| bonus.get("new_bs")).and_then(|b| serde_json::from_value::<Vec<Vec<i64>>>(b.clone()).ok());
            let bonus_bs_count = expected_response.get("context").and_then(|ctx| ctx.get("bonus")).and_then(|bonus| bonus.get("bs_count")).and_then(|v| v.as_i64()).unwrap_or(6);
            // decrease bonus_bs_values to the init state
            let mut init_bs_values = bonus_bs_values.clone();
            if let Some(temp_new_bs) = bonus_new_bs.as_mut() {
                if let Some(temp_board) = bonus_board.as_ref() {
                    if let Some(bs_values) = init_bs_values.as_mut() {
                        let mut board_is_executed = vec![vec![true; BOARD_HEIGHT]; BOARD_WIDTH];
                        temp_new_bs.sort_by(|a, b| b[0].cmp(&a[0]).then(b[1].cmp(&a[1])));
                        temp_new_bs.iter().for_each(|coords| {
                            let col_num_current = coords[0] as usize;
                            let row_num_current = coords[1] as usize;
                            match temp_board[col_num_current][row_num_current] {
                                    11 => {
                                        board_is_executed[col_num_current][row_num_current] = false;
                                        for col_num_target in 0..BOARD_WIDTH {
                                            for row_num_target in 0..BOARD_HEIGHT {
                                                if board_is_executed[col_num_target][row_num_target] && bs_values[col_num_target][row_num_target] > 0.0 && temp_new_bs.iter().any(|coords| {*coords == vec![col_num_target as i64, row_num_target as i64]}) {
                                                    bs_values[col_num_target][row_num_target] = bs_values[col_num_target][row_num_target].clone() - bs_values[col_num_current][row_num_current].clone();
                                                }
                                            }
                                        }
                                    }
                                    12 => {bs_values[col_num_current][row_num_current] = 0.0}
                                    13 => {
                                        board_is_executed[col_num_current][row_num_current] = false;
                                        let mult = bonus_multi_values.as_ref().and_then(|arr| arr.iter().find_map(|item| {if item.pos[0] == col_num_current as i64 && item.pos[1] == row_num_current as i64 {Some(item.mult_value)} else {None}})).unwrap_or(0);
                                        for col_num_target in (0..BOARD_WIDTH).rev() {
                                            for row_num_target in (0..BOARD_HEIGHT).rev() {
                                                if board_is_executed[col_num_target][row_num_target] && bs_values[col_num_target][row_num_target] > 0.0 && temp_new_bs.iter().any(|coords| {*coords == vec![col_num_target as i64, row_num_target as i64]}) {
                                                    bs_values[col_num_target][row_num_target] = bs_values[col_num_target][row_num_target].clone() / mult as f64;
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                } 
                        });
                    }
                }
            }
            let bonus_respin = bonus_category_data.and_then(|map1| map1.get(&mechanic_id)).and_then(|map2| map2.get(&(bonus_bs_count-1).to_string()));
            if let Some(respin) = bonus_respin {
                let temp_board = if let Some(orig_board) = &bonus_origin_board {orig_board.clone()} else {bonus_board.clone().unwrap_or_default()};
                if let Some(temp_bs_values) = &init_bs_values {
                    bonus_new_bs.as_ref().map(|coords| {coords.iter().for_each(|pos| {
                        let x = pos[0] as usize;
                        let y = pos[1] as usize;
                        match temp_board[x][y] {
                            10 => {
                                bonus_coins_appearances.as_mut().map(|appearances| {
                                    appearances[x][y] = respin.coins.iter().find_map(|(key, data)| {
                                        if data.id == Some(temp_board[x][y]) {
                                            bonus_coins_values.as_mut().map(|values| {
                                                values[x][y] = data.value.iter().find_map(|(key, enum_val)| {
                                                    match enum_val {
                                                        MultiValueEnum::Int(i) if (*i as f64) == temp_bs_values[x][y] => key.parse::<u32>().ok().map(|k| k - 1),
                                                        MultiValueEnum::Float(f) if *f == temp_bs_values[x][y] => key.parse::<u32>().ok().map(|k| k - 1),
                                                        MultiValueEnum::String(s) => match s.as_str() {
                                                            MINI_STR if (MINI_VALUE as f64) == temp_bs_values[x][y] => key.parse::<u32>().ok().map(|k| k - 1),
                                                            MINOR_STR if (MINOR_VALUE as f64) == temp_bs_values[x][y] => key.parse::<u32>().ok().map(|k| k - 1),
                                                            MAJOR_STR if (MAJOR_VALUE as f64) == temp_bs_values[x][y] => key.parse::<u32>().ok().map(|k| k - 1),
                                                            _ => None,
                                                        },
                                                        _ => None
                                                    }
                                                }).unwrap_or(10000);
                                            });
                                            key.parse::<u32>().ok().map(|k| k - 1)
                                        } else {None}
                                    }).unwrap_or(10000)
                                });
                            }
                            11 | 12 | 13 => {
                                bonus_specials_appearances.as_mut().map(|appearances| {
                                    appearances[x][y] = respin.specials.iter().find_map(|(key, data)| {
                                        if data.id == Some(temp_board[x][y]) {
                                            bonus_specials_values.as_mut().map(|values| {
                                                values[x][y] = data.value.iter().find_map(|(key, enum_val)| {
                                                    match enum_val {
                                                        MultiValueEnum::Int(i) if (*i as f64) == temp_bs_values[x][y] => key.parse::<u32>().ok().map(|k| k - 1),
                                                        MultiValueEnum::Float(f) if *f == temp_bs_values[x][y] => key.parse::<u32>().ok().map(|k| k - 1),
                                                        MultiValueEnum::String(s) => match s.as_str() {
                                                            MINI_STR if (MINI_VALUE as f64) == temp_bs_values[x][y] => key.parse::<u32>().ok().map(|k| k - 1),
                                                            MINOR_STR if (MINOR_VALUE as f64) == temp_bs_values[x][y] => key.parse::<u32>().ok().map(|k| k - 1),
                                                            MAJOR_STR if (MAJOR_VALUE as f64) == temp_bs_values[x][y] => key.parse::<u32>().ok().map(|k| k - 1),
                                                            _ => None,
                                                        },
                                                        _ => None
                                                    }
                                                }).unwrap_or(10000)
                                            });
                                            if temp_board[x][y] == 13 {
                                                let mult = bonus_multi_values.as_ref().and_then(|arr| arr.iter().find_map(|item| {
                                                    if item.pos[0] == x as i64 && item.pos[1] == y as i64 {Some(item.mult_value)} else {None}
                                                })).unwrap_or(0);
                                                bonus_specials_mults.as_mut().map(|mults| {
                                                    if let Some(mults_map) = &data.mult {
                                                        mults[x][y] = mults_map.iter().find_map(|(key, &i)| {
                                                            if i == mult {key.parse::<u32>().ok().map(|k| k - 1)} else {None}
                                                        }).unwrap_or(10000);
                                                    };
                                                });
                                            };
                                            key.parse::<u32>().ok().map(|k| k - 1)
                                        } else {None}
                                    }).unwrap_or(10000)
                                });
                            }
                            14 => {
                                bonus_mystery_appearances.as_mut().map(|appearances| {
                                    appearances[x][y] = respin.mystery.iter().find_map(|(key, data)| {
                                        if data.id == temp_board[x][y] {
                                            bonus_mystery_ids.as_mut().map(|ids| {
                                                ids[x][y] = data.symbols.iter().find_map(|(key, enum_id)| {
                                                    if let (Some(id), Some(board)) = (enum_id.id, bonus_board.as_ref()) {
                                                        if let Some(row) = board.get(x) {
                                                            if let Some(cell) = row.get(y) {
                                                                if *cell == id {
                                                                    bonus_mystery_values.as_mut().map(|values| {
                                                                        values[x][y] = enum_id.value.iter().find_map(|(key, enum_val)| {
                                                                            match enum_val {
                                                                                MultiValueEnum::Int(i) if (*i as f64) == temp_bs_values[x][y] => key.parse::<u32>().ok().map(|k| k - 1),
                                                                                MultiValueEnum::Float(f) if *f == temp_bs_values[x][y] => key.parse::<u32>().ok().map(|k| k - 1),
                                                                                MultiValueEnum::String(s) => match s.as_str() {
                                                                                    MINI_STR if (MINI_VALUE as f64) == temp_bs_values[x][y] => key.parse::<u32>().ok().map(|k| k - 1),
                                                                                    MINOR_STR if (MINOR_VALUE as f64) == temp_bs_values[x][y] => key.parse::<u32>().ok().map(|k| k - 1),
                                                                                    MAJOR_STR if (MAJOR_VALUE as f64) == temp_bs_values[x][y] => key.parse::<u32>().ok().map(|k| k - 1),
                                                                                    _ => None,
                                                                                },
                                                                                _ => None
                                                                            }
                                                                        }).unwrap_or(10000)
                                                                    });
                                                                    if *cell == 13 {
                                                                        let mult = bonus_multi_values.as_ref().and_then(|arr| arr.iter().find_map(|item| {
                                                                            if item.pos[0] == x as i64 && item.pos[1] == y as i64 {Some(item.mult_value)} else {None}
                                                                        })).unwrap_or(0);
                                                                        bonus_mystery_mults.as_mut().map(|mults| {
                                                                            if let Some(mults_map) = &enum_id.mult {
                                                                                mults[x][y] = mults_map.iter().find_map(|(key, &i)| {
                                                                                    if i == mult {key.parse::<u32>().ok().map(|k| k - 1)} else {None}
                                                                                }).unwrap_or(10000);
                                                                            };
                                                                        });
                                                                    };
                                                                    key.parse::<u32>().ok().map(|k| k - 1)
                                                                } else {None}
                                                            } else {None}
                                                        } else {None}
                                                    } else {None}
                                                }).unwrap_or(10000)
                                            });
                                            key.parse::<u32>().ok().map(|k| k - 1)
                                        } else {None}
                                    }).unwrap_or(10000)
                                });
                            }
                            _ => {}
                        }
                    })});
                };
            };
        }
    }

    MockData {
        session_id,
        huid,
        roundnum,
        board,
        spins_category,
        spins_board_positions,
        spins_coins_appearances,
        spins_coins_values,
        spins_specials_appearances,
        spins_bonus_win,
        spins_bac_coins_appearances,
        spins_bac_coins_values,
        bonus_category,
        bonus_specials_init_values,
        bonus_specials_init_mults,
        bonus_coins_appearances,
        bonus_coins_values,
        bonus_specials_appearances,
        bonus_specials_values,
        bonus_specials_mults,
        bonus_mystery_appearances,
        bonus_mystery_ids,
        bonus_mystery_values,
        bonus_mystery_mults,
    }
}

fn crutches(actual_response: &mut Value, expected_response: &mut Value, ) {
    let keys = &[];
    if !keys.is_empty() {remove_keys(actual_response, keys); remove_keys(expected_response, keys);}
    sort_array(actual_response.get_mut("context").and_then(|context| context.get_mut("spins")).and_then(|bonus| bonus.get_mut("bac_pos")).unwrap_or(&mut json!([])).as_array_mut());
    sort_array(expected_response.get_mut("context").and_then(|context| context.get_mut("spins")).and_then(|bonus| bonus.get_mut("bac_pos")).unwrap_or(&mut json!([])).as_array_mut());
    sort_array(actual_response.get_mut("context").and_then(|context| context.get_mut("bonus")).and_then(|bonus| bonus.get_mut("new_bs")).unwrap_or(&mut json!([])).as_array_mut());
    sort_array(expected_response.get_mut("context").and_then(|context| context.get_mut("bonus")).and_then(|bonus| bonus.get_mut("new_bs")).unwrap_or(&mut json!([])).as_array_mut());
    sort_special_array(actual_response.get_mut("context").and_then(|context| context.get_mut("bonus")).and_then(|bonus| bonus.get_mut("mystery_values")).unwrap_or(&mut json!([])).as_array_mut());
    sort_special_array(expected_response.get_mut("context").and_then(|context| context.get_mut("bonus")).and_then(|bonus| bonus.get_mut("mystery_values")).unwrap_or(&mut json!([])).as_array_mut());
    sort_special_array(actual_response.get_mut("context").and_then(|context| context.get_mut("bonus")).and_then(|bonus| bonus.get_mut("boost_values")).unwrap_or(&mut json!([])).as_array_mut());
    sort_special_array(expected_response.get_mut("context").and_then(|context| context.get_mut("bonus")).and_then(|bonus| bonus.get_mut("boost_values")).unwrap_or(&mut json!([])).as_array_mut());
    sort_special_array(actual_response.get_mut("context").and_then(|context| context.get_mut("bonus")).and_then(|bonus| bonus.get_mut("collect_values")).unwrap_or(&mut json!([])).as_array_mut());
    sort_special_array(expected_response.get_mut("context").and_then(|context| context.get_mut("bonus")).and_then(|bonus| bonus.get_mut("collect_values")).unwrap_or(&mut json!([])).as_array_mut());
    sort_special_array(actual_response.get_mut("context").and_then(|context| context.get_mut("bonus")).and_then(|bonus| bonus.get_mut("multi_values")).unwrap_or(&mut json!([])).as_array_mut());
    sort_special_array(expected_response.get_mut("context").and_then(|context| context.get_mut("bonus")).and_then(|bonus| bonus.get_mut("multi_values")).unwrap_or(&mut json!([])).as_array_mut());
}

fn remove_keys(value: &mut Value, keys: &[&str]) {
    if let Value::Object(map) = value {
        for &key in keys {map.remove(key);}
        for v in map.values_mut() {remove_keys(v, keys);}
    } else if let Value::Array(arr) = value {
        for v in arr.iter_mut() {remove_keys(v, keys);}
    }
}

fn sort_array(array: Option<&mut Vec<Value>>) {
    if let Some(sorting_array) = array {
        sorting_array.sort_by(|a, b| {
            let a0 = a.get(0).and_then(Value::as_i64).unwrap_or(0);
            let b0 = b.get(0).and_then(Value::as_i64).unwrap_or(0);
            let a1 = a.get(1).and_then(Value::as_i64).unwrap_or(0);
            let b1 = b.get(1).and_then(Value::as_i64).unwrap_or(0);
            a0.cmp(&b0).then(a1.cmp(&b1))
        })
    }
}

fn sort_special_array(array: Option<&mut Vec<Value>>) {
    if let Some(sorting_array) = array {
        sorting_array.sort_by(|a, b| {
            let a_pos = a.get("pos").and_then(Value::as_array).unwrap();
            let b_pos = b.get("pos").and_then(Value::as_array).unwrap();
            let a0 = a_pos.get(0).and_then(Value::as_i64).unwrap_or(0);
            let b0 = b_pos.get(0).and_then(Value::as_i64).unwrap_or(0);
            let a1 = a_pos.get(1).and_then(Value::as_i64).unwrap_or(0);
            let b1 = b_pos.get(1).and_then(Value::as_i64).unwrap_or(0);
            a0.cmp(&b0).then(a1.cmp(&b1))
        })
    }
}