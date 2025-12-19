use fugaso_core::protocol::PlayerRequest;
use fugaso_data::{fugaso_action::ActionKind, fugaso_round::RoundDetail};
use fugaso_math::protocol::{id, FreeGame, Gain, GameData, GameResult, Promo, ReSpinInfo, SpinData, };
use fugaso_math::protocol_thor::{self, ChangeItem, Lift, OlympusManiaInfo, WrathStuck };
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_value, Value};
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::BufReader,
    sync::Arc,
};
mod integration;
use integration::FuGaSoTuple;
use maplit::hashmap;
use num_traits::ToPrimitive;
use server::enj::thor_hit_the_bonus::models::network::response::start::Context;
use server::enj::thor_hit_the_bonus::{
    self,
    models::{
        self,
        enums::{ActionsEnum, CommandsEnum, MultiValueEnum},
        network::response::start::Bs,
    },
};

const GAME_SOURCE_NAME: &str = "thor_hit_the_bonus";
const GAME_FUGASO_FOLDER: &str = "olympus_mania";

#[derive(Debug, Serialize, Deserialize)]
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
    let json: Value = from_reader(reader).unwrap();
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
        models::network::response::start::Start {
            settings,
            ..
        } => {
            //Response::Start(v) => {
            //v.settings.paylines
            let settings_on = settings.unwrap();
            let wins: BTreeMap<char, BTreeMap<usize, i32>> = to_value(&settings_on.paytable)
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
                        })
                        .collect::<BTreeMap<usize, i32>>();
                    (symbol, entries)
                })
                .collect();
            let lines =
                settings_on.paylines.iter().map(|p| p.iter().map(|v| v).map(|v| symbols[*v as usize]).map(|v| v.to_string()).collect::<Vec<_>>().join("")).collect::<Vec<_>>();
            Conf {
                wins,
                lines,
            }
        }
        _ => panic!("error config create!"),
    };
    println!("{}", serde_json::to_string(&cfg).expect("error cfg json"));
}

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

#[test]
//#[ignore]
#[allow(unused)]
fn test_convert() {
    /*convert("00-no_win.json");
    convert("bg_type_0.json");
    convert("bg_type_1.json");
    convert("bg_type_2.json");
    convert("bg_type_3.json");
    convert("bg_type_5.json");
    convert("bg_type_6.json");
    convert("bg_type_7.json");*/
    convert("bg_type_10.json");
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

fn convert_bs_values(round_bet: i64, bs_values: &Vec<Vec<i64>>, mps: &Vec<Vec<i32>>) -> Vec<Gain> {
    let symbol = thor_hit_the_bonus::settings::SYM_COIN;
    let sum = (0..bs_values.len()).flat_map(|c| (0..bs_values[c].len()).map(move |r| (c, r))).map(|(c, r)| bs_values[c][r] * round_bet * mps[c][r] as i64).sum::<i64>();
    vec![Gain {
        symbol,
        count: bs_values.iter().flat_map(|b_v| b_v.iter().filter(|v| **v > 0)).count(),
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

fn covert_bs_v(value: &MultiValueEnum, round_bet: i64) -> i32 {
    match value {
        MultiValueEnum::Int(i) => (*i as i64 / round_bet) as i32,
        MultiValueEnum::Float(f) => (*f as i64 / round_bet) as i32,
        MultiValueEnum::String(s) => match s.as_str() {
            "mini" => 10,
            "minor" => 20,
            "major" => 50,
            _ => panic!("illegal jack!"),
        },
    }
}

fn convert_mysteries(mysteries: Vec<Bs>, grid: &Vec<Vec<char>>, round_bet: i64) -> Vec<protocol_thor::RandItem> {
    mysteries
        .iter()
        .map(|mystery_value| {
            let v = mystery_value.value as i32 / round_bet as i32;
            let c = mystery_value.reel as usize;
            let r = mystery_value.position as usize;
            let s = grid[c][r];
            protocol_thor::RandItem {
                p: (c, r),
                s,
                v,
            }
        })
        .collect::<Vec<protocol_thor::RandItem>>()
}

fn covert_wrath_stucks(context: &Context, spins: bool) -> HashMap<i32, Vec<WrathStuck>> {
    let bs = if spins {
        context.spins.bs.as_ref().unwrap()
    } else {
        context.bonus.as_ref().map(|b| &b.bs).unwrap()
    };
    let wrath = bs
        .iter()
        .filter_map(|b| match b.bs_type {
            models::enums::BonusTypeEnum::InfinityWrath => Some(WrathStuck {
                p: (b.reel as usize, b.position as usize),
                c: context.bet20.thor_wrath_spins_left as i32,
                s: context.bet20.thor_state.unwrap() as i32,
                v: Some(b.value as i32 / context.spins.round_bet as i32),
            }),
            models::enums::BonusTypeEnum::InfinityWrathSticky => Some(WrathStuck {
                p: (b.reel as usize, b.position as usize),
                s: context.bet20.thor_state.unwrap() as i32,
                v: Some(b.value as i32 / context.spins.round_bet as i32),
                ..Default::default()
            }),
            _ => None,
        })
        .collect::<Vec<_>>();

    let stucks_wrath = if wrath.len() > 0 {
        hashmap! {
            context.spins.bet_per_line as i32 => wrath
        }
    } else {
        hashmap! {}
    };
    stucks_wrath
}

fn convert_lift(prev_mps: &Vec<Vec<i32>>, mps: &Vec<Vec<i32>>) -> Vec<Lift> {
    let lift_new = (0..mps.len())
        .flat_map(|c| (0..mps[c].len()).map(move |r| (c, r)))
        .filter_map(|(c, r)| {
            if mps[c][r] > prev_mps[c][r] {
                if prev_mps[c][r] == 1 {
                    Some(Lift {
                        pos: (c, r),
                        mult: mps[c][r],
                    })
                } else {
                    Some(Lift {
                        pos: (c, r),
                        mult: mps[c][r] - prev_mps[c][r],
                    })
                }
            } else {
                None
            }
        })
        .collect();
    lift_new
}

#[allow(unused)]
fn convert(name: &str) {
    let mut l_bet_factor: i32 = 20;
    let list = parse_list_game(name);
    let mut prev_lift: Vec<Vec<i32>> = vec![];
    let mut prev_lift_2: Vec<Vec<i32>> = vec![];

    let mut iter = list.into_iter().peekable();
    let mut results = Vec::new();
    let bet_counters = [1, 100, 200, 300];
    while let Some(t) = iter.next() {
        // println!("{}", t.response);
        // println!("<------>");
        let converted_tr: Option<FuGaSoTuple<OlympusManiaInfo, ReSpinInfo>> = match serde_json::from_value(t.response).inspect_err(|e| {eprintln!("err {e}");}).unwrap_or(None)
        {
            Some(models::network::response::start::Start {
                command,
                context,
                modes,
                request_id,
                session_id,
                settings,
                status,
                user,
                origin_data,
            }) => {
                if command == CommandsEnum::Play {
                    let fugaso_tuple = if let Some(l_context) = context.as_ref() {
                        if l_context.last_action == ActionsEnum::BonusInit || l_context.last_action == ActionsEnum::BonusSpinsStop {
                            None
                        } else {
                                let (bet_per_line, lines) = (l_context.spins.bet_per_line, l_context.spins.lines);
                                let selected_mode = l_context.spins.selected_mode.as_ref().map(|v| v.as_usize()).unwrap_or(0);
                                let bet_counter = bet_counters[selected_mode];
                                let request = fugaso_math::math::Request {
                                    bet: bet_per_line as i32,
                                    denom: l_bet_factor,
                                    line: lines as usize,
                                    bet_counter,
                                    ..Default::default()
                                };
                                
                                let bonus = l_context.bonus.as_ref();
                                let (grid, holds, reelset_number) = if l_context.last_action == ActionsEnum::BuySpin || l_context.last_action == ActionsEnum::Spin {
                                    let spins = &l_context.spins;
                                    let grid = convert_board(&spins.board.iter().map(|row| row.iter().map(|&v| v as i32).collect()).collect());
                                    let mut holds = vec![0];
                                    (grid, holds, Some(0))
                                } else if l_context.last_action == ActionsEnum::Respin {
                                    let bonus_on = bonus.expect("bonus is none!");
                                    let grid = convert_board(&bonus_on.board.iter().map(|row| row.iter().map(|&v| v as i32).collect()).collect());
                                    let holds = vec![0];
                                    (grid, holds, Some(0))
                                } else {
                                    panic!("error grid - {}!", l_context.last_action);
                                };

                                let (next_act, total, gains, link) = if l_context.actions == vec![ActionsEnum::Spin, ActionsEnum::BuySpin] && l_context.spins.total_win.unwrap_or(0) > 0
                                {
                                    let spins = l_context.spins.clone();
                                    let gains = convert_win_lines(&spins.winlines.unwrap_or(vec![]));

                                    (
                                        ActionKind::COLLECT,
                                        spins.total_win.unwrap_or(0),
                                        gains,
                                        OlympusManiaInfo {
                                            total: spins.total_win.unwrap_or(0),
                                            lift: spins.mps.clone(),
                                            ..Default::default()
                                        },
                                    )
                                } else if l_context.actions == vec![ActionsEnum::BonusInit] {
                                    let (gains, free, total, rounds_left, rands, mults0, mults1, grid0) = {
                                        let spins = l_context.spins.clone();
                                        let bet_per_line = spins.bet_per_line;
                                        let gains = vec![];
                                        let rounds_left = china_festival::BONUS_COUNT;
                                        let next_packet = serde_json::from_value::<models::network::response::start::Start>(iter.peek().unwrap().response.clone()).unwrap();
                                        let rands: Vec<protocol_thor::RandItem> = vec![];
                                        let bonus = next_packet.context.clone().unwrap().bonus.clone().unwrap();
                                        let mut mults0: Vec<Vec<i32>> =
                                            bonus.orig_bs_v.iter().map(|row| row.iter().map(|value| covert_bs_v(value, bonus.round_bet)).collect::<Vec<i32>>()).collect();
                                        let mults1: Vec<Vec<i32>> =
                                            bonus.bs_v.iter().map(|row| row.iter().map(|value| covert_bs_v(value, bonus.round_bet)).collect::<Vec<i32>>()).collect();
                                        if mults0 == mults1 {
                                            mults0.clear();
                                        }
                                        let grid0 = convert_board(
                                            &next_packet
                                                .context
                                                .clone()
                                                .unwrap()
                                                .bonus
                                                .clone()
                                                .unwrap()
                                                .orig_board
                                                .unwrap_or_default()
                                                .iter()
                                                .map(|row| row.iter().map(|&v| v as i32).collect())
                                                .collect(),
                                        );
                                        (gains, FreeGame::default(), spins.total_win.unwrap_or(0), rounds_left, rands, mults0, mults1, grid0)
                                    };

                                    let stuck_accums = covert_wrath_stucks(l_context, true);
                                    let collects = convert_collects(l_context.spins.new_collectors.as_ref().unwrap_or(&vec![]), l_context.spins.round_bet);

                                    let lift_new = convert_lift(&l_context.spins.board.iter().map(|c| vec![1; c.len()]).collect(), &l_context.spins.mps);
                                    prev_lift = l_context.spins.mps.clone();
                                    (
                                        ActionKind::RESPIN,
                                        total,
                                        gains,
                                        OlympusManiaInfo {
                                            total,
                                            respins: rounds_left as i32,
                                            lift: l_context.spins.mps.clone(),
                                            lift_new,
                                            stop: None,
                                            rands,
                                            mults0,
                                            mults1,
                                            grid0,
                                            stuck_accums,
                                            collects,
                                            ..Default::default()
                                        },
                                    )
                                } else if l_context.actions == vec![ActionsEnum::Respin] {
                                    let b = bonus.expect("error get bonus");
                                    let respins = b.rounds_left as i32;

                                    let rands: Vec<protocol_thor::RandItem> = vec![];
                                    let bet_per_line = l_context.spins.bet_per_line;
                                    let mut mults0: Vec<Vec<i32>> = l_context
                                        .bonus
                                        .clone()
                                        .unwrap()
                                        .orig_bs_v
                                        .iter()
                                        .map(|row| row.iter().map(|value| covert_bs_v(value, b.round_bet)).collect::<Vec<i32>>())
                                        .collect();
                                    let mults1: Vec<Vec<i32>> =
                                        l_context.bonus.clone().unwrap().bs_v.iter().map(|row| row.iter().map(|value| covert_bs_v(value, b.round_bet)).collect::<Vec<i32>>()).collect();
                                    if mults0 == mults1 {
                                        mults0.clear();
                                    }
                                    let mut grid0 = vec![];
                                    let stuck_accums = covert_wrath_stucks(l_context, false);
                                    let collects = convert_collects(b.new_collectors.as_ref().unwrap_or(&vec![]), b.round_bet);
                                    let lift_new = convert_lift(&prev_lift, &b.mps);
                                    prev_lift = b.mps.clone();

                                    (
                                        ActionKind::RESPIN,
                                        b.total_win,
                                        vec![],
                                        OlympusManiaInfo {
                                            total: b.total_win,
                                            respins,
                                            accum: 0,
                                            lift: b.mps.clone(),
                                            lift_new,
                                            stop: None,
                                            rands,
                                            mults0,
                                            mults1,
                                            grid0,
                                            stuck_accums,
                                            collects,
                                            ..Default::default()
                                        },
                                    )
                                } else if l_context.actions == vec![ActionsEnum::BonusSpinsStop] {
                                    let b = bonus.expect("error bonus!");

                                    let symbol = thor_hit_the_bonus::settings::SYM_COIN;
                                    let gains = convert_bs_values(b.round_bet.into(), &b.bs_values, &b.mps);
                                    let respins = b.rounds_left as i32;

                                    let rands: Vec<protocol_thor::RandItem> = if let Some(mystery_values) = l_context.bonus.clone().unwrap().mysteries.clone() {
                                        convert_mysteries(mystery_values, &grid, b.round_bet)
                                    } else {
                                        vec![]
                                    };

                                    let bet_per_line = l_context.spins.bet_per_line;
                                    let mut mults0: Vec<Vec<i32>> =
                                        b.orig_bs_v.clone().iter().map(|row| row.iter().map(|value| covert_bs_v(value, b.round_bet)).collect::<Vec<i32>>()).collect();
                                    let mut mults1: Vec<Vec<i32>> = b.bs_v.iter().map(|row| row.iter().map(|value| covert_bs_v(value, b.round_bet)).collect::<Vec<i32>>()).collect();
                                    let lift = b.mps.clone();
                                    for c in 0..mults1.len() {
                                        for r in 0..mults1[c].len(){
                                            mults1[c][r] /= lift[c][r];
                                        }
                                    }
                                    let mut grid0 = convert_board(&b.orig_board.clone().unwrap_or_default().iter().map(|row| row.iter().map(|&v| v as i32).collect()).collect());
                                    if grid0 == grid {
                                        grid0.clear();
                                    }
                                    if mults0 == mults1 {
                                        mults0.clear();
                                    }
                                    let stuck_accums = covert_wrath_stucks(l_context, false);
                                    let collects = convert_collects(b.new_collectors.as_ref().unwrap_or(&vec![]), b.round_bet);
                                    let lift_new = convert_lift(&prev_lift, &b.mps);
                                    prev_lift = b.mps.clone();

                                    let link = OlympusManiaInfo {
                                        total: b.total_win,
                                        respins,
                                        accum: b.round_win,
                                        lift: b.mps.clone(),
                                        lift_new,
                                        rands,
                                        mults1,
                                        grid0,
                                        stuck_accums,
                                        collects,
                                        ..Default::default()
                                    };
                                    let next_act = if l_context.actions == vec![ActionsEnum::BonusSpinsStop] {
                                        ActionKind::COLLECT
                                    } else {
                                        ActionKind::FREE_SPIN
                                    };
                                    (next_act, bonus.as_ref().map(|b| b.total_win).unwrap_or(0), gains, link)
                                } else {
                                    (ActionKind::BET, 0, vec![], OlympusManiaInfo::default())
                                };
                                let spin_data: SpinData<OlympusManiaInfo, ReSpinInfo> = SpinData {
                                    id: id::GAME_DATA,
                                    balance: user.balance,
                                    credit_type: 100,
                                    result: GameResult {
                                        total,
                                        stops: vec![0, 0, 0, 0, 0],
                                        holds,
                                        grid,
                                        special: Some(link),
                                        gains,
                                        ..Default::default()
                                    },
                                    curr_lines: request.line,
                                    curr_bet: request.bet,
                                    curr_denom: request.denom,
                                    curr_reels: 5,
                                    next_act,
                                    category: reelset_number.unwrap_or(0),
                                    round_id: 0,
                                    round_type: RoundDetail::SIMPLE,
                                    round_multiplier: 1,
                                    promo: Promo {
                                        amount: 0,
                                        multi: 0,
                                    },
                                    free: Some(FreeGame::default()),
                                    ..Default::default()
                                };
                                let (input, response) = if l_context.last_action == ActionsEnum::Spin || l_context.last_action == ActionsEnum::BuySpin {
                                    (PlayerRequest::BetSpin(request), fugaso_core::protocol::Response::GameData(Arc::new(GameData::Spin(spin_data))))
                                } else if l_context.last_action == ActionsEnum::Respin {
                                    (PlayerRequest::ReSpin, fugaso_core::protocol::Response::GameData(Arc::new(GameData::ReSpin(spin_data))))
                                } else {
                                    panic!("unsupported!")
                                };
                                Some(FuGaSoTuple {
                                    input,
                                    output: vec![response],
                                })
                        }
                    } else {
                        None
                    };
                    fugaso_tuple
                } else {
                    if command == CommandsEnum::Start {l_bet_factor = settings.unwrap().bet_factor[0] as i32}
                    None
                }
            }
            _ => None,
        };
        //}).collect::<Vec<_>>();

        if !converted_tr.is_none() {
            results.push(converted_tr)
        };
    }
    serde_json::to_writer(File::create(format!("packets_result/{GAME_FUGASO_FOLDER}/{name}")).expect("error file open"), &results).expect("error write file");
}

fn convert_collects(new_collectors: &Vec<Bs>, round_bet: i64) -> Vec<ChangeItem> {
    new_collectors
        .iter()
        .map(|c| ChangeItem {
            p: (c.reel as usize, c.position as usize),
            v: (c.value / round_bet) as i32,
        })
        .collect()
}
