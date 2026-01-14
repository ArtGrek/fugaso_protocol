
//use num_traits::float::TotalOrder;
use serde::{Serialize, Deserialize, };
use serde_json::Value;
use serde_path_to_error::deserialize;
use std::{fs::File, io::BufReader, sync::Arc,};
use std::collections::{BTreeMap, };

mod integration;
use integration::FuGaSoTuple;
use fugaso_core::protocol::PlayerRequest;
use fugaso_data::{fugaso_action::ActionKind, fugaso_round::RoundDetail};
use fugaso_math::protocol::{GameData, SpinData, id, GameResult, Gain, ReSpinInfo, Promo, };
use fugaso_math::protocol_mega_thunder::{
	MegaThunderLinkInfo, LiftItem, GrandLightningIn, GrandLightningOut, CommandEnum, ActionNameEnum, Settings, Winlines, 
	COIN, JACKPOT, MULTI, MINI_VALUE, MINI_CHAR, MINOR_VALUE, MINOR_CHAR, MAJOR_VALUE, MAJOR_CHAR, 
};
const GAME_SOURCE_NAME: &str = "grand_lightning";
const GAME_FUGASO_FOLDER: &str = "mega_thunder_link";
pub const BOARD_HEIGHT: usize = 3;
pub const BOARD_WIDTH: usize = 5;
const DEV_PACKET_NAME: &str = "44fe51f0487c4118a5408a3c9c3af79b.json";

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct TupleGame {
    #[serde(rename = "in")]
    request: Value,
    #[serde(rename = "out")]
    response: Value,
}

fn parse_packet(p: &str) -> Vec<TupleGame> {
    let name = format!("packets/{GAME_SOURCE_NAME}/{p}");
    let file = File::open(&name).expect(&format!("error open {name}!"));
    let reader = BufReader::new(file);
    let response = serde_json::from_reader(reader).expect(&format!("error read {p}!"));
    response
}

#[test]
#[allow(unused)]
fn test_structure() {
    for (idx, tuple) in parse_packet(DEV_PACKET_NAME).into_iter().enumerate() {
        let val: Value = tuple.response.clone();
        let json_str = val.to_string();
        let mut deserializer = serde_json::Deserializer::from_str(&json_str);
        match deserialize::<_, Option<GrandLightningOut>>(&mut deserializer) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("[{DEV_PACKET_NAME}] [ERROR]\n→ line {idx}\n→ body: {json_str}\n→ reason: {err}\n");
            }
        }
    }
}

#[test]
//#[ignore]
#[allow(unused)]
fn test_convert() {
    convert("00-no_win.json");
    convert("01-win.json");
    convert("11-s-multi_2.json");
    convert("23-fs-multi_5.json");
    convert("24-fs-grand.json");
    convert("25-fs-grand.json");
    convert("44fe51f0487c4118a5408a3c9c3af79b.json");
}

fn convert(name: &str) {
	//start additional global variables
    let bet_counters = [1, 70, 300];
	let mut game_settings: Option<Settings> = None;
	//end additional global variables
    let list = parse_packet(name);
    let mut iter = list.into_iter().peekable();
    let mut results: Vec<FuGaSoTuple<MegaThunderLinkInfo, ReSpinInfo>> = Vec::new();
    while let Some(tuple) = iter.next() {let converted_tr_opt: Option<Vec<FuGaSoTuple<MegaThunderLinkInfo, ReSpinInfo>>> = 
		if let Ok(Some(grand_lightning_in)) = serde_json::from_value::<Option<GrandLightningIn>>(tuple.request) {
			if let Ok(Some(grand_lightning_out)) = serde_json::from_value::<Option<GrandLightningOut>>(tuple.response) {
				//start edit converter
				if grand_lightning_in.command == CommandEnum::Play {
					let mut round_tansactions = Vec::new();
					//start parse play transactions
					let action = grand_lightning_in.action.map(|action| {action}).expect("play action not impement");
					let context = grand_lightning_out.context.map(|context| {context}).expect("play context not impement");
					let user = grand_lightning_out.user.map(|user| {user}).expect("play user not impement");
					if action.name == ActionNameEnum::BonusInit {
						//start pars respin transactions
						continue;
					#[cfg(any())]
					{
						let response: SpinData<MegaThunderLinkInfo, ReSpinInfo> = SpinData::default();
						//end pars respin transactions
						let input = PlayerRequest::ReSpin;
						let output = fugaso_core::protocol::Response::GameData(Arc::new(GameData::ReSpin(response.clone())));
						round_tansactions.push(FuGaSoTuple {input, output: vec![output] });
						Some(round_tansactions)
					}
					} else if action.name == ActionNameEnum::Respin {
						//start pars respin transactions
						let response: SpinData<MegaThunderLinkInfo, ReSpinInfo> = {
							let id = id::GAME_DATA;
							let balance = user.balance;
							let credit_type = 100;
							let curr_lines = context.spins.lines as usize;
							let curr_bet = context.spins.bet_per_line as i32;
							let curr_denom = 10;
							let curr_reels = 5;
							let category = 0;
							let round_id = 0;
							let round_type = RoundDetail::SIMPLE;
							let round_multiplier = 1;
							//result
							let bonus = context.bonus.map(|bonus| {bonus}).expect("play respin bonus not impement");
							let total = context.spins.total_win.unwrap_or(0) +  bonus.total_win;
							let stops = vec![0, 0, 0, 0, 0];
							let holds = vec![0];
							let grid0 = vec![];
							let grid = convert_board(&bonus.board, &bonus.bs_values);
                            let gains = vec![];
							//special
							let respins = bonus.rounds_left as i32;
							let accum = bonus.total_win;
							let overlay = None;
							let next_act = if context.actions == vec![ActionNameEnum::BonusSpinsStop] {ActionKind::COLLECT} else {ActionKind::RESPIN};
							let (mults, lifts): (Vec<Vec<i32>>, Vec<Vec<i32>>) = bonus.bs_values.iter().enumerate().map(|(col_num, col)| {
								col.iter().enumerate().map(|(row_num, &v)| {
									if bonus.board[col_num][row_num] != 0 {(v as i32, bonus.bs_multi[col_num][row_num] as i32)} else {(0, 0)}
								}).collect()
							}).collect();
							let mut lifts_new = vec![];
							bonus.changes.iter().for_each(|change| {
									if change.symbol == MULTI {
										lifts_new.push(LiftItem {
											p: (change.reel as usize, change.row as usize),
											m: change.multiplier.unwrap_or(1) as i32,
											v: change.value as i32
										});
									};
							});
							let grand = bonus.grand.iter().map(|&v| v as i32).collect::<Vec<i32>>();
							SpinData { 
								id, 
								balance, 
								credit_type, 
								result: GameResult { 
									total, 
									stops, 
									holds, 
									cards: Default::default(), 
									grid0, 
									grid, 
									special: Some(MegaThunderLinkInfo { 
										total, 
										respins, 
										accum, 
										stop: Default::default(), 
										overlay, 
										mults,
										lifts,
										lifts_new,
										grand,
									}),
									gains, 
									restore: Some(ReSpinInfo { 
										total: Default::default(), 
										mults: Default::default(), 
										respins: Default::default(), 
										overlay: Default::default(), 
										accum: Default::default() 
									}), 
									extra_data: None
								}, 
								curr_lines, 
								curr_bet, 
								curr_denom, 
								curr_reels, 
								next_act, 
								category, 
								round_id, 
								round_type,
								round_multiplier, 
								promo: Promo { 
									amount: Default::default(), 
									multi: Default::default() 
								}, 
								free: None
							}
						};
						//end pars respin transactions
						let input = PlayerRequest::ReSpin;
						let output = fugaso_core::protocol::Response::GameData(Arc::new(GameData::ReSpin(response.clone())));
						round_tansactions.push(FuGaSoTuple {input, output: vec![output] });
						Some(round_tansactions)
					} else if action.name == ActionNameEnum::BonusSpinsStop {
						//start pars respin transactions
						continue;
					#[cfg(any())]
					{
						let response: SpinData<MegaThunderLinkInfo, ReSpinInfo> = SpinData::default();
						//end pars respin transactions
						let input = PlayerRequest::ReSpin;
						let output = fugaso_core::protocol::Response::GameData(Arc::new(GameData::ReSpin(response.clone())));
						round_tansactions.push(FuGaSoTuple {input, output: vec![output] });
						Some(round_tansactions)
					}
					} else if action.name == ActionNameEnum::Spin || action.name == ActionNameEnum::BuySpin {
						//start pars spin transactions
						let request = {
							let bet = action.params.bet_per_line.unwrap_or_default() as i32;
							let line = action.params.lines.unwrap_or_default() as usize;
							let denom = 10;
							let bet_index = action.params.selected_mode.map(|v| {v as usize}).unwrap_or(0);
							let bet_counter = bet_counters[bet_index];
							let reels = bet_index;
							fugaso_math::math::Request {bet, line, denom, bet_index, bet_counter, reels, }
						};
						let response: SpinData<MegaThunderLinkInfo, ReSpinInfo> = {
							let id = id::GAME_DATA;
							let balance = user.balance;
							let credit_type = 100;
							let curr_lines = context.spins.lines as usize;
							let curr_bet = context.spins.bet_per_line as i32;
							let curr_denom = 10;
							let curr_reels = 5;
							let category = 0;
							let round_id = 0;
							let round_type = RoundDetail::SIMPLE;
							let round_multiplier = 1;
							//result
							let stops = vec![0, 0, 0, 0, 0];
							let holds = vec![0];
							let grid0 = vec![];
							let (grid, overlay) = if let Some(board) = context.spins.original_board {
								let grid = convert_board(&board, &context.spins.bs_values);
								let overlay = Some(convert_board(&context.spins.board, &context.spins.bs_values));
								(grid, overlay)
							} else {
								let grid = convert_board(&context.spins.board, &context.spins.bs_values);
								let overlay = None;
								(grid, overlay)
							};
                            let gains = convert_win_lines(&context.spins.winlines.unwrap_or(vec![]));
							let (total, next_act, respins, accum, mults, lifts, lifts_new, grand) = if context.actions == vec![ActionNameEnum::BonusInit] {
                                let next_grand_lightning_out = serde_json::from_value::<GrandLightningOut>(iter.peek().expect("next packet not found").response.clone()).expect("next packet not found"); 
								let next_context = next_grand_lightning_out.context.map(|context| {context}).expect("next play context not impement");
								let next_bonus = next_context.bonus.map(|bonus| {bonus}).expect("next play respin bonus not impement");
								let total = context.spins.total_win.unwrap_or(0) + next_bonus.total_win;
								let respins = next_bonus.rounds_left as i32;
								let accum = next_bonus.total_win;
								let next_act = ActionKind::RESPIN;
								let (mut mults, mut lifts): (Vec<Vec<i32>>, Vec<Vec<i32>>) = context.spins.bs_values.iter().enumerate().map(|(col_num, col)| {
									col.iter().enumerate().map(|(row_num, &v)| {
										if context.spins.board[col_num][row_num] == COIN || context.spins.board[col_num][row_num] == JACKPOT {(v as i32, 1)} else {(0, 0)}
									}).collect()
								}).collect();
								let mut lifts_new = vec![];
								next_bonus.changes.iter().for_each(|change| {
										if change.symbol == MULTI {
											lifts.iter_mut().for_each(|lc| {
												lc.iter_mut().for_each(|l| {
													*l *= change.multiplier.unwrap_or(1) as i32;
												});
											});
											mults[change.reel as usize][change.row as usize] = change.value as i32;
											lifts[change.reel as usize][change.row as usize] = 1;
											lifts_new.push(LiftItem {
												p: (change.reel as usize, change.row as usize),
												m: change.multiplier.unwrap_or(1) as i32,
												v: change.value as i32
											});
										};
								});
								let grand = next_bonus.grand.iter().map(|&v| v as i32).collect::<Vec<i32>>();
								(total, next_act, respins, accum, mults, lifts, lifts_new, grand)
							} else {
								let total = context.spins.total_win.unwrap_or(0);
								let respins = 0;
								let accum = 0;
								let next_act = if context.spins.total_win.unwrap_or(0) > 0 {ActionKind::COLLECT} else {ActionKind::BET};
								let (mults, mut lifts): (Vec<Vec<i32>>, Vec<Vec<i32>>) = context.spins.bs_values.iter().enumerate().map(|(col_num, col)| {
									col.iter().enumerate().map(|(row_num, &v)| {
										if context.spins.board[col_num][row_num] == COIN || context.spins.board[col_num][row_num] == JACKPOT {(v as i32, 1)} else {(0, 0)}
									}).collect()
								}).collect();
								let mut lifts_new = vec![];
								context.spins.bs_values.iter().enumerate().for_each(|(col_num, col)| {
									col.iter().enumerate().for_each(|(row_num, &v)| {
										if context.spins.board[col_num][row_num] == MULTI {
											lifts.iter_mut().for_each(|lc| {
												lc.iter_mut().for_each(|l| {
													*l *= v as i32;
												});
											});
											lifts_new.push(LiftItem {
												p: (col_num, row_num),
												m: v as i32,
												v: (mults.iter().flat_map(|row| row.iter()).sum::<i32>() * (v as i32))
											});
										};
									})
								});
								let grand = vec![];
								(total, next_act, respins, accum, mults, lifts, lifts_new, grand)
							};
							SpinData { 
								id, 
								balance, 
								credit_type, 
								result: GameResult { 
									total, 
									stops, 
									holds, 
									cards: Default::default(), 
									grid0, 
									grid, 
									special: Some(MegaThunderLinkInfo { 
										total, 
										respins, 
										accum, 
										stop: Default::default(), 
										overlay, 
										mults,
										lifts,
										lifts_new,
										grand,
									}),
									gains, 
									restore: Some(ReSpinInfo { 
										total: Default::default(), 
										mults: Default::default(), 
										respins: Default::default(), 
										overlay: Default::default(), 
										accum: Default::default() 
									}), 
									extra_data: None
								}, 
								curr_lines, 
								curr_bet, 
								curr_denom, 
								curr_reels, 
								next_act, 
								category, 
								round_id, 
								round_type,
								round_multiplier, 
								promo: Promo { 
									amount: Default::default(), 
									multi: Default::default() 
								}, 
								free: None
							}
						};
						//end pars spin transactions
						let input = PlayerRequest::BetSpin(request.clone());
						let output = fugaso_core::protocol::Response::GameData(Arc::new(GameData::Spin(response.clone())));
						round_tansactions.push(FuGaSoTuple {input, output: vec![output] });
						Some(round_tansactions)
					} else {continue;}
					//end parse play transactions
				} else {
					//start parse other command transactions
					if grand_lightning_in.command == CommandEnum::Start {
						if game_settings.is_none() {game_settings = grand_lightning_out.settings.clone()}
					}
					continue;
					//end parse other command transactions
				}
				//end edit converter
			} else {None}
		} else {None};
		if let Some(converted_tr) = converted_tr_opt {results.extend(converted_tr)};
    };
    std::fs::create_dir_all(format!("packets_result/{GAME_FUGASO_FOLDER}")).unwrap();
    serde_json::to_writer(File::create(format!("packets_result/{GAME_FUGASO_FOLDER}/{name}")).expect("error file open"), &results,).expect("error write file");
}

fn convert_board(board: &Vec<Vec<i64>>, bs_values: &Vec<Vec<i64>>) -> Vec<Vec<char>> {
    board.iter().enumerate().map(|(col_num, col)| {
		col.iter().enumerate().map(|(row_num, row)| {
			if *row == JACKPOT {
				match bs_values[col_num][row_num] {
					MINI_VALUE => {MINI_CHAR},
					MINOR_VALUE => {MINOR_CHAR},
					MAJOR_VALUE => {MAJOR_CHAR}
					_ => {panic!("unknown jackpot")}
				}
			} else {
				char::from_u32(*row as u32 + '@' as u32).expect("error symbol")
			}
		}).collect::<Vec<_>>()
	}).collect::<Vec<_>>()
}

fn convert_win_lines(win_lines: &Vec<Winlines>) -> Vec<Gain> {
    let mut gains = win_lines.iter().map(|win_line| {
		Gain {
			symbol: char::from_u32(win_line.symbol as u32 + '@' as u32).expect("error symbol"),
			count: win_line.occurrences as usize,
			amount: win_line.amount,
			line_num: (win_line.line as usize) - 1,
			multi: 1,
			..Default::default()
		}
	}).collect::<Vec<_>>();
    gains.sort_by_key(|w| w.line_num);
    gains
}

#[derive(Debug, Serialize)]
pub struct Conf {
    pub lines: Vec<String>,
    pub wins: BTreeMap<char, BTreeMap<usize, i32>>,
}

#[tokio::test]
#[allow(unused)]
async fn test_config() {
    let symbols = vec!['^', '-', '_', 'V'];
    let mut json_str: Value = Default::default();
    let file = File::open(format!("packets/{GAME_SOURCE_NAME}/44fe51f0487c4118a5408a3c9c3af79b.json")).unwrap();
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).unwrap();
    let transactions = json.as_array().unwrap();
    for transaction in transactions {
        if transaction.get("out").and_then(|tr_out| tr_out.get("settings")).is_some() {
            json_str = transaction.get("out").unwrap().clone();
            break;
        }
    }
    let start = serde_json::from_value::<GrandLightningOut>(json_str);
    let cfg = match start {
        Ok(grand_lightning_out) => {
            let wins: BTreeMap<char, BTreeMap<usize, i32>> = grand_lightning_out.settings.clone().map(|settings| {
                settings.paytable.iter().fold(BTreeMap::new(), |mut acc, v| {
                    let number = v.0.parse::<u32>().expect("error parse number!");
                    let symbol = char::from_u32(number + '@' as u32).expect("error symbol");
                    if let Some(_vec) = acc.get_mut(&symbol) {panic!("error symbol already in map!")} else {
                        acc.insert(
                            symbol,
                            v.1.iter().map(|p| (p.occurrences as usize, p.multiplier as i32)).collect(),
                        );
                    }
                    acc
                })
            }).unwrap_or_default();
            let lines = grand_lightning_out.settings.clone().map(|settings| {
                settings.paylines.iter().map(|p| {
                    p.iter()
                        .map(|v| v)
                        .map(|v| symbols[*v as usize])
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join("")
                }).collect::<Vec<_>>()
            }).unwrap_or_default();
            Conf { wins, lines }
        }
       Err(e) => panic!("error config create!: {e}"),
    };
    println!("{}", serde_json::to_string(&cfg).expect("error cfg json"));
    std::fs::create_dir_all(format!("packets_result/{GAME_FUGASO_FOLDER}")).unwrap();
    serde_json::to_writer(File::create(format!("packets_result/{GAME_FUGASO_FOLDER}/config.json")).expect("error open file config"), &cfg,).expect("error write config");
}
