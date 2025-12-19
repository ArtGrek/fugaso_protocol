//use std::ops::Add;
use std::{collections::BTreeMap, fs::File, io::BufReader, sync::Arc};

use fugaso_core::protocol::PlayerRequest;
use fugaso_data::{fugaso_action::ActionKind, fugaso_round::RoundDetail};
use fugaso_math::protocol::TrinityPowerKind;
use fugaso_math::protocol;
use fugaso_math::protocol::{id, FreeGame, Gain, GameData, GameResult, Promo, ReSpinInfo, SpinData, TrinityPowerLinkInfo};
use integration::FuGaSoTuple;
use num_traits::ToPrimitive;
use serde::Serialize;
use serde::Deserialize;
//use std::iter::Peekable;

mod integration;
use serde_json::Value;
use china_festival::game::models;

const GAME_SOURCE_NAME: &str = "china_festival";
const GAME_FUGASO_FOLDER: &str = "china_festival";

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct TupleGame {
    #[serde(rename = "in")]
    request: Value,
    #[serde(rename = "out")]
    response: Value,
}

fn parse_list_game(p: &str) -> Vec<TupleGame> {
    let file = File::open(format!("packets/{GAME_SOURCE_NAME}/{p}")).unwrap();
    let reader = BufReader::new(file);
    let response = serde_json::from_reader(reader).expect(&format!("error read {p}!"));
    response
}

#[derive(Debug, Serialize)]
pub struct Conf {
    pub lines: Vec<String>,
    pub wins: BTreeMap<char, BTreeMap<usize, i32>>,
}

#[tokio::test]
#[allow(unused)]
async fn test_config() {
    let mut json_str: Value = Default::default();
    let file = File::open("packets/china_festival/00-no_win.json").unwrap();
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).unwrap();
    let transactions = json.as_array().unwrap();
    for transaction in transactions {
        if transaction.get("out").and_then(|context| context.get("command")).and_then(|v| v.as_str()) == Some("start") {
            json_str = transaction.get("out").unwrap().clone();
            break;
        }
    }

    let start: models::network::response::start::Start = serde_json::from_value(json_str).expect("error start!");
    /*let math = 
        ThreeAfricanDrumsMath::new(MockThreeAfricanDrumsRand::new()).expect("error create math!");
    let start = math.start().expect("error start!");*/

    let symbols = vec!['^', '-', '_', 'V'];
    let cfg = match start {
        models::network::response::start::Start { settings, .. } => {
        //Response::Start(v) => {
            //v.settings.paylines
            let wins: BTreeMap<char, BTreeMap<usize, i32>> =
                settings.clone().unwrap_or_default()
                    .paytable
                    .iter()
                    .fold(BTreeMap::new(), |mut acc, v| {
                        let number = v.0.parse::<u32>().expect("error parse number!");
                        let symbol = char::from_u32(number + '@' as u32).expect("error symbol");
                        if let Some(_vec) = acc.get_mut(&symbol) {
                            panic!("error symbol already in map!")
                        } else {
                            acc.insert(
                                symbol,
                                v.1.iter()
                                    .map(|p| (p.occurrences as usize, p.multiplier))
                                    .collect(),
                            );
                        }
                        acc
                    });
            let lines = settings.clone().unwrap_or_default()
                .paylines
                .iter()
                .map(|p| {
                    p.iter()
                        .map(|v| v)
                        .map(|v| symbols[*v as usize])
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join("")
                })
                .collect::<Vec<_>>();
            Conf { wins, lines }
        }
        _ => panic!("error config create!"),
    };
    println!("{}", serde_json::to_string(&cfg).expect("error cfg json"));
}



#[test]
//#[ignore]
#[allow(unused)]
fn test_convert() {
   /*  convert("00-no_win.json");
    convert("01-win.json");
    convert("03-jackpots.json");
    convert("10-boost.json");
    convert("11-boost_after_double.json");
    convert("12-boost_after_collect.json");
    convert("14-boost_after_double_after_collect.json");
    convert("15-boost_after_collect_after_double.json");
    convert("16-boost_jackpot.json");
    convert("17-boost_with_mystery_boost.json");*/
    convert("171-boost_with_mystery_boost.json");
    /*convert("18-boost_with_mystery_double.json");
    convert("19-boost_with_mystery_collect.json");

    convert("20-double.json");
    convert("21-double_after_boost.json");
    convert("22-double_after_collect.json");
    convert("24-double_after_collect_after_boost.json");
    convert("25-double_after_boost_after_collect.json");
    convert("26-double_jackpot.json");
    

    convert("27-double_with_mystery_double.json");
    convert("28-double_with_mystery_boost.json");
    convert("29-double_with_mystery_collect.json");
    
    convert("30-collect.json");
    convert("31-collect_after_double.json");
    convert("32-collect_after_boost.json");
    convert("34-collect_after_double_after_boost.json");
    convert("35-collect_after_boost_after_double.json");
    convert("36-collect_jackpot.json");
    convert("37-collect_with_mystery_collect.json");
    convert("38-collect_with_mystery_boost.json");
    convert("39-collect_with_mystery_double.json");

    
    convert("40-boost_and_double.json");
    convert("41-boost_and_double_after_collect.json");
    convert("42-double_and_collect.json");
    convert("43-double_and_collect_after_boost.json");
    convert("44-collect_and_boost.json");
    convert("45-collect_and_boost_after_double.json");
    convert("46-boost_and_double_and_collect.json");*/
}

fn convert_board(board: &Vec<Vec<i32>>) -> Vec<Vec<char>> {
    board
        .iter()
        .map(|reel| {
            reel.iter()
                .map(|v| {
                    let symbol = char::from_u32(*v as u32 + '@' as u32).expect("error symbol");
                    symbol
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn convert_win_lines(win_lines: &Vec<models::network::response::start::Winline>) -> Vec<Gain> {
    let mut gains = win_lines
        .iter()
        .map(|l| {
            let symbol = char::from_u32(l.symbol as u32 + '@' as u32).expect("error symbol");
            Gain {
                symbol,
                count: l.occurrences as usize,
                amount: l.amount,
                line_num: (l.line as usize) - 1,
                multi: 1,
                ..Default::default()
            }
        })
        .collect::<Vec<_>>();
    gains.sort_by_key(|w| w.line_num);
    gains
}

fn convert_bs_values(round_bet: i64, bs_values: &Vec<Vec<f64>>) -> Vec<Gain> {
    let symbol = china_festival::SYMBOL_COIN;
    let sum = bs_values
        .iter()
        .flat_map(|v| v)
        .map(|v| (*v * round_bet as f64).round() as i64)
        .sum::<i64>();
    vec![Gain {
        symbol,
        count: bs_values
            .iter()
            .flat_map(|b_v| b_v.iter().filter(|v| **v > 0.0))
            .count(),
        amount: sum,
        multi: 1,
        ..Default::default()
    }]
}

fn _convert_holds(grid: &Vec<Vec<char>>, bs_values: &Vec<Vec<f64>>, bet_factor: i32) -> Vec<i32> {
    let mut holds = grid.iter().flat_map(|v| v).map(|_| 0).collect::<Vec<_>>();
    for reel in 0..bs_values.len() {
        for position in 0..bs_values[reel].len() {
            let i = reel * grid[reel].len() + position;
            let d = (bs_values[reel][position] * bet_factor as f64).round() as i32;
            holds[i] = d.to_i32().expect("error convert!");
        }
    }
    holds
}

fn convert_bg_type(bonus_mechanic: &Vec<i32>) -> Option<TrinityPowerKind> {
    match (bonus_mechanic.iter().sum(), bonus_mechanic.len()) {
        (1, 1) => {Some(TrinityPowerKind::Add)}
        (2, 1) => {Some(TrinityPowerKind::Twice)}
        (3, 1) => {Some(TrinityPowerKind::Cluster)}
        (3, 2) => {Some(TrinityPowerKind::AddTwice)}
        (5, 2) => {Some(TrinityPowerKind::TwiceCluster)}
        (4, 2) => {Some(TrinityPowerKind::AddCluster)}
        (6, 3) => {Some(TrinityPowerKind::AddTwiceCluster)}
        _ => {None}
    }
}

fn covert_bs_v(value: &Value) -> i32 {
    match &value {
        Value::Number(num) => {num.as_f64().unwrap_or_default() as i32}
        Value::String(s) => {
            match s.as_str() {
                "mini" => {300}
                "minor" => {600}
                "major" => {2000}
                _ => {0}
            }
        }
        _ => {0}
    }
}


#[allow(unused)]
fn convert(name: &str) {
    let mut l_bet_factor: i32 = 20;
    let list = parse_list_game(name);
    let mut prev_lift: Vec<Vec<i32>> = vec![];
    let mut prev_lift_2: Vec<Vec<i32>> = vec![];

    let mut iter = list.into_iter().peekable();
    let mut results = Vec::new();
    let bet_counters = [1, 100, 300];
    while let Some(t) = iter.next() {
        let converted_tr: Option<FuGaSoTuple<TrinityPowerLinkInfo, ReSpinInfo>> = match serde_json::from_value(t.response).inspect_err(|e|{
            eprintln!("{e}");
        }).unwrap_or(None) {
            Some(models::network::response::start::Start { command, context, modes, request_id, session_id, settings, status, user }) => {
                if command == "play" {
                    if context.last_action == "bonus_init" || context.last_action == "bonus_spins_stop" {
                        None
                    } else {
                        let (bet_per_line, lines) = (context.spins.bet_per_line, context.spins.lines);
                        let selected_mode = context.spins.selected_mode.as_ref().map(|v|v.parse::<usize>().expect("error selected mode!"));
                        let bet_counter = bet_counters[selected_mode.unwrap_or(0)];
                        let request = fugaso_math::math::Request {bet: bet_per_line, denom: l_bet_factor, line: lines as usize, bet_counter, ..Default::default()};
                        let bonus = context.bonus.as_ref();
                        let (grid, holds, lift, reelset_number, lift_new) =
                        if context.last_action == "buy_spin" || context.last_action == "spin" {
                            let spins = &context.spins;
                            let grid = convert_board(&spins.board.iter().map(|row| row.to_vec()).collect());
                            let mut holds = vec![0];
                            (grid, holds, vec![], Some(0), vec![])
                        } else if context.last_action == "respin" {
                            let bonus_on = bonus.expect("bonus is none!");
                            let grid = convert_board(&bonus_on.board.iter().map(|row| row.to_vec()).collect());
                            let holds = vec![0];
                            let (lift, lift_new) = (vec![], vec![]);
                            (grid, holds, lift, Some(0), lift_new,)
                        } else {panic!("error grid - {}!", context.last_action);};
                        prev_lift = lift.clone();

                        let (next_act, total, gains, link) = 
                        if context.actions == vec!["spin", "buy_spin"] && context.spins.total_win.unwrap_or(0) > 0 {
                            let spins = context.spins;
                            let gains = convert_win_lines(&spins.winlines.unwrap_or(vec![]));
                            (ActionKind::COLLECT, spins.total_win.unwrap_or(0), gains, TrinityPowerLinkInfo {total: spins.total_win.unwrap_or(0), lift, lift_new, ..Default::default()},)
                        } else if context.actions == vec!["bonus_init"] {
                            let (gains, free, total, rounds_left, kind, adds, twices, clusters, rands, mults0, mults1, grid0, grid1) = {
                                let spins = context.spins;
                                let bet_per_line = spins.bet_per_line;
                                let gains = convert_win_lines(&spins.winlines.unwrap_or(vec![]));
                                let rounds_left = china_festival::BONUS_COUNT;
                                let kind = convert_bg_type(&spins.bonus_mechanic.unwrap_or(vec![]));
                                let next_packet = serde_json::from_value::<models::network::response::start::Start>(iter.peek().unwrap().response.clone()).unwrap(); 
                                let adds: Vec<protocol::ChangeItem> = if let Some(boost_values) = next_packet.context.bonus.clone().unwrap().boost_values.clone()
                                {
                                    boost_values.iter().map(|boost_value| {
                                        let v = covert_bs_v(&boost_value.bs_v) as i32;
                                        let x = boost_value.pos[0] as usize;
                                        let y = boost_value.pos[1] as usize;
                                        protocol::ChangeItem { p: (x, y), v: v }
                                    }).collect::<Vec<protocol::ChangeItem>>()
                                } else {vec![]};
                                let twices: Vec<protocol::ChangeItem> = if let Some(double_values) = next_packet.context.bonus.clone().unwrap().double_values.clone()
                                {
                                    double_values.iter().map(|double_value| {
                                        let v = covert_bs_v(&double_value.bs_v) as i32;
                                        let x = double_value.pos[0] as usize;
                                        let y = double_value.pos[1] as usize;
                                        protocol::ChangeItem { p: (x, y), v: v }
                                    }).collect::<Vec<protocol::ChangeItem>>()
                                } else {vec![]};
                                let clusters: Vec<protocol::ChangeItem> = if let Some(collect_values) = next_packet.context.bonus.clone().unwrap().collect_values.clone()
                                {
                                    collect_values.iter().map(|collect_value| {
                                        let v = covert_bs_v(&collect_value.bs_v) as i32;
                                        let x = collect_value.pos[0] as usize;
                                        let y = collect_value.pos[1] as usize;
                                        protocol::ChangeItem { p: (x, y), v: v }
                                    }).collect::<Vec<protocol::ChangeItem>>()
                                } else {vec![]};
                                let rands: Vec<protocol::RandItem> = if let Some(mystery_values) = next_packet.context.bonus.clone().unwrap().mystery_values.clone()
                                {
                                    mystery_values.iter().map(|mystery_value| {
                                        let v = covert_bs_v(&mystery_value.bs_v) as i32;
                                        let s = char::from_u32((mystery_value.id as u32) + '@' as u32).expect("error symbol");
                                        let x = mystery_value.pos[0] as usize;
                                        let y = mystery_value.pos[1] as usize;
                                        protocol::RandItem { p: (x, y), s, v }
                                    }).collect::<Vec<protocol::RandItem>>()
                                } else {vec![]};
                                let mults0: Vec<Vec<i32>> = next_packet.context.bonus.clone().unwrap().orig_bs_v.iter().map(|row| {row.iter().map(|value| covert_bs_v(value) as i32 / bet_per_line).collect::<Vec<i32>>()}).collect();
                                let mults1: Vec<Vec<i32>> = next_packet.context.bonus.clone().unwrap().bs_v.iter().map(|row| {row.iter().map(|value| covert_bs_v(value) as i32 / bet_per_line).collect::<Vec<i32>>()}).collect();
                                let grid0 = convert_board(&next_packet.context.bonus.clone().unwrap().orig_board.iter().map(|row| row.to_vec()).collect());
                                let grid1 = convert_board(&next_packet.context.bonus.clone().unwrap().board.iter().map(|row| row.to_vec()).collect());
                                (gains, FreeGame::default(), spins.total_win.unwrap_or(0), rounds_left, kind, adds, twices, clusters, rands, mults0, mults1, grid0, grid1)
                            };
                            (ActionKind::RESPIN, total, gains, TrinityPowerLinkInfo {total, respins: rounds_left as i32, lift, lift_new, kind, bombs: vec![], stop: None, adds, twices, clusters, rands, mults0, mults1, grid0, grid1, ..Default::default()},)
                        } else if context.actions == vec!["respin"] {
                            let b = bonus.expect("error get bonus");
                            let respins = b.rounds_left as i32;
                            let kind = convert_bg_type(&b.bonus_mechanic);
                            let adds: Vec<protocol::ChangeItem> = if let Some(boost_values) = context.bonus.clone().unwrap().boost_values.clone()
                            {
                                boost_values.iter().map(|boost_value| {
                                    let v = covert_bs_v(&boost_value.bs_v) as i32;
                                    let x = boost_value.pos[0] as usize;
                                    let y = boost_value.pos[1] as usize;
                                    protocol::ChangeItem { p: (x, y), v: v }
                                }).collect::<Vec<protocol::ChangeItem>>()
                            } else {vec![]};
                            let twices: Vec<protocol::ChangeItem> = if let Some(double_values) = context.bonus.clone().unwrap().double_values.clone()
                            {
                                double_values.iter().map(|double_value| {
                                    let v = covert_bs_v(&double_value.bs_v) as i32;
                                    let x = double_value.pos[0] as usize;
                                    let y = double_value.pos[1] as usize;
                                    protocol::ChangeItem { p: (x, y), v: v }
                                }).collect::<Vec<protocol::ChangeItem>>()
                            } else {vec![]};
                            let clusters: Vec<protocol::ChangeItem> = if let Some(collect_values) = context.bonus.clone().unwrap().collect_values.clone()
                            {
                                collect_values.iter().map(|collect_value| {
                                    let v = covert_bs_v(&collect_value.bs_v) as i32;
                                    let x = collect_value.pos[0] as usize;
                                    let y = collect_value.pos[1] as usize;
                                    protocol::ChangeItem { p: (x, y), v: v }
                                }).collect::<Vec<protocol::ChangeItem>>()
                            } else {vec![]};
                            
                            let rands: Vec<protocol::RandItem> = if let Some(mystery_values) = context.bonus.clone().unwrap().mystery_values.clone()
                            {
                                mystery_values.iter().map(|mystery_value| {
                                    let v = covert_bs_v(&mystery_value.bs_v) as i32;
                                    let s = char::from_u32((mystery_value.id as u32) + '@' as u32).expect("error symbol");
                                    let x = mystery_value.pos[0] as usize;
                                    let y = mystery_value.pos[1] as usize;
                                    protocol::RandItem { p: (x, y), s, v }
                                }).collect::<Vec<protocol::RandItem>>()
                            } else {vec![]};
                            let bet_per_line = context.spins.bet_per_line;
                            let mults0: Vec<Vec<i32>> = context.bonus.clone().unwrap().orig_bs_v.iter().map(|row| {row.iter().map(|value| covert_bs_v(value) as i32 / bet_per_line).collect::<Vec<i32>>()}).collect();
                            let mults1: Vec<Vec<i32>> = context.bonus.clone().unwrap().bs_v.iter().map(|row| {row.iter().map(|value| covert_bs_v(value) as i32 / bet_per_line).collect::<Vec<i32>>()}).collect();

                            let grid0= convert_board(&context.bonus.clone().unwrap().orig_board.iter().map(|row| row.to_vec()).collect());
                            let grid1 = vec![];

                            (ActionKind::RESPIN, b.total_win, vec![], TrinityPowerLinkInfo {total: b.total_win, respins, accum: 0, lift, lift_new, kind, bombs: vec![], stop: None, adds, twices, clusters, rands, mults0, mults1, grid0, grid1, ..Default::default()},)
                        } else if context.actions == vec!["bonus_spins_stop"] || context.actions == vec!["bonus_freespins_stop"] {
                            let b = bonus.expect("error bonus!");
                            let symbol = china_festival::SYMBOL_COIN;
                            let gains = convert_bs_values(b.round_bet.into(), &b.bs_values.iter().map(|row| row.to_vec()).collect());
                            let respins = b.rounds_left as i32;
                            let kind = convert_bg_type(&b.bonus_mechanic);

                            let adds: Vec<protocol::ChangeItem> = if let Some(boost_values) = context.bonus.clone().unwrap().boost_values.clone()
                            {
                                boost_values.iter().map(|boost_value| {
                                    let v = covert_bs_v(&boost_value.bs_v) as i32;
                                    let x = boost_value.pos[0] as usize;
                                    let y = boost_value.pos[1] as usize;
                                    protocol::ChangeItem { p: (x, y), v: v }
                                }).collect::<Vec<protocol::ChangeItem>>()
                            } else {vec![]};
                            let twices: Vec<protocol::ChangeItem> = if let Some(double_values) = context.bonus.clone().unwrap().double_values.clone()
                            {
                                double_values.iter().map(|double_value| {
                                    let v = covert_bs_v(&double_value.bs_v) as i32;
                                    let x = double_value.pos[0] as usize;
                                    let y = double_value.pos[1] as usize;
                                    protocol::ChangeItem { p: (x, y), v: v }
                                }).collect::<Vec<protocol::ChangeItem>>()
                            } else {vec![]};
                            let clusters: Vec<protocol::ChangeItem> = if let Some(collect_values) = context.bonus.clone().unwrap().collect_values.clone()
                            {
                                collect_values.iter().map(|collect_value| {
                                    let v = covert_bs_v(&collect_value.bs_v) as i32;
                                    let x = collect_value.pos[0] as usize;
                                    let y = collect_value.pos[1] as usize;
                                    protocol::ChangeItem { p: (x, y), v: v }
                                }).collect::<Vec<protocol::ChangeItem>>()
                            } else {vec![]};
                            
                            let rands: Vec<protocol::RandItem> = if let Some(mystery_values) = context.bonus.clone().unwrap().mystery_values.clone()
                            {
                                mystery_values.iter().map(|mystery_value| {
                                    let v = covert_bs_v(&mystery_value.bs_v) as i32;
                                    let s = char::from_u32((mystery_value.id as u32) + '@' as u32).expect("error symbol");
                                    let x = mystery_value.pos[0] as usize;
                                    let y = mystery_value.pos[1] as usize;
                                    protocol::RandItem { p: (x, y), s, v }
                                }).collect::<Vec<protocol::RandItem>>()
                            } else {vec![]};

                            let bet_per_line = context.spins.bet_per_line;
                            let mults0: Vec<Vec<i32>> = b.orig_bs_v.iter().map(|row| {row.iter().map(|value| covert_bs_v(value) as i32 / bet_per_line).collect::<Vec<i32>>()}).collect();
                            let mults1: Vec<Vec<i32>> = b.bs_v.iter().map(|row| {row.iter().map(|value| covert_bs_v(value) as i32 / bet_per_line).collect::<Vec<i32>>()}).collect();
                            let grid0 = convert_board(&b.orig_board.iter().map(|row| row.to_vec()).collect());
                            let grid1 = vec![];

                            let link = TrinityPowerLinkInfo {total: b.total_win, respins, accum: b.round_win, lift, lift_new, kind, bombs: vec![], stop: None, adds, twices, clusters, rands, mults0, mults1, grid0, grid1, ..Default::default()};
                            let next_act = if context.actions == vec!["bonus_spins_stop"] {ActionKind::COLLECT} else {ActionKind::FREE_SPIN};
                            (next_act, bonus.as_ref().map(|b| b.total_win).unwrap_or(0), gains, link,)
                        } else {(ActionKind::BET, 0, vec![], TrinityPowerLinkInfo::default(),)};
                        let spin_data: SpinData<TrinityPowerLinkInfo, ReSpinInfo> = SpinData {
                            id: id::GAME_DATA,
                            balance: user.balance,
                            credit_type: 100,
                            result: GameResult {total, stops: vec![0, 0, 0, 0, 0], holds, grid, special: Some(link), gains, ..Default::default()},
                            curr_lines: request.line,
                            curr_bet: request.bet,
                            curr_denom: request.denom,
                            curr_reels: 5,
                            next_act,
                            category: reelset_number.unwrap_or(0),
                            round_id: 0,
                            round_type: RoundDetail::SIMPLE,
                            round_multiplier: 1,
                            promo: Promo {amount: 0, multi: 0,},
                            free: Some(FreeGame::default()),
                            ..Default::default()
                        };
                        let (input, response) = if context.last_action == "spin" ||  context.last_action == "buy_spin" {
                            (PlayerRequest::BetSpin(request), fugaso_core::protocol::Response::GameData(Arc::new(GameData::Spin(spin_data,))),)
                        } else if context.last_action == "respin" {
                            (PlayerRequest::ReSpin, fugaso_core::protocol::Response::GameData(Arc::new(GameData::ReSpin(spin_data,))),)
                        } else {panic!("unsupported!")};
                        Some(FuGaSoTuple {input, output: vec![response],})
                    }
                } else {
                    if command == "start" {l_bet_factor = settings.clone().unwrap().bet_factor[0]}
                    None
                }
            }
            _ => None,
        };
    //}).collect::<Vec<_>>();
    
        if !converted_tr.is_none() {results.push(converted_tr)};
    }
    serde_json::to_writer(File::create(format!("packets_result/{GAME_FUGASO_FOLDER}/{name}")).expect("error file open"), &results,).expect("error write file");
}