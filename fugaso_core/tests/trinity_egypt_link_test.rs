//use std::ops::Add;
use std::{collections::BTreeMap, fs::File, io::BufReader, sync::Arc};

use fugaso_core::protocol::PlayerRequest;
use fugaso_data::{fugaso_action::ActionKind, fugaso_round::RoundDetail};
use fugaso_math::protocol::TrinityEgyptKind;
use fugaso_math::protocol;
use fugaso_math::protocol::{id, FreeGame, Gain, GameData, GameResult, Promo, ReSpinInfo, SpinData, TrinityEgyptLinkInfo};
use integration::FuGaSoTuple;
use num_traits::ToPrimitive;
use serde::Serialize;
use serde::Deserialize;
//use std::iter::Peekable;

mod integration;
use serde_json::Value;
use serde_json::to_value;
use server::bng::three_aztec_temples::{self, models};
use models::enums::{ActionsEnum, CommandsEnum, MultiValueEnum};

const GAME_SOURCE_NAME: &str = "three_aztec_temples";
const GAME_FUGASO_FOLDER: &str = "trinity_egypt_link";

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct TupleGame {
    #[serde(rename = "in")]
    request: Value,
    #[serde(rename = "out")]
    response: Value,
}

fn parse_list_game(p: &str) -> Vec<TupleGame> {
    let name = format!("packets/{GAME_SOURCE_NAME}/{p}");
    let file = File::open(&name).expect(&format!("error open {name}!"));
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
    let file = File::open(format!("packets/{GAME_SOURCE_NAME}/00-no_win.json")).unwrap();
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
            let wins: BTreeMap<char, BTreeMap<usize, i32>> = to_value(&settings.clone().paytable)
                .expect("paytable to_value failed")
                .as_object()
                .expect("not an object")
                .iter()
                .map(|(key, value)| {
                    let number = key.parse::<u32>().expect("invalid key");
                    let symbol = char::from_u32(number + ('A' as u32) - 1).expect("invalid symbol");
                    let entries = value
                        .as_array()
                        .expect("not an array")
                        .iter()
                        .map(|v| {
                            let occurrences = v.get("occurrences").expect("no occurrences").as_u64().expect("invalid occurrences") as usize;
                            let multiplier = v.get("multiplier").expect("no multiplier").as_i64().expect("invalid multiplier") as i32;
                            (occurrences, multiplier)
                        }).collect::<BTreeMap<usize, i32>>();
                    (symbol, entries)
                }).collect();
            let lines = settings.clone().paylines.iter().map(|p| {
                p.iter()
                    .map(|v| v)
                    .map(|v| symbols[*v as usize])
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            }).collect::<Vec<_>>();
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
    convert("00-no_win.json");
    convert("01-win.json");
    convert("10-boost.json");
    convert("11-boost_next_collect.json");
    convert("12-boost_next_mult.json");
    convert("14-boost_next_collect_next_mult.json");
    convert("15-boost_next_mult_next_collect.json");
    convert("20-collect.json");
    convert("21-collect_next_boost.json");
    convert("24-collect_next_mult_next_boost.json");
    convert("25-collect_next_boost_next_mult.json");
    convert("30-mult.json");
    convert("30-mult_x3.json");
    /*convert("03-jackpots.json");
    convert("11-boost_after_double.json");
    convert("12-boost_after_collect.json");
    convert("14-boost_after_double_after_collect.json");
    convert("15-boost_after_collect_after_double.json");
    convert("16-boost_jackpot.json");
    convert("17-boost_with_mystery_boost.json");
    convert("171-boost_with_mystery_boost.json");
    convert("18-boost_with_mystery_double.json");
    convert("19-boost_with_mystery_collect.json");

    convert("21-double_after_boost.json");
    convert("22-double_after_collect.json");
    convert("24-double_after_collect_after_boost.json");
    convert("25-double_after_boost_after_collect.json");
    convert("26-double_jackpot.json");
    

    convert("27-double_with_mystery_double.json");
    convert("28-double_with_mystery_boost.json");
    convert("29-double_with_mystery_collect.json");
    
    convert("31-collect_after_double.json");
    convert("32-collect_after_boost.json");
    convert("34-collect_after_double_after_boost.json");
    convert("35-collect_after_boost_after_double.json");
    convert("36-collect_jackpot.json");
    convert("37-collect_with_mystery_collect.json");
    convert("38-collect_with_mystery_boost.json");
    convert("39-collect_with_mystery_double.json");

    convert("11-boost_next_double.json");
    convert("12-boost_next_collect.json");
    convert("14-boost_next_double_next_collect.json");
    convert("15-boost_next_collect_next_double.json");
    convert("20-double.json");
    convert("21-double_next_boost.json");
    convert("24-double_next_collect_next_boost.json");
    convert("25-double_next_boost_next_collect.json");
    convert("30-collect.json");
    convert("33-collect_next_boost_and_double.json");
    convert("34-collect_next_double_next_boost.json");
    convert("35-collect_next_boost_next_double.json");
    convert("40-boost_and_double.json");
    convert("41-boost_and_double_next_collect.json");
    convert("43-double_and_collect_next_boost.json");
    convert("44-collect_and_boost.json");
    convert("45-collect_and_boost_next_double.json");
    convert("50-boost_and_double_and_collect.json");
    convert("33-mult_next_boost_and_collect.json");
    convert("34-mult_next_collect_next_boost.json");
    convert("35-mult_next_boost_next_collect.json");
    convert("40-boost_and_collect.json");
    convert("41-boost_and_collect_next_mult.json");
    convert("43-collect_and_mult_next_boost.json");
    convert("44-mult_and_boost.json");
    convert("45-mult_and_boost_next_collect.json");
    convert("50-boost_and_collect_and_mult.json");
    convert("61-coin_next_boost.json");
    convert("61-coin_next_boost_next_collect.json");
    convert("61-coin_next_boost_next_collect_next_mult.json");
    convert("62-coin_next_collect_next_boost.json");
    convert("62-coin_next_collect_next_mult_next_boost.json");
    convert("63-coin_next_mult_next_boost_next_collect.json");
    convert("63-coin_next_mult_next_collect_next_boost.json");*/
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

fn convert_win_lines(win_lines: &Vec<models::network::response::start::Winlines>) -> Vec<Gain> {
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
    let symbol = three_aztec_temples::settings::SYMBOL_COIN;
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

fn convert_bg_type(bonus_mechanic: &Vec<i32>) -> Option<TrinityEgyptKind> {
    match (bonus_mechanic.iter().sum(), bonus_mechanic.len()) {
        (1, 1) => {Some(TrinityEgyptKind::Add)}
        (2, 1) => {Some(TrinityEgyptKind::Cluster)}
        (3, 1) => {Some(TrinityEgyptKind::Lift)}
        (3, 2) => {Some(TrinityEgyptKind::AddCluster)}
        (5, 2) => {Some(TrinityEgyptKind::ClusterLift)}
        (4, 2) => {Some(TrinityEgyptKind::AddLift)}
        (6, 3) => {Some(TrinityEgyptKind::AddClusterLift)}
        _ => {None}
    }
}

fn covert_bs_v(value: &MultiValueEnum) -> i32 {
    match value {
        MultiValueEnum::Int(i) => *i as i32,
        MultiValueEnum::Float(f) => *f as i32,
        MultiValueEnum::String(s) => match s.as_str() {
            "mini" => 300,
            "minor" => 600,
            "major" => 2000,
            _ => 0,
        },
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
        let converted_tr: Option<FuGaSoTuple<TrinityEgyptLinkInfo, ReSpinInfo>> = match serde_json::from_value(t.response).inspect_err(|e|{
            eprintln!("{e}");
        }).unwrap_or(None) {
            Some(models::network::response::start::Start { command, context, modes, request_id, roundnum, session_id, settings, status, user }) => {
                if command == CommandsEnum::Play {
                    let fugaso_tuple =
                    if let Some(l_context) = context.as_ref() {
                        if l_context.last_action == ActionsEnum::BonusInit || l_context.last_action == ActionsEnum::BonusSpinsStop {
                            None
                        } else {
                            let (bet_per_line, lines) = (l_context.spins.bet_per_line, l_context.spins.lines);
                            let selected_mode = l_context.spins.selected_mode.as_ref().map(|v|v.as_usize()).unwrap_or(0);
                            let bet_counter = bet_counters[selected_mode];
                            let request = fugaso_math::math::Request {bet: bet_per_line as i32, denom: l_bet_factor, line: lines as usize, bet_counter, ..Default::default()};
                            let bonus = l_context.bonus.as_ref();
                            let (grid, holds, reelset_number) =
                            if l_context.last_action == ActionsEnum::BuySpin || l_context.last_action == ActionsEnum::Spin {
                                let spins = &l_context.spins;
                                let grid = convert_board(&spins.board.iter().map(|row| row.iter().map(|&v| v as i32).collect()).collect());
                                let mut holds = vec![0];
                                (grid, holds, Some(0))
                            } else if l_context.last_action == ActionsEnum::Respin {
                                let bonus_on = bonus.expect("bonus is none!");
                                let grid = convert_board(&bonus_on.board.iter().map(|row| row.iter().map(|&v| v as i32).collect()).collect());
                                let holds = vec![0];
                                (grid, holds, Some(0))
                            } else {panic!("error grid - {}!", l_context.last_action);};

                            let (next_act, total, gains, link) = 
                            if l_context.actions == vec![ActionsEnum::Spin, ActionsEnum::BuySpin] && l_context.spins.total_win.unwrap_or(0) > 0 {
                                let spins = l_context.spins.clone();
                                let gains = convert_win_lines(&spins.winlines.unwrap_or(vec![]));
                                (ActionKind::COLLECT, spins.total_win.unwrap_or(0), gains, TrinityEgyptLinkInfo {total: spins.total_win.unwrap_or(0), ..Default::default()},)
                            } else if l_context.actions == vec![ActionsEnum::BonusInit] {
                                let (gains, free, total, rounds_left, kind, adds, lift, clusters, rands, mults0, mults1, grid0, grid1) = {
                                    let spins = l_context.spins.clone();
                                    let bet_per_line = spins.bet_per_line;
                                    let gains = convert_win_lines(&spins.winlines.unwrap_or(vec![]));
                                    let rounds_left = china_festival::BONUS_COUNT;
                                    let kind = convert_bg_type(&spins.bonus_mechanic.unwrap_or_default().into_iter().map(|v| v as i32).collect());
                                    let next_packet = serde_json::from_value::<models::network::response::start::Start>(iter.peek().unwrap().response.clone()).unwrap(); 
                                    let adds: Vec<protocol::ChangeItem> = if let Some(boost_values) = next_packet.context.clone().unwrap().bonus.clone().unwrap().boost_values.clone()
                                    {
                                        boost_values.iter().map(|boost_value| {
                                            let v = covert_bs_v(&boost_value.bs_v) as i32;
                                            let x = boost_value.pos[0] as usize;
                                            let y = boost_value.pos[1] as usize;
                                            protocol::ChangeItem { p: (x, y), v: v }
                                        }).collect::<Vec<protocol::ChangeItem>>()
                                    } else {vec![]};
                                    let lift = if let Some(double_values) = next_packet.context.clone().unwrap().bonus.clone().unwrap().multi_values.clone()
                                    {
                                        double_values.iter().map(|double_value| {
                                            let v = covert_bs_v(&double_value.bs_v) as i32;
                                            let x = double_value.pos[0] as usize;
                                            let y = double_value.pos[1] as usize;
                                            let m = double_value.mult_value as i32;
                                            protocol::MultItem { p: (x, y), v, m }
                                        }).collect::<Vec<_>>()
                                    } else {vec![]};
                                    let clusters: Vec<protocol::ChangeItem> = if let Some(collect_values) = next_packet.context.clone().unwrap().bonus.clone().unwrap().collect_values.clone()
                                    {
                                        collect_values.iter().map(|collect_value| {
                                            let v = covert_bs_v(&collect_value.bs_v) as i32;
                                            let x = collect_value.pos[0] as usize;
                                            let y = collect_value.pos[1] as usize;
                                            protocol::ChangeItem { p: (x, y), v: v }
                                        }).collect::<Vec<protocol::ChangeItem>>()
                                    } else {vec![]};
                                    let rands: Vec<protocol::RandItem> = if let Some(mystery_values) = next_packet.context.clone().unwrap().bonus.clone().unwrap().mystery_values.clone()
                                    {
                                        mystery_values.iter().map(|mystery_value| {
                                            let v = covert_bs_v(&mystery_value.bs_v) as i32;
                                            let s = char::from_u32((mystery_value.id as u32) + '@' as u32).expect("error symbol");
                                            let x = mystery_value.pos[0] as usize;
                                            let y = mystery_value.pos[1] as usize;
                                            protocol::RandItem { p: (x, y), s, v }
                                        }).collect::<Vec<protocol::RandItem>>()
                                    } else {vec![]};
                                    let bonus = next_packet.context.clone().unwrap().bonus.clone().unwrap();
                                    let mults0: Vec<Vec<i32>> = bonus.origin_bs_v.unwrap_or_default().iter().map(|row| {row.iter().map(|value| covert_bs_v(value) as i32 / bet_per_line as i32).collect::<Vec<i32>>()}).collect();
                                    let mults1: Vec<Vec<i32>> = bonus.bs_v.iter().map(|row| {row.iter().map(|value| covert_bs_v(value) as i32 / bet_per_line as i32).collect::<Vec<i32>>()}).collect();
                                    let grid0 = convert_board(&next_packet.context.clone().unwrap().bonus.clone().unwrap().origin_board.unwrap_or_default().iter().map(|row| row.iter().map(|&v| v as i32).collect()).collect());
                                    let grid1 = vec![];//convert_board(&next_packet.context.clone().unwrap().bonus.clone().unwrap().board.iter().map(|row| row.iter().map(|&v| v as i32).collect()).collect());
                                    (gains, FreeGame::default(), spins.total_win.unwrap_or(0), rounds_left, kind, adds, lift, clusters, rands, mults0, mults1, grid0, grid1)
                                };
                                (ActionKind::RESPIN, total, gains, TrinityEgyptLinkInfo {total, respins: rounds_left as i32, kind, bombs: vec![], stop: None, adds, lift, clusters, rands, mults0, mults1, grid0, grid1, ..Default::default()},)
                            } else if l_context.actions == vec![ActionsEnum::Respin] {
                                let b = bonus.expect("error get bonus");
                                let respins = b.rounds_left as i32;
                                let kind = convert_bg_type(&b.bonus_mechanic.clone().unwrap_or_default().into_iter().map(|v| v as i32).collect());
                                let adds: Vec<protocol::ChangeItem> = if let Some(boost_values) = l_context.bonus.clone().unwrap().boost_values.clone()
                                {
                                    boost_values.iter().map(|boost_value| {
                                        let v = covert_bs_v(&boost_value.bs_v) as i32;
                                        let x = boost_value.pos[0] as usize;
                                        let y = boost_value.pos[1] as usize;
                                        protocol::ChangeItem { p: (x, y), v: v }
                                    }).collect::<Vec<protocol::ChangeItem>>()
                                } else {vec![]};
                                let lift = if let Some(double_values) = l_context.bonus.clone().unwrap().multi_values.clone()
                                {
                                    double_values.iter().map(|double_value| {
                                        let v = covert_bs_v(&double_value.bs_v) as i32;
                                        let x = double_value.pos[0] as usize;
                                        let y = double_value.pos[1] as usize;
                                        let m = double_value.mult_value as i32;
                                        protocol::MultItem { p: (x, y), v: v, m }
                                    }).collect::<Vec<_>>()
                                } else {vec![]};
                                let clusters: Vec<protocol::ChangeItem> = if let Some(collect_values) = l_context.bonus.clone().unwrap().collect_values.clone()
                                {
                                    collect_values.iter().map(|collect_value| {
                                        let v = covert_bs_v(&collect_value.bs_v) as i32;
                                        let x = collect_value.pos[0] as usize;
                                        let y = collect_value.pos[1] as usize;
                                        protocol::ChangeItem { p: (x, y), v: v }
                                    }).collect::<Vec<protocol::ChangeItem>>()
                                } else {vec![]};
                                
                                let rands: Vec<protocol::RandItem> = if let Some(mystery_values) = l_context.bonus.clone().unwrap().mystery_values.clone()
                                {
                                    mystery_values.iter().map(|mystery_value| {
                                        let v = covert_bs_v(&mystery_value.bs_v) as i32;
                                        let s = char::from_u32((mystery_value.id as u32) + '@' as u32).expect("error symbol");
                                        let x = mystery_value.pos[0] as usize;
                                        let y = mystery_value.pos[1] as usize;
                                        protocol::RandItem { p: (x, y), s, v }
                                    }).collect::<Vec<protocol::RandItem>>()
                                } else {vec![]};
                                let bet_per_line = l_context.spins.bet_per_line;
                                let mults0: Vec<Vec<i32>> = l_context.bonus.clone().unwrap().origin_bs_v.unwrap_or_default().iter().map(|row| {row.iter().map(|value| covert_bs_v(value) as i32 / bet_per_line as i32).collect::<Vec<i32>>()}).collect();
                                let mults1: Vec<Vec<i32>> = l_context.bonus.clone().unwrap().bs_v.iter().map(|row| {row.iter().map(|value| covert_bs_v(value) as i32 / bet_per_line as i32).collect::<Vec<i32>>()}).collect();

                                let grid0= convert_board(&l_context.bonus.clone().unwrap().origin_board.unwrap_or_default().iter().map(|row| row.iter().map(|&v| v as i32).collect()).collect());
                                let grid1 = vec![];

                                (ActionKind::RESPIN, b.total_win, vec![], TrinityEgyptLinkInfo {total: b.total_win, respins, accum: 0, kind, bombs: vec![], stop: None, adds, lift, clusters, rands, mults0, mults1, grid0, grid1, ..Default::default()},)
                            } else if l_context.actions == vec![ActionsEnum::BonusSpinsStop] {
                                let b = bonus.expect("error bonus!");
                                let symbol = three_aztec_temples::settings::SYMBOL_COIN;
                                let gains = convert_bs_values(b.round_bet.into(), &b.bs_values.iter().map(|row| row.iter().map(|v| v.as_f64()).collect()).collect());
                                let respins = b.rounds_left as i32;
                                let kind = convert_bg_type(&b.bonus_mechanic.clone().unwrap_or_default().into_iter().map(|v| v as i32).collect());

                                let adds: Vec<protocol::ChangeItem> = if let Some(boost_values) = l_context.bonus.clone().unwrap().boost_values.clone()
                                {
                                    boost_values.iter().map(|boost_value| {
                                        let v = covert_bs_v(&boost_value.bs_v) as i32;
                                        let x = boost_value.pos[0] as usize;
                                        let y = boost_value.pos[1] as usize;
                                        protocol::ChangeItem { p: (x, y), v: v }
                                    }).collect::<Vec<protocol::ChangeItem>>()
                                } else {vec![]};
                                let lift = if let Some(double_values) = l_context.bonus.clone().unwrap().multi_values.clone()
                                {
                                    double_values.iter().map(|double_value| {
                                        let v = covert_bs_v(&double_value.bs_v) as i32;
                                        let x = double_value.pos[0] as usize;
                                        let y = double_value.pos[1] as usize;
                                        let m = double_value.mult_value as i32;
                                        protocol::MultItem { p: (x, y), v, m }
                                    }).collect::<Vec<_>>()
                                } else {vec![]};
                                let clusters: Vec<protocol::ChangeItem> = if let Some(collect_values) = l_context.bonus.clone().unwrap().collect_values.clone()
                                {
                                    collect_values.iter().map(|collect_value| {
                                        let v = covert_bs_v(&collect_value.bs_v) as i32;
                                        let x = collect_value.pos[0] as usize;
                                        let y = collect_value.pos[1] as usize;
                                        protocol::ChangeItem { p: (x, y), v: v }
                                    }).collect::<Vec<protocol::ChangeItem>>()
                                } else {vec![]};
                                
                                let rands: Vec<protocol::RandItem> = if let Some(mystery_values) = l_context.bonus.clone().unwrap().mystery_values.clone()
                                {
                                    mystery_values.iter().map(|mystery_value| {
                                        let v = covert_bs_v(&mystery_value.bs_v) as i32;
                                        let s = char::from_u32((mystery_value.id as u32) + '@' as u32).expect("error symbol");
                                        let x = mystery_value.pos[0] as usize;
                                        let y = mystery_value.pos[1] as usize;
                                        protocol::RandItem { p: (x, y), s, v }
                                    }).collect::<Vec<protocol::RandItem>>()
                                } else {vec![]};

                                let bet_per_line = l_context.spins.bet_per_line;
                                let mults0: Vec<Vec<i32>> = b.origin_bs_v.clone().unwrap_or_default().iter().map(|row| {row.iter().map(|value| covert_bs_v(value) as i32 / bet_per_line as i32).collect::<Vec<i32>>()}).collect();
                                let mults1: Vec<Vec<i32>> = b.bs_v.iter().map(|row| {row.iter().map(|value| covert_bs_v(value) as i32 / bet_per_line as i32).collect::<Vec<i32>>()}).collect();
                                let grid0 = convert_board(&b.origin_board.clone().unwrap_or_default().iter().map(|row| row.iter().map(|&v| v as i32).collect()).collect());
                                let grid1 = vec![];

                                let link = TrinityEgyptLinkInfo {total: b.total_win, respins, accum: b.round_win, kind, bombs: vec![], stop: None, adds, lift, clusters, rands, mults0, mults1, grid0, grid1, ..Default::default()};
                                let next_act = if l_context.actions == vec![ActionsEnum::BonusSpinsStop] {ActionKind::COLLECT} else {ActionKind::FREE_SPIN};
                                (next_act, bonus.as_ref().map(|b| b.total_win).unwrap_or(0), gains, link,)
                            } else {(ActionKind::BET, 0, vec![], TrinityEgyptLinkInfo::default(),)};
                            let spin_data: SpinData<TrinityEgyptLinkInfo, ReSpinInfo> = SpinData {
                                id: id::GAME_DATA,
                                balance: user.unwrap().balance,
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
                            let (input, response) = if l_context.last_action == ActionsEnum::Spin ||  l_context.last_action == ActionsEnum::BuySpin {
                                (PlayerRequest::BetSpin(request), fugaso_core::protocol::Response::GameData(Arc::new(GameData::Spin(spin_data,))),)
                            } else if l_context.last_action == ActionsEnum::Respin {
                                (PlayerRequest::ReSpin, fugaso_core::protocol::Response::GameData(Arc::new(GameData::ReSpin(spin_data,))),)
                            } else {panic!("unsupported!")};
                            Some(FuGaSoTuple {input, output: vec![response],})
                        }
                    } else {None};
                    fugaso_tuple
                } else {
                    if command == CommandsEnum::Start {l_bet_factor = settings.clone().bet_factor[0] as i32}
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

