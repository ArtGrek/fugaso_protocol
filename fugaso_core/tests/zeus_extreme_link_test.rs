use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{self as serde_de, Visitor};
use std::fmt;
use serde_json::Value;
use std::{fs::File, io::BufReader, sync::Arc,};

mod integration;
use integration::FuGaSoTuple;
use fugaso_core::protocol::PlayerRequest;
use fugaso_data::{fugaso_action::ActionKind, fugaso_round::RoundDetail};
use fugaso_math::protocol::{id, Gain, GameData, GameResult, Promo, ReSpinInfo, SpinData };
use fugaso_math::protocol_zeus::{ZeusExtremeLinkInfo, Step, Lift, ChangeItem};
const GAME_SOURCE_NAME: &str = "gladius_death_or_glory";
const GAME_FUGASO_FOLDER: &str = "zeus_extreme_link";
pub const BOARD_HEIGHT: usize = 5;
pub const BOARD_WIDTH: usize = 5;

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

fn convert_symbol_to_letter(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|ch| match ch {
					//empty cells
                    '\\' => 'A',
                    '^'  => 'B',
                    '['  => 'C',
                    ']'  => 'D',
					//collector empty
                    ')' => 'E',
					//collector values any
                    '*' => 'F',
					//coin values 5, 10, 25, 50
                    'G'  => 'G',
                    'H'  => 'H',
                    'I'  => 'I',
                    'J'  => 'J',
					//coin values 1, 2, 3, 4
                    '='  => 'K',
                    '>'  => 'L',
                    '?'  => 'M',
                    '@'  => 'N',
					//coin empty
                    '(' => 'O',
					//bonus
                    '3' => 'P',
					//coin values 100, 250, 500, 1000
                    'Q'  => 'Q',
                    'R'  => 'R',
                    'S'  => 'S',
                    'T'  => 'T',
					//multipliers 2, 3, 4, 5, 10, 20
                    '+'  => 'U',
                    ','  => 'V',
                    '-'  => 'W',
                    '.' => 'X',
                    '/' => 'Y',
                    '0' => 'Z',
                    _ => ch,
                })
                .collect()
        })
        .collect()
}

fn convert_symbol_to_multiplayer(grid: Vec<Vec<char>>) -> Vec<Vec<i32>> {
    grid.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|ch| match ch {
					//empty cells
                    '\\' => 0,
                    '^'  => 0,
                    '['  => 0,
                    ']'  => 0,
					//collector empty
                    ')' => 0,
					//collector values any
                    '*' => 0,
					//coin values 5, 10, 25, 50
                    'G'  => 5,
                    'H'  => 10,
                    'I'  => 25,
                    'J'  => 50,
					//coin values 1, 2, 3, 4
                    '='  => 1,
                    '>'  => 2,
                    '?'  => 3,
                    '@'  => 4,
					//coin empty
                    '(' => 0,
					//bonus
                    '3' => 0,
					//coin values 100, 250, 500, 1000
                    'Q'  => 100,
                    'R'  => 250,
                    'S'  => 500,
                    'T'  => 1000,
					//multipliers 2, 3, 4, 5, 10, 20
                    '+'  => 0,
                    ','  => 0,
                    '-'  => 0,
                    '.' => 0,
                    '/' => 0,
                    '0' => 0,
                    _ => 0,
                })
                .collect()
        })
        .collect()
}

fn parse_bet_info(json: &Value) -> (i32, usize, i32) {
	let denom: i32 = 10;
    let bet_counters = [1, 2, 100, 150];
    let mut total_bet = 0;
    let mut selected_mode = 0;
    if let Some(bet) = json.get("bets").and_then(|b| b.as_array()).and_then(|arr| arr.first()) {
        if let Some(bet_amount) = bet.get("betAmount").and_then(|v| v.as_str()) {total_bet = bet_amount.parse::<i32>().unwrap_or(0) / denom;}
        if let Some(buy_bonus) = bet.get("buyBonus").and_then(|v| v.as_str()) {
            selected_mode = match buy_bonus {
                "mod_bonus" => 1,
                "mod_shield" => 2,
                "fs" => 3,
                _ => 0,
            }
        }
    }
	let bet_counter = bet_counters[selected_mode];
    (total_bet, bet_counter, denom)
}

fn convert(name: &str) {
    let list = parse_list_game(name);
    let mut iter = list.into_iter().peekable();
    let mut results: Vec<FuGaSoTuple<ZeusExtremeLinkInfo, ReSpinInfo>> = Vec::new();
    while let Some(tuple) = iter.next() {
			let converted_tr: Option<Vec<FuGaSoTuple<ZeusExtremeLinkInfo, ReSpinInfo>>> = match serde_json::from_value::<Option<Gladius>>(tuple.response).inspect_err(|e| {eprintln!("err {e}");}).unwrap_or(None) {
				Some(Gladius { account_balance, round, .. }) => {
					if tuple.request.get("continueInstructions").is_none() {
						if let Some(round_context) = round.as_ref() {
							//input
							let (bet_amount, bet_counter, denomination, ) = parse_bet_info(&tuple.request);
							//request
							let request = fugaso_math::math::Request {
								bet: bet_amount,
								bet_counter,
								denom: denomination,
								..Default::default()
							};
							let input = PlayerRequest::BetSpin(request.clone());
							if round_context.events.last().and_then(|ev| Some(ev.etn.clone())) == Some(EventTypeEnum::FeatureExit) {
								let mut global_total = 0;
								//free spins out transactions
								let mut fs_tr = Vec::new();
								{	
									//output spin
									let grid0 = round_context.events.first().and_then(|ev| ev.c.grid.clone()).map(|s| {
										let trimmed = if s.len() > 2 { &s[2..] } else { "" };
										let chars: Vec<char> = trimmed.chars().collect();
										(0..BOARD_WIDTH).map(|c| {
												(0..BOARD_HEIGHT).map(|r| {
													chars[r * BOARD_HEIGHT + c]
												}).collect::<Vec<char>>()
											}).collect::<Vec<Vec<char>>>()
									}).unwrap_or_default();
									let stops = round_context.events.first().and_then(|ev| ev.c.stops.clone()).map(|vec| {
										vec.into_iter()
											.filter_map(|s| s.parse::<usize>().ok())
											.collect::<Vec<usize>>()
									}).unwrap_or_default();

									let mut gains = Vec::new();
									let mut steps = Vec::new();
									let mut spin_win_amount = 0;
									let mut temp_collects = vec![vec![0;5];5];
									for (_event_num, event) in round_context.events.clone().into_iter().skip(1).enumerate() {
										if matches!(event.etn, EventTypeEnum::CoinReveal(_)) {
											let grid = event.c.grid.map(|s| {
												let trimmed = if s.len() > 2 { &s[2..] } else { "" };
												let chars: Vec<char> = trimmed.chars().collect();
												(0..BOARD_WIDTH).map(|c| {
														(0..BOARD_HEIGHT).map(|r| {
															chars[r * BOARD_HEIGHT + c]
														}).collect::<Vec<char>>()
													}).collect::<Vec<Vec<char>>>()
											}).unwrap_or_default();

											let mut mults = convert_symbol_to_multiplayer(grid.clone());
											
											let mut collects = Vec::new();
											let mut lift_new = Vec::new();
											let mut mults1: Vec<Vec<i32>> = Vec::new();

											if let Some(ref actions) = event.c.actions {
												for act in actions {
													match act.at {
														ActionsAtEnum::Collector => {
															let x = act.data.p.clone().unwrap_or_default().parse::<usize>().ok().unwrap_or_default() % BOARD_WIDTH; 
															let y = act.data.p.clone().unwrap_or_default().parse::<usize>().ok().unwrap_or_default() / BOARD_WIDTH;
															let v = act.data.total.clone().unwrap_or_default().parse::<i32>().ok().unwrap_or_default() / 10;
															temp_collects[x][y] = v;
															collects.push(ChangeItem { p: (x, y), v: v });
															//mults[x][y] = v; ???
														}
														ActionsAtEnum::Multi => {
															let x = act.data.p.clone().unwrap_or_default().parse::<usize>().ok().unwrap_or_default() % BOARD_WIDTH; 
															let y = act.data.p.clone().unwrap_or_default().parse::<usize>().ok().unwrap_or_default() / BOARD_WIDTH;
															let m = act.data.m.clone().unwrap_or_default() as i32;
															lift_new.push(Lift { pos: (x, y), mult: m });

															
															let mut mults1_temp = vec![vec![0; BOARD_HEIGHT]; BOARD_WIDTH];
															if let Some(hits) = &act.data.hits {
																for (index, hit) in hits.iter().enumerate() {
																	let x = *hit as usize % BOARD_WIDTH; 
																	let y = *hit as usize / BOARD_WIDTH;
																	mults1_temp[x][y] = act.data.results
																		.as_ref()
																		.and_then(|results| results.get(index))
																		.map(|&val| (val / 10) as i32)
																		.unwrap_or_default();
																}
															}
															mults1 = mults1_temp;
														}
														_ => {}
													}
												}
											}


											for (x, col) in temp_collects.iter().enumerate() {
												for (y, row) in col.iter().enumerate() {
													if *row > 0 {mults[x][y] = *row}
												}
											}

											let gain_count =  event.c.actions.as_ref()
												.and_then(|acs| {acs.iter().find(|ac| matches!(ac.at, ActionsAtEnum::Collector | ActionsAtEnum::CashWin))})
												.and_then(|ac| ac.data.h.clone())
												.map(|s| { s.chars().filter(|&c| c == '1').count()}).unwrap_or_default();

											let gain_amount = event.c.actions.as_ref()
												.and_then(|acs| {acs.iter().find(|ac| matches!(ac.at, ActionsAtEnum::Collector | ActionsAtEnum::CashWin))})
												.map(|ac| (ac.data.clone(), ac.at.clone()))
												.and_then(|(data, at)| {
													match at {
														ActionsAtEnum::Collector => {
															let step_win = data.total.map(|s| s.parse::<i64>().ok().unwrap_or_default()).unwrap_or_default() * request.bet as i64;
															spin_win_amount += step_win;
															Some(step_win)
														}
														ActionsAtEnum::CashWin => {
															data.w.map(|s| {
																let step_win = s.parse::<i64>().ok().unwrap_or_default()/* * request.bet as i64*/; // if need multiply gain win amount to bet in spin running respin
																spin_win_amount += step_win;
																Some(step_win)
															}).unwrap_or_default()
														}
														_ => {Some(0)}
													}
												}).unwrap_or_default();

											let gain_symbol = event.c.actions.as_ref()
												.and_then(|acs| {acs.iter().find(|ac| matches!(ac.at, ActionsAtEnum::Collector | ActionsAtEnum::CashWin))})
												.map(|ac| {
													match ac.at {
														ActionsAtEnum::Collector => {'F'}
														_ => {'K'}
													}
												}).unwrap_or_default(); 

											let gain = Gain { 
												count: gain_count, 
												amount: gain_amount, 
												symbol: gain_symbol,
												multi: 1,
												..Default::default()
											};
											gains = vec![gain];
											steps.push(Step { grid: convert_symbol_to_letter(grid), collects, lift_new, mults, mults1 });
										}
									}
									global_total += spin_win_amount;
									let next_act = ActionKind::RESPIN;
									let holds = Vec::new();
									let reelset_number = 0;	
									let (grid0, stops, gains, link) = 
										(grid0, stops, gains, ZeusExtremeLinkInfo {total: global_total, respins: 3, accum: 0, steps, ..Default::default()});
									//response
									let response: SpinData<ZeusExtremeLinkInfo, ReSpinInfo> = SpinData {
										id: id::GAME_DATA,
										balance: account_balance.as_ref().map(|a| a.balance).unwrap_or(0) as i64,
										credit_type: 100,
										result: GameResult {
											total: global_total,
											stops,
											holds,
											grid: convert_symbol_to_letter(grid0),
											special: Some(link),
											gains,
											..Default::default()
										},
										curr_lines: 1,
										curr_bet: request.bet,
										curr_denom: request.denom,
										curr_reels: 5,
										next_act,
										category: reelset_number,
										round_id: 0,
										round_type: RoundDetail::SIMPLE,
										round_multiplier: 1,
										promo: Promo {
											amount: 0,
											multi: 0,
										},
										..Default::default()
									};
									let output = fugaso_core::protocol::Response::GameData(Arc::new(GameData::Spin(response)));
									fs_tr.push(FuGaSoTuple {input,output: vec![output]});
								}

								let mut global_accum = 0;
								{
									//output respins
									let mut events = round_context.events.clone().into_iter().peekable();
									let mut respins_count = 3;
									let mut coin_count = 0;
									let mut event_num = 0;
									while let Some(event) = events.next() {
										if event.etn == EventTypeEnum::FsReveal {
											let mut next_act = ActionKind::RESPIN;
											respins_count -= 1;
											let temp_coin_count = event.c.grid.as_deref().map(|s| s.chars().filter(|&c| c == '(').count()).unwrap_or(0);
											if temp_coin_count > coin_count {respins_count = 3;}
											coin_count = temp_coin_count;
											
											let grid0 = event.c.grid.clone().map(|s| {
												let trimmed = if s.len() > 2 { &s[2..] } else { "" };
												let chars: Vec<char> = trimmed.chars().collect();
												(0..BOARD_WIDTH).map(|c| {
														(0..BOARD_HEIGHT).map(|r| {
															chars[r * BOARD_HEIGHT + c]
														}).collect::<Vec<char>>()
													}).collect::<Vec<Vec<char>>>()
											}).unwrap_or_default();
											let stops = event.c.stops.clone().map(|vec| {
												vec.into_iter()
													.filter_map(|s| s.parse::<usize>().ok())
													.collect::<Vec<usize>>()
											}).unwrap_or_default();
											let mut gains = Vec::new();
											let mut steps = Vec::new();
											let mut respin_win_amount = 0;
											let mut temp_collects = vec![vec![0;5];5];

											let mut next_event: Events;
											if let Some(first_next_event) = events.peek() {
												next_event = first_next_event.clone();
												let mut step_count = 1;
												while matches!(next_event.etn, EventTypeEnum::FsCoinReveal(_)) {
													
													let grid = next_event.c.grid.clone().map(|s| {
														let trimmed = if s.len() > 2 { &s[2..] } else { "" };
														let chars: Vec<char> = trimmed.chars().collect();
														(0..BOARD_WIDTH).map(|c| {
																(0..BOARD_HEIGHT).map(|r| {
																	chars[r * BOARD_HEIGHT + c]
																}).collect::<Vec<char>>()
															}).collect::<Vec<Vec<char>>>()
													}).unwrap_or_default();
													let mut mults = convert_symbol_to_multiplayer(grid.clone());
													let mut collects = Vec::new();
													let mut lift_new = Vec::new();
													let mut mults1: Vec<Vec<i32>> = Vec::new();
													if let Some(ref actions) = next_event.c.actions {
														for act in actions {
															match act.at {
																ActionsAtEnum::Collector => {
																	let x = act.data.p.clone().unwrap_or_default().parse::<usize>().ok().unwrap_or_default() % BOARD_WIDTH; 
																	let y = act.data.p.clone().unwrap_or_default().parse::<usize>().ok().unwrap_or_default() / BOARD_WIDTH;
																	let v = act.data.total.clone().unwrap_or_default().parse::<i32>().ok().unwrap_or_default() / 10;
																	temp_collects[x][y] = v;
																	collects.push(ChangeItem { p: (x, y), v: v });
																	//mults[x][y] = v; ???
																}
																ActionsAtEnum::Multi => {
																	let x = act.data.p.clone().unwrap_or_default().parse::<usize>().ok().unwrap_or_default() % BOARD_WIDTH; 
																	let y = act.data.p.clone().unwrap_or_default().parse::<usize>().ok().unwrap_or_default() / BOARD_WIDTH;
																	let m = act.data.m.clone().unwrap_or_default() as i32;
																	lift_new.push(Lift { pos: (x, y), mult: m });

																	
																	let mut mults1_temp = vec![vec![0; BOARD_HEIGHT]; BOARD_WIDTH];
																	if let Some(hits) = &act.data.hits {
																		for (index, hit) in hits.iter().enumerate() {
																			let x = *hit as usize % BOARD_WIDTH; 
																			let y = *hit as usize / BOARD_WIDTH;
																			mults1_temp[x][y] = act.data.results
																				.as_ref()
																				.and_then(|results| results.get(index))
																				.map(|&val| (val / 10) as i32)
																				.unwrap_or_default();
																		}
																	}
																	mults1 = mults1_temp;
																}
																_ => {}
															}
														}
													}
													for (x, col) in temp_collects.iter().enumerate() {
														for (y, row) in col.iter().enumerate() {
															if *row > 0 {mults[x][y] = *row}
														}
													}
													let gain_count =  next_event.c.actions.as_ref()
														.and_then(|acs| acs.last())
														.and_then(|ac| ac.data.h.clone())
														.map(|s| { s.chars().filter(|&c| c == '1').count()}).unwrap_or_default();
													let gain_amount = next_event.c.actions.as_ref()
														.and_then(|acs| acs.last())
														.map(|ac| (ac.data.clone(), ac.at.clone()))
														.and_then(|(data, at)| {
															match at {
																/*ActionsAtEnum::Collector => {
																	let step_win = data.total.map(|s| {s.parse::<i64>().ok().unwrap_or_default()}).unwrap_or_default() * request.bet as i64;
																	respin_win_amount += step_win;
																	Some(step_win)
																}*/
																ActionsAtEnum::CashWin => {
																	let step_win = data.w.map(|s| {s.parse::<i64>().ok().unwrap_or_default()}).unwrap_or_default()/* * request.bet as i64*/; // if need multiply gain win amount to bet in respin 
																	respin_win_amount += step_win;
																	Some(step_win)
																}
																_ => {Some(0)}
															}
														}).unwrap_or_default();
													let gain_symbol = next_event.c.actions.as_ref()
														.and_then(|acs| acs.last())
														.map(|ac| {
															match ac.at {
																ActionsAtEnum::Collector => {'F'}
																_ => {'K'}
															}
														}).unwrap_or_default(); 
													let gain = Gain { 
														count: gain_count, 
														amount: gain_amount, 
														symbol: gain_symbol,
														multi: 1,
														..Default::default()
													};
													gains = vec![gain];
													steps.push(Step { grid: convert_symbol_to_letter(grid), collects, lift_new, mults, mults1 });
													
													let mut respin_steps_events = round_context.events.clone().into_iter().peekable();
													for _ in 0..event_num {respin_steps_events.next();}
													for _ in 0..(step_count + 1) {respin_steps_events.next();}
													step_count += 1;
													if let Some(next_next_event) = respin_steps_events.peek() {next_event = next_next_event.clone();}
													if next_event.etn == EventTypeEnum::FeatureExit {next_act = ActionKind::COLLECT;};
												}
											}
											
											global_total += respin_win_amount;
											global_accum += respin_win_amount;
											let holds = Vec::new();
											let reelset_number = 0;	
											let (grid0, stops, gains, link) = 
												(grid0, stops, gains, ZeusExtremeLinkInfo {total: global_total, respins: respins_count, accum: global_accum, steps, ..Default::default()});
											//response
											let response: SpinData<ZeusExtremeLinkInfo, ReSpinInfo> = SpinData {
												id: id::GAME_DATA,
												balance: account_balance.as_ref().map(|a| a.balance).unwrap_or(0) as i64,
												credit_type: 100,
												result: GameResult {
													total: global_total,
													stops,
													holds,
													grid: convert_symbol_to_letter(grid0),
													special: Some(link),
													gains,
													..Default::default()
												},
												curr_lines: 1,
												curr_bet: request.bet,
												curr_denom: request.denom,
												curr_reels: 5,
												next_act,
												category: reelset_number,
												round_id: 0,
												round_type: RoundDetail::SIMPLE,
												round_multiplier: 1,
												promo: Promo {
													amount: 0,
													multi: 0,
												},
												..Default::default()
											};

											let output = fugaso_core::protocol::Response::GameData(Arc::new(GameData::ReSpin(response.clone())));
											fs_tr.push(FuGaSoTuple {input: PlayerRequest::ReSpin,output: vec![output] });
										};
										event_num += 1;
									};
								}

								Some(fs_tr)
							} else {
								//output spins transactions
								let grid0 = round_context.events.first().and_then(|ev| ev.c.grid.clone()).map(|s| {
									let trimmed = if s.len() > 2 { &s[2..] } else { "" };
									let chars: Vec<char> = trimmed.chars().collect();
									(0..BOARD_WIDTH).map(|c| {
											(0..BOARD_HEIGHT).map(|r| {
												chars[r * BOARD_HEIGHT + c]
											}).collect::<Vec<char>>()
										}).collect::<Vec<Vec<char>>>()
								}).unwrap_or_default();
								let stops = round_context.events.first().and_then(|ev| ev.c.stops.clone()).map(|vec| {
									vec.into_iter()
										.filter_map(|s| s.parse::<usize>().ok())
										.collect::<Vec<usize>>()
								}).unwrap_or_default();
								let mut gains = Vec::new();
								let mut steps = Vec::new();
								let mut event_collect_amount = 0;
								let mut temp_collects = vec![vec![0;5];5];
								for (_event_num, event) in round_context.events.clone().into_iter().skip(1).enumerate() {

									let grid = event.c.grid.map(|s| {
										let trimmed = if s.len() > 2 { &s[2..] } else { "" };
										let chars: Vec<char> = trimmed.chars().collect();
										(0..BOARD_WIDTH).map(|c| {
												(0..BOARD_HEIGHT).map(|r| {
													chars[r * BOARD_HEIGHT + c]
												}).collect::<Vec<char>>()
											}).collect::<Vec<Vec<char>>>()
									}).unwrap_or_default();

									let mut mults = convert_symbol_to_multiplayer(grid.clone());
									
									let mut collects = Vec::new();
									let mut lift_new = Vec::new();
									let mut mults1: Vec<Vec<i32>> = Vec::new();

									if let Some(ref actions) = event.c.actions {
										for act in actions {
											match act.at {
												ActionsAtEnum::Collector => {
													let x = act.data.p.clone().unwrap_or_default().parse::<usize>().ok().unwrap_or_default() % BOARD_WIDTH; 
													let y = act.data.p.clone().unwrap_or_default().parse::<usize>().ok().unwrap_or_default() / BOARD_WIDTH;
													let v = act.data.total.clone().unwrap_or_default().parse::<i32>().ok().unwrap_or_default() / 10;
													temp_collects[x][y] = v;
													collects.push(ChangeItem { p: (x, y), v: v });
													//mults[x][y] = v; ???
												}
												ActionsAtEnum::Multi => {
													let x = act.data.p.clone().unwrap_or_default().parse::<usize>().ok().unwrap_or_default() % BOARD_WIDTH; 
													let y = act.data.p.clone().unwrap_or_default().parse::<usize>().ok().unwrap_or_default() / BOARD_WIDTH;
													let m = act.data.m.clone().unwrap_or_default() as i32;
													lift_new.push(Lift { pos: (x, y), mult: m });

													
													let mut mults1_temp = vec![vec![0; BOARD_HEIGHT]; BOARD_WIDTH];
													if let Some(hits) = &act.data.hits {
														for (index, hit) in hits.iter().enumerate() {
															let x = *hit as usize % BOARD_WIDTH; 
															let y = *hit as usize / BOARD_WIDTH;
															mults1_temp[x][y] = act.data.results
																.as_ref()
																.and_then(|results| results.get(index))
																.map(|&val| (val / 10) as i32)
																.unwrap_or_default();
														}
													}
													mults1 = mults1_temp;
												}
												_ => {}
											}
										}
									}

									for (x, col) in temp_collects.iter().enumerate() {
										for (y, row) in col.iter().enumerate() {
											if *row > 0 {mults[x][y] = *row}
										}
									}


									let gain_count =  event.c.actions.as_ref()
										.and_then(|acs| acs.last())
										.and_then(|ac| ac.data.h.clone())
										.map(|s| { s.chars().filter(|&c| c == '1').count()}).unwrap_or_default();

									let gain_amount = event.c.actions.as_ref()
										.and_then(|acs| acs.last())
										.map(|ac| (ac.data.clone(), ac.at.clone()))
										.and_then(|(data, at)| {
											match at {
												ActionsAtEnum::Collector => {
													let step_win = data.total.map(|s| s.parse::<i64>().ok().unwrap_or_default()).unwrap_or_default() * request.bet as i64;
													event_collect_amount += step_win;
													Some(step_win)
												}
												ActionsAtEnum::CashWin => {
													data.w.map(|s| {
														let step_win = s.parse::<i64>().ok().unwrap_or_default()/* * request.bet as i64*/; // if need multiply gain win amount to bet in spin
														Some(step_win)
													}).unwrap_or_default()
												}
												_ => {Some(0)}
											}
										}).unwrap_or_default();

									let gain_symbol = event.c.actions.as_ref()
										.and_then(|acs| acs.last())
										.map(|ac| {
											match ac.at {
												ActionsAtEnum::Collector => {'F'}
												ActionsAtEnum::CashWin => {'K'}
												_ => {'Z'}
											}
										}).unwrap_or_default(); 

									let gain = Gain { 
										count: gain_count, 
										amount: gain_amount, 
										symbol: gain_symbol,
										multi: 1,
										..Default::default()
									};
									gains = vec![gain];
									steps.push(Step { grid: convert_symbol_to_letter(grid), collects, lift_new, mults, mults1 });
								}
								

								
								let total = round_context.events.last().and_then(|ev| ev.awa.parse::<i64>().ok()).unwrap_or(0);
								let next_act = if total > 0 {ActionKind::COLLECT} else {ActionKind::BET};
								let holds = Vec::new();
								let reelset_number = 0;	
								let (stops, gains, grid0, link) = 
									(stops, gains, grid0, ZeusExtremeLinkInfo {total, respins: 0, accum: 0, steps,..Default::default() });
								//response
								let response: SpinData<ZeusExtremeLinkInfo, ReSpinInfo> = SpinData {
									id: id::GAME_DATA,
									balance: account_balance.as_ref().map(|a| a.balance).unwrap_or(0) as i64,
									credit_type: 100,
									result: GameResult {
										total,
										stops,
										holds,
										grid: convert_symbol_to_letter(grid0),
										special: Some(link),
										gains,
										..Default::default()
									},
									curr_lines: 1,
									curr_bet: request.bet,
									curr_denom: request.denom,
									curr_reels: 5,
									next_act,
									category: reelset_number,
									round_id: 0,
									round_type: RoundDetail::SIMPLE,
									round_multiplier: 1,
									promo: Promo {
										amount: 0,
										multi: 0,
									},
									..Default::default()
								};
								let output = fugaso_core::protocol::Response::GameData(Arc::new(GameData::Spin(response)));

								Some(vec![FuGaSoTuple {input,output: vec![output]}])
							}
						} else {None}
					} else {None}
				}
				_ => None,
			};
			if let Some(converted_trz) = converted_tr {results.extend(converted_trz)};
    }
    std::fs::create_dir_all(format!("packets_result/{GAME_FUGASO_FOLDER}")).unwrap();
    serde_json::to_writer(File::create(format!("packets_result/{GAME_FUGASO_FOLDER}/{name}")).expect("error file open"), &results,).expect("error write file");
}

#[test]
//#[ignore]
#[allow(unused)]
fn test_convert() {
    //convert("00-no_win.json");
    //convert("01-win.json");
    convert("02-fs.json");
    convert("04-collect_collect_multi.json");
    //convert("05-buy_mod_shield_bet_200_line_1.json");
}

pub mod string_u64 {
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(val: &u64, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(&val.to_string())
    }

    pub fn deserialize<'de, D>(d: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(d)?;
        s.parse::<u64>().map_err(serde::de::Error::custom)
    }
}

mod de {
    use serde::{Deserialize, Deserializer};
    use serde::de::Error;

    pub fn opt_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Option::<serde_json::Value>::deserialize(deserializer)?;
        Ok(match v {
            Some(serde_json::Value::String(s)) => Some(s.parse::<u64>().map_err(D::Error::custom)?),
            Some(serde_json::Value::Number(n)) => n.as_u64(),
            Some(_) => return Err(D::Error::custom("expected string or number")),
            None => None,
        })
    }

    pub fn opt_vec_u64<'de, D>(deserializer: D) -> Result<Option<Vec<u64>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Option::<Vec<serde_json::Value>>::deserialize(deserializer)?;
        Ok(v.map(|arr| {
            arr.into_iter()
                .map(|item| match item {
                    serde_json::Value::String(s) => s.parse::<u64>().map_err(D::Error::custom),
                    serde_json::Value::Number(n) => n.as_u64().ok_or_else(|| D::Error::custom("invalid number")),
                    _ => Err(D::Error::custom("expected string or number")),
                }).collect::<Result<Vec<_>, _>>().unwrap()
        }))
    }
}

use strum_macros::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Display, PartialEq)]
pub enum ContinueInstructionsActionsEnum {
	#[serde(rename = "win_presentation_complete")]
	WinPresentationComplete,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum GeoEnum {
	#[default]
	#[serde(rename = "AM-ER")]
	AmEr,
	#[serde(rename = "AU-NSW")]
	AuNsw,
	#[serde(rename = "AZ-BA")]
	AzBa,
	#[serde(rename = "DE-SN")]
	DeSn,
	#[serde(rename = "GE-TB")]
	GeTb,
	#[serde(rename = "GR-B")]
	GrB,
	#[serde(rename = "GR-I")]
	GrI,
	#[serde(rename = "RU-STA")]
	RuSta,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum RoundStatusEnum {
	#[default]
	#[serde(rename = "completed")]
	Completed,
	#[serde(rename = "wfwpc")]
	Wfwpc,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum BonusIdEnum {
	#[default]
	#[serde(rename = "default")]
	Default,
	#[serde(rename = "mod_bonus")]
	Bonus,
	#[serde(rename = "mod_shield")]
	Shield,
	#[serde(rename = "fs")]
	Fs,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default, PartialEq)]
pub enum ActionsAtEnum {
	#[default]
	#[serde(rename = "visualReels")]
	VisualReels,
	#[serde(rename = "bonusfeaturewon")]
	BonusFeatureWon,
	#[serde(rename = "cashWin")]
	CashWin,
	#[serde(rename = "collector")]
	Collector,
	#[serde(rename = "gridwin")]
	GridWin,
	#[serde(rename = "multi")]
	Multi,
	#[serde(rename = "refill")]
	Refill,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum DataRsEnum {
	#[default]
	#[serde(rename = "visual_base")]
	VisualBase,
	#[serde(rename = "visual_no_fs")]
	VisualNoFs,
}


#[derive(Debug, Clone, Default, PartialEq)]
pub enum EventTypeEnum {
	#[default]
    Reveal,
    CoinReveal(u8),
    FeatureEnter,
    FsReveal,
    FsCoinReveal(u8),
    FeatureExit,
}

impl Serialize for EventTypeEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            EventTypeEnum::Reveal => serializer.serialize_str("reveal"),
            EventTypeEnum::CoinReveal(n) => serializer.serialize_str(&format!("coin_reveal_{}", n)),
            EventTypeEnum::FeatureEnter => serializer.serialize_str("feature_enter"),
            EventTypeEnum::FsReveal => serializer.serialize_str("fs_reveal"),
            EventTypeEnum::FsCoinReveal(n) => serializer.serialize_str(&format!("fs_coin_reveal_{}", n)),
            EventTypeEnum::FeatureExit => serializer.serialize_str("feature_exit"),
        }
    }
}

struct EventVisitor;

impl<'de> Visitor<'de> for EventVisitor {
    type Value = EventTypeEnum;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("string like 'reveal', 'coin_reveal_3', 'fs_coin_reveal_2', 'feature_enter', 'feature_exit'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde_de::Error,
    {
        match v {
            "reveal" => Ok(EventTypeEnum::Reveal),
            "fs_reveal" => Ok(EventTypeEnum::FsReveal),
            "feature_enter" => Ok(EventTypeEnum::FeatureEnter),
            "feature_exit" => Ok(EventTypeEnum::FeatureExit),
            _ => {
                if let Some(num) = v.strip_prefix("coin_reveal_") {
                    return num.parse::<u8>()
                        .map(EventTypeEnum::CoinReveal)
                        .map_err(E::custom);
                }
                if let Some(num) = v.strip_prefix("fs_coin_reveal_") {
                    return num.parse::<u8>()
                        .map(EventTypeEnum::FsCoinReveal)
                        .map_err(E::custom);
                }
                Err(E::custom(format!("invalid value: {}", v)))
            }
        }
    }
}

impl<'de> Deserialize<'de> for EventTypeEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(EventVisitor)
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MultiValue {
    Number(u32),
    Text(String),
}

impl Default for MultiValue {
    fn default() -> Self {
        MultiValue::Text("desktop".to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Accountbalance {
	#[serde(with = "string_u64")]
	pub balance: u64 /* 400, 1800, 7200 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus_balance: Option<String>,
	pub currency_code: String /* EUR */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub real_balance: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Bonusgames {
	pub bet_cost_multiplier: String /* 2, 100, 150 */,
	pub bonus_game_id: BonusIdEnum /* fs, mod_bonus, mod_shield */,
	pub expected_rtp: f64 /* 96.28, 96.31, 96.34 */,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Data {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bfc: Option<String> /* 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bfw: Option<String> /* fs */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub count: Option<String> /* 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub h: Option<String> /* 0000010000000000000000000, 0000010000000000000000001, 0000010000000000000000010, 0000010000000000000001000 */,
	#[serde(skip_serializing_if = "Option::is_none", default, deserialize_with = "de::opt_vec_u64")]
	pub hits: Option<Vec<u64>> /* 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24 */,
	#[serde(skip_serializing_if = "Option::is_none", default, deserialize_with = "de::opt_u64")]
	pub m: Option<u64> /* 2, 3, 4, 5, 10, 20 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mask: Option<String> /* 0000010000000000000100010, 0000010000000000001000001, 0000010000000000010001000, 0000010000000001001000000 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub p: Option<String> /* 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24 */,
	#[serde(skip_serializing_if = "Option::is_none", default, deserialize_with = "de::opt_vec_u64")]
	pub results: Option<Vec<u64>> /* 20, 30, 40, 50, 60, 80, 90, 100, 120, 150, 160, 180, 200, 220, 240, 250, 260, 270, 280, 300 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rs: Option<DataRsEnum> /* visual_base, visual_no_fs */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub symbol: Option<String> /* 11 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub total: Option<String> /* 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 240, 260 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub w: Option<String> /* 0, 200, 400, 600, 800, 1000, 1200, 1400, 1600, 1800, 2000, 2200, 2400, 2600, 2800, 3000, 3200, 3400, 3600, 3800 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub win_amount: Option<String> /* 0 */,
}

impl Default for Data {
	fn default() -> Self {
		Self { 
			bfc: Option::default(), 
			bfw: Option::default(), 
			count: Option::default(), 
			h: Option::default(), 
			hits: Option::default(), 
			m: Option::default(), 
			mask: Option::default(), 
			p: Option::default(), 
			results: Option::default(),  
			rs: Some(DataRsEnum::default()), 
			symbol: Option::default(), 
			total: Option::default(), 
			w: Option::default(), 
			win_amount: Option::default(), 
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Actions {
	pub at: ActionsAtEnum /* bonusfeaturewon, cashWin, collector, gridwin, multi, refill, visualReels */,
	pub data: Data,
}

impl Default for Actions {
	fn default() -> Self {
		Self { 
			at: ActionsAtEnum::default(), 
			data: Data::default(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Context {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub actions: Option<Vec<Actions>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus_feature_count: Option<String> /* 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus_feature_won: Option<String> /* fs */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub grid: Option<String> /* , +\[[], --, --(((((((((((((((((((((((((, --(((((((((((((((((\(((((((, --(((((((([(][([([(^]]((]^(, --(((((((([(^](\(^(\][(([\(, --((((((((\([\(](\(\\]((][(, --((((((((\(\\([([(\[]((\\(, --((((((([(([((((((^(((((((, --((((((([](]((((((](((((((, --(((((((](((((((((^(((((((, --(((((((^(((((((((\((((((( */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reel_set: Option<BonusIdEnum> /* default, fs */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stops: Option<Vec<String>> /* 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42 */,
}

impl Default for Context {
	fn default() -> Self {
		Self { 
			actions: Some(vec![Actions::default()]), 
			bonus_feature_count: Option::default(), 
			bonus_feature_won: Option::default(), 
			grid: Option::default(), 
			reel_set: Some(BonusIdEnum::default()), 
			stops: Option::default(), 
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Events {
	pub awa: String /* 0, 200, 400, 600, 800, 1000, 1200, 1400, 1600, 1800, 2000, 2200, 2400, 2600, 2800, 3000, 3200 */,
	pub awc: String /* 0 */,
	pub ba: String /* 0 */,
	pub bc: String /* 0 */,
	pub c: Context,
	pub en: String /* 0 */,
	pub et: u32 /* 2 */,
	pub etn: EventTypeEnum /* coin_reveal_0, coin_reveal_1, coin_reveal_2, coin_reveal_3, feature_enter, feature_exit, fs_coin_reveal_0, fs_coin_reveal_1, fs_coin_reveal_2, fs_coin_reveal_3, fs_reveal, reveal */,
	pub wa: String /* 0, 200, 400, 600, 800, 1000, 1200, 1400, 1600, 1800, 2000, 2200, 2400, 2600, 2800, 3000, 3200 */,
	pub wc: String /* 0 */,
}

impl Default for Events {
	fn default() -> Self {
		Self {
			awa: "0".to_string(),
			awc: "0".to_string(),
			ba: "0".to_string(),
			bc: "0".to_string(),
			c: Context::default(),
			en: "0".to_string(),
			et: 2,
			etn: Default::default(),
			wa: "0".to_string(),
			wc: "0".to_string(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Round {
	pub events: Vec<Events>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jackpot_win: Option<String>,
	pub possible_actions: Vec<ActionsAtEnum> /*  */,
	pub round_id: String /* 9334310, 9334614, 9334887 */,
	pub status: RoundStatusEnum /* completed, wfwpc */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Gladius {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub account_balance: Option<Accountbalance>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub auto_collect_after: Option<String> /* 86400 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub auto_play_alternatives: Option<Vec<String>> /* 10, 25, 50, 75, 100, 500, 1000 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub autoplay_disabled: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub autoplay_loss_limit_required: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub autoplay_win_limit_required: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub available_mission: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub available_mystery_prize: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub available_tournament: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub backend_game_version: Option<String> /* 1.0.1 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_levels: Option<Vec<String>> /* 10, 20, 40, 60, 80, 100, 120, 140, 160, 180, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1500, 2000, 2500, 3000, 3500, 4000, 4500, 5000 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus_games: Option<Vec<Bonusgames>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cheats_enabled: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub clear_old_round_immediately_on_new_round: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_data: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_bet_level: Option<String> /* 200 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dialog: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub disable_bet_when_screens_are_open: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub disable_external_links: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub disable_keybinds: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub disable_mid_round_full_screen_menus: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub disable_round_history: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub disable_win_history: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub display_game_info_rtp_range: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub display_max_win_multiplier: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub display_max_win_odds: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub display_net_position: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub display_payout_table_as_multipliers: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub display_payout_table_on_game_launch: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub display_rtp: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub display_session_timer: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub events: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub free_round_offer: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub game_id: Option<String> /* 1807 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub game_state: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub geo: Option<GeoEnum> /* AM-ER, AU-NSW, AZ-BA, GE-TB, GR-I, RU-STA */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hide_game_info_date: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hide_game_info_interrupted: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hide_game_info_rtp: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jurisdiction: Option<String> /* curacao */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub keep_alive_interval: Option<String> /* 300 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language_code: Option<String> /* en-us */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_exposure: Option<String> /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_feature_cost: Option<String> /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_feature_spin_cost: Option<String> /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub minimum_round_duration: Option<String> /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String> /* Demo */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub offer: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub offline_promotion_wins: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parallel_rounds_support_disabled: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub partner_id: Option<String> /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pending_win: Option<String> /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub player_id: Option<String> /* Demo */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub progression_data: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub promotion_no_longer_available: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub promotion_win: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub remember_bet_level: Option<bool> /* true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub replay_link_disabled: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rm: Option<String> /* 96 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rollback_after: Option<String> /* 86400 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub round: Option<Round>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub round_id: Option<String> /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub round_in_progress_currency: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub round_status: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub server_time: Option<String> /* 2025-09-01T12:22:35Z, 2025-09-01T12:22:43Z, 2025-09-01T12:22:44Z */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub server_version: Option<String> /* 2.0.235 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session_rescue_enabled: Option<bool> /* true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session_timeout_seconds: Option<String> /* 1800 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session_uuid: Option<String> /* 05ef294a-a256-400d-bdf1-38ab44873395 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slam_stop_disabled: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub spacebar_disabled: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String> /* error */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status_code: Option<u32> /* 0, 10, 21 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status_data: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status_message: Option<String> /* , Demo mode limit reached!, Session timeout! */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stop_autoplay_on_feature_win: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub super_turbo_disabled: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub turbo_disabled: Option<bool> /* false */,
}

impl Gladius {
	pub fn new() -> Self {
		Self::default()
	}
    pub fn set(&mut self, code: u32, message: String, ) {
        self.status_code = Some(code);
        self.status_message = Some(message);
    
    }
}

