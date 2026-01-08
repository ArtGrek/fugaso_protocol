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
use fugaso_math::protocol::{id, GameData, GameResult, Promo, ReSpinInfo, SpinData, FreeGame, };
use fugaso_math::protocol_new_game_name::{OriginalGameNameIn, OriginalGameNameOut, NewGameNameLinkInfo, CommandEnum, Settings, };
const GAME_SOURCE_NAME: &str = "original_game_name";
const GAME_FUGASO_FOLDER: &str = "new_game_name_link";
pub const BOARD_HEIGHT: usize = 3;
pub const BOARD_WIDTH: usize = 5;

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
    let file = File::open(format!("packets/{GAME_SOURCE_NAME}/00-no_win.json")).unwrap();
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).unwrap();
    let transactions = json.as_array().unwrap();
    for transaction in transactions {
        if transaction.get("out").and_then(|context| context.get("paytable")).is_some() {
            json_str = transaction.get("out").unwrap().clone();
            break;
        }
    }
    let start = serde_json::from_value::<OriginalGameNameOut>(json_str);
    let cfg = match start {
        Ok(original_game_name_out) => {
            let wins: BTreeMap<char, BTreeMap<usize, i32>> = original_game_name_out.settings.clone().map(|settings| {
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
            let lines = original_game_name_out.settings.clone().map(|settings| {
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
        _ => panic!("error config create!"),
    };
    println!("{}", serde_json::to_string(&cfg).expect("error cfg json"));
    std::fs::create_dir_all(format!("packets_result/{GAME_FUGASO_FOLDER}")).unwrap();
    serde_json::to_writer(File::create(format!("packets_result/{GAME_FUGASO_FOLDER}/config.json")).expect("error open file config"), &cfg,).expect("error write config");
}

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

fn check_structure(name: &str) {
    for (idx, tuple) in parse_packet(name).into_iter().enumerate() {
        let val: Value = tuple.response.clone();
        let json_str = val.to_string();
        let mut deserializer = serde_json::Deserializer::from_str(&json_str);
        match deserialize::<_, Option<OriginalGameNameOut>>(&mut deserializer) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("[{name}] [ERROR]\n→ line {idx}\n→ body: {json_str}\n→ reason: {err}\n");
            }
        }
    }
}

fn convert(name: &str) {
	//start additional global variables
	let mut game_settings: Option<Settings> = None;
	//end additional global variables
    let list = parse_packet(name);
    let mut iter = list.into_iter().peekable();
    let mut results: Vec<FuGaSoTuple<NewGameNameLinkInfo, ReSpinInfo>> = Vec::new();
    while let Some(tuple) = iter.next() {let converted_tr_opt: Option<Vec<FuGaSoTuple<NewGameNameLinkInfo, ReSpinInfo>>> = 
		if let Ok(Some(original_game_name_out)) = serde_json::from_value::<Option<OriginalGameNameOut>>(tuple.response) {
			if let Ok(Some(original_game_name_in)) = serde_json::from_value::<Option<OriginalGameNameIn>>(tuple.request) {
				//start edit converter
				if original_game_name_in.command == CommandEnum::Play {
					//start parse play transactions

					let mut round_tansactions = Vec::new();
					if true {
						//start pars respin transactions

						let response: SpinData<NewGameNameLinkInfo, ReSpinInfo> = SpinData { 
							id: id::GAME_DATA, 
							balance: Default::default(), 
							credit_type: Default::default(), 
							result: GameResult { 
								total: Default::default(), 
								stops: Default::default(), 
								holds: Default::default(), 
								cards: Default::default(), 
								grid0: Default::default(), 
								grid: Default::default(), 
								special: Some(NewGameNameLinkInfo { 
									total: Default::default(), 
									respins: Default::default(), 
									accum: Default::default(), 
									stop: Default::default(), 
									overlay: Default::default()
								}),
								gains: Default::default(), 
								restore: Some(ReSpinInfo { 
									total: Default::default(), 
									multipliers: Default::default(), 
									respins: Default::default(), 
									overlay: Default::default(), 
									accum: Default::default() 
								}), 
								extra_data: Some(ReSpinInfo { 
									total: Default::default(), 
									multipliers: Default::default(), 
									respins: Default::default(), 
									overlay: Default::default(), 
									accum: Default::default() 
								})
							}, 
							curr_lines: Default::default(), 
							curr_bet: Default::default(), 
							curr_denom: Default::default(), 
							curr_reels: Default::default(), 
							next_act: ActionKind::BET, 
							category: Default::default(), 
							round_id: Default::default(), 
							round_type: RoundDetail::SIMPLE,
							round_multiplier: Default::default(), 
							promo: Promo { 
								amount: Default::default(), 
								multi: Default::default() 
							}, 
							free: Some(FreeGame { 
								total_win: Default::default(), 
								symbol: Default::default(), 
								category: Default::default(), 
								initial: Default::default(), 
								left: Default::default(), 
								done: Default::default() 
							}) 
						};
						//end pars respin transactions
						let input = PlayerRequest::ReSpin;
						let output = fugaso_core::protocol::Response::GameData(Arc::new(GameData::ReSpin(response.clone())));
						round_tansactions.push(FuGaSoTuple {input, output: vec![output] });
						Some(round_tansactions)
					} else {
						//start pars spin transactions

						let request = fugaso_math::math::Request {
							bet: Default::default(),
							line: Default::default(),
							denom: Default::default(),
							bet_index: Default::default(),
							bet_counter: Default::default(),
							reels: Default::default(),
						};
						let response: SpinData<NewGameNameLinkInfo, ReSpinInfo> = SpinData { 
							id: id::GAME_DATA, 
							balance: Default::default(), 
							credit_type: Default::default(), 
							result: GameResult { 
								total: Default::default(), 
								stops: Default::default(), 
								holds: Default::default(), 
								cards: Default::default(), 
								grid0: Default::default(), 
								grid: Default::default(), 
								special: Some(NewGameNameLinkInfo { 
									total: Default::default(), 
									respins: Default::default(), 
									accum: Default::default(), 
									stop: Default::default(), 
									overlay: Default::default()
								}),
								gains: Default::default(), 
								restore: Some(ReSpinInfo { 
									total: Default::default(), 
									multipliers: Default::default(), 
									respins: Default::default(), 
									overlay: Default::default(), 
									accum: Default::default() 
								}), 
								extra_data: Some(ReSpinInfo { 
									total: Default::default(), 
									multipliers: Default::default(), 
									respins: Default::default(), 
									overlay: Default::default(), 
									accum: Default::default() 
								})
							}, 
							curr_lines: Default::default(), 
							curr_bet: Default::default(), 
							curr_denom: Default::default(), 
							curr_reels: Default::default(), 
							next_act: ActionKind::BET, 
							category: Default::default(), 
							round_id: Default::default(), 
							round_type: RoundDetail::SIMPLE,
							round_multiplier: Default::default(), 
							promo: Promo { 
								amount: Default::default(), 
								multi: Default::default() 
							}, 
							free: Some(FreeGame { 
								total_win: Default::default(), 
								symbol: Default::default(), 
								category: Default::default(), 
								initial: Default::default(), 
								left: Default::default(), 
								done: Default::default() 
							}) 
						};
						//end pars spin transactions
						let input = PlayerRequest::BetSpin(request.clone());
						let output = fugaso_core::protocol::Response::GameData(Arc::new(GameData::Spin(response.clone())));
						round_tansactions.push(FuGaSoTuple {input, output: vec![output] });
						Some(round_tansactions)
					}
					//end parse play transactions
				} else {
					//start parse other command transactions
					if original_game_name_in.command == CommandEnum::Start {
						if game_settings.is_none() {game_settings = original_game_name_out.settings.clone()}
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

#[test]
//#[ignore]
#[allow(unused)]
fn test_convert() {
    convert("00-no_win.json");
    //convert("01-win.json");
    //convert("02-fs.json");
    //convert("03-fs_tree_train.json");
    //convert("04-collect_collect_multi.json");
    //convert("bet_200_line_1_c99f22c4e09b4390b9f77369fa0371f2.json");
    check_structure("00-no_win.json");
}
