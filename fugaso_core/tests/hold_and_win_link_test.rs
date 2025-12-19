use serde::{Serialize, Deserialize, };
use serde_json::Value;
use serde_path_to_error::deserialize;
use std::{fs::File, io::BufReader, sync::Arc,};
use std::collections::{HashMap, BTreeMap, };
use strum_macros::Display;

mod integration;
use integration::FuGaSoTuple;
use fugaso_core::protocol::PlayerRequest;
use fugaso_data::{fugaso_action::ActionKind, fugaso_round::RoundDetail};
use fugaso_math::protocol::{id, Gain, GameData, GameResult, Promo, ReSpinInfo, SpinData, };
use fugaso_math::protocol_hold_and_win::ThunderExpressInfo;
const GAME_SOURCE_NAME: &str = "source_hold_and_win";
const GAME_FUGASO_FOLDER: &str = "thunder_express";
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
        if transaction.get("out")
		.and_then(|context| context.get("result"))
		.and_then(|result| result.get("game"))
		.and_then(|game| game.get("settings")).is_some() {
            json_str = transaction.get("out").unwrap().clone();
            break;
        }
    }
    let start: SourceHoldAndWin = serde_json::from_value(json_str).expect("error parsing start!");
    let cfg = match start {
        SourceHoldAndWin { result, .. } => {
			let payout = result.unwrap().game.settings.unwrap().payout.clone();
			let tiles = payout.tiles.clone();
			let mut wins: BTreeMap<char, BTreeMap<usize, i32>> = BTreeMap::new();
			let normals = vec![
				tiles.normal.normal_1,
				tiles.normal.normal_2,
				tiles.normal.normal_3,
				tiles.normal.normal_4,
				tiles.normal.normal_5,
				tiles.normal.normal_6,
				tiles.normal.normal_7,
				tiles.normal.normal_8,
			];
			for (idx, elem) in normals.iter().enumerate() {
				let symbol = char::from_u32('A' as u32 + idx as u32).unwrap();
				let mut map = BTreeMap::new();
				map.insert(3, (elem.normal_elem_3 * 100.0).round() as i32);
				map.insert(4, (elem.normal_elem_4 * 100.0).round() as i32);
				map.insert(5, (elem.normal_elem_5 * 100.0).round() as i32);
				wins.insert(symbol, map);
			}
			let mut wild_map = BTreeMap::new();
			wild_map.insert(5, (tiles.wilds.pays.pays_5 * 100.0).round() as i32);
			wins.insert('I', wild_map);
			let lines = payout.line_definitions.iter().map(|line| {
				line.iter().map(|v| match v {
					3 => '^',
					4 => '-',
					5 => '_',
					6 => 'V',
					_ => '?',
				}).collect::<String>()
			}).collect::<Vec<_>>();
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

fn _check_structure(name: &str) {
    for (idx, tuple) in parse_packet(name).into_iter().enumerate() {
        let val: Value = tuple.response.clone();
        let json_str = val.to_string();
        let mut deserializer = serde_json::Deserializer::from_str(&json_str);
        match deserialize::<_, Option<SourceHoldAndWin>>(&mut deserializer) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("[{name}] [ERROR]\n→ line {idx}\n→ body: {json_str}\n→ reason: {err}\n");
            }
        }
    }
}

fn convert(name: &str) {
    let list = parse_packet(name);
    let mut iter = list.into_iter().peekable();
    let mut results: Vec<FuGaSoTuple<ThunderExpressInfo, ReSpinInfo>> = Vec::new();
    while let Some(tuple) = iter.next() {
		let converted_tr_opt: Option<Vec<FuGaSoTuple<ThunderExpressInfo, ReSpinInfo>>> = match serde_json::from_value::<Option<SourceHoldAndWin>>(tuple.response).inspect_err(|e| {eprintln!("err {e}");}).unwrap_or(None) {
			Some(SourceHoldAndWin { result, .. }) => {
				if let Ok(Some(player)) = serde_json::from_value::<Option<Player>>(tuple.request) {
					if let Some(result_context) = result {
						if let Some(spins) = result_context.game.spins.as_ref() {
							if let Some(first_spin) = spins.first() {
								let mut round_tansactions = Vec::new();
								
								//input
								let denom: i32 = 10;
								let bet_counters = [1, 70, 150, 2];
								let mut total_bet = 0;
								let mut selected_mode = 0;
								if let Some(bet) = &player.bet {total_bet = (bet.slot * 100.0) as i32  / denom;}
								if let Some(buy_feature) = &player.buy_feature {
									selected_mode = match buy_feature.id {
										293 => 1,
										294 => 2,
										295 => 3,
										_ => 0,
									}
								}
								let bet_counter = bet_counters[selected_mode];
								//request
								let request = fugaso_math::math::Request {
									bet: total_bet,
									bet_counter,
									denom,
									..Default::default()
								};
								let input = PlayerRequest::BetSpin(request.clone());
								
								let total_win;
								// spins
								{
									let grid = {
										let sd = &first_spin.spin_data;
										let reels_payout = &sd.reels_payout;
										let reels_offset = &sd.reels_offset;
										(0..BOARD_WIDTH).map(|c| {
											let offset = reels_offset.get(c).copied().unwrap_or(0) as usize;
											(0..BOARD_HEIGHT).map(|r| {
												let num = reels_payout.get(c).and_then(|col| col.get(r + offset)).copied().unwrap_or_default();
												char::from_u32('A' as u32 + (num as u32).saturating_sub(1)).unwrap_or('?')
											}).collect::<Vec<_>>()
										}).collect::<Vec<Vec<_>>>()
									};
									let gains: Vec<Gain> = first_spin.spin_data.tile_wins.iter().map(|tw| {
										Gain {
											symbol: char::from_u32('A' as u32 + (tw.tile_id as u32).saturating_sub(1)).unwrap_or('?'),
											count: tw.positions.len(),
											offset: 0,
											amount: (tw.total_amount * 100.0) as i64,
											line_num: tw.line_id as usize,
											multi: tw.multiplier as i32,
											free_spins: 0,
											columns: None,
											indexes: Vec::new(),
											..Default::default()
										}
									}).collect();
									let overlay = if let Some(next_spin) = spins.get(1) {
										if next_spin.spins_type == SpinsTypeEnum::Respin { 
											if first_spin.spin_data.reels_payout != next_spin.spin_data.reels_payout {
												let sd = &next_spin.spin_data;
												let reels_payout = &sd.reels_payout;
												let reels_offset = &sd.reels_offset;
												Some((0..BOARD_WIDTH).map(|c| {
													let offset = reels_offset.get(c).copied().unwrap_or(0) as usize;
													(0..BOARD_HEIGHT).map(|r| {
														let num = reels_payout.get(c).and_then(|col| col.get(r + offset)).copied().unwrap_or_default();
														char::from_u32('A' as u32 + (num as u32).saturating_sub(1)).unwrap_or('?')
													}).collect::<Vec<_>>()
												}).collect::<Vec<Vec<_>>>())
											} else {None}
										} else {None}
									} else {None};
									let mut mults_opt: Option<Vec<Vec<i32>>> = None;
									let mut mults1_opt: Option<Vec<Vec<i32>>> = None;
									let (total, next_act, respins) =if first_spin.spin_data.activator.is_some() {
										if let Some(next_spin) = spins.get(1) {
											if next_spin.spins_type == SpinsTypeEnum::Respin { 
												next_spin.spin_data.cash_tiles.clone().unwrap_or_default().iter().for_each(|tile| {
													let mults_temp = mults_opt.get_or_insert(vec![vec![0; 3]; 5]);
													let v = tile.features.clone().and_then(|f| f.multiplier).map(|m| m.to as i32);
													v.map(|val| {
														mults_temp[tile.x as usize][(tile.y-3) as usize] = val;
													});
												});
												next_spin.spin_data.collector_tiles.clone().unwrap_or_default().iter().for_each(|tile| {
													let mults1_temp = mults1_opt.get_or_insert(vec![vec![0; 3]; 5]);
													let v = tile.features.clone().and_then(|f| f.multiplier).map(|m| m.to as i32);
													v.map(|val| {
														mults1_temp[tile.x as usize][(tile.y-3) as usize] = val;
													});
												});
											}
										}
										let total = (first_spin.win.total * 100.0) as i64;
										(total, ActionKind::RESPIN, 3)
									} else {
										first_spin.spin_data.cash_tiles.clone().unwrap_or_default().iter().for_each(|tile| {
											let mults_temp = mults_opt.get_or_insert(vec![vec![0; 3]; 5]);
											let v = tile.features.clone().and_then(|f| f.multiplier).map(|m| m.to as i32);
											v.map(|val| {
												mults_temp[tile.x as usize][(tile.y-3) as usize] = val;
											});
										});
										first_spin.spin_data.collector_tiles.clone().unwrap_or_default().iter().for_each(|tile| {
											let mults1_temp = mults1_opt.get_or_insert(vec![vec![0; 3]; 5]);
											let v = tile.features.clone().and_then(|f| f.multiplier).map(|m| m.to as i32);
											v.map(|val| {
												mults1_temp[tile.x as usize][(tile.y-3) as usize] = val;
											});
										});
										let total = result_context.game.win.as_ref().map(|ev| ev.total * 100.0).unwrap_or(0.0) as i64;
										let next_act = if total > 0 {ActionKind::COLLECT} else {ActionKind::BET};
										(total, next_act, 0)
									};
									let mults = if let Some(mults) = mults_opt {mults} else {Vec::new()};
									let mults1 = if let Some(mults1) = mults1_opt {mults1} else {Vec::new()};
									total_win = total;
									let link = ThunderExpressInfo {total, respins, overlay, mults, mults1,..Default::default() };
									let response: SpinData<ThunderExpressInfo, ReSpinInfo> = SpinData {
										id: id::GAME_DATA,
										balance: result_context.user.balance.after_bet.as_ref().map(|a| a.cash * 100.0).unwrap_or(0.0) as i64,
										credit_type: 100,
										result: GameResult {
											total,
											grid,
											special: Some(link),
											gains,
											..Default::default()
										},
										curr_lines: 5,
										curr_bet: request.bet,
										curr_denom: request.denom,
										curr_reels: 5,
										next_act,
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
									round_tansactions.push(FuGaSoTuple {input, output: vec![output]});
								}

								//freespins
								if first_spin.spin_data.activator.is_some() {
									for (freespin_num, freespin) in spins.iter().enumerate().skip(1) {
										if freespin.spins_type == SpinsTypeEnum::Freespin { 
											let is_last = freespin_num + 1 == spins.len();
											let grid = {
												let sd = &freespin.spin_data;
												let reels_payout = &sd.reels_payout;
												let reels_offset = &sd.reels_offset;
												(0..BOARD_WIDTH).map(|c| {
													let offset = reels_offset.get(c).copied().unwrap_or(0) as usize;
													(0..BOARD_HEIGHT).map(|r| {
														let num = reels_payout.get(c).and_then(|col| col.get(r + offset)).copied().unwrap_or_default();
														char::from_u32('A' as u32 + (num as u32).saturating_sub(1)).unwrap_or('?')
													}).collect::<Vec<_>>()
												}).collect::<Vec<Vec<_>>>()
											};
											let gains_collector: Vec<Gain> = freespin.spin_data.collector_tiles.clone().unwrap_or_default().iter().map(|tw| {
												let amount = tw.features.clone().and_then(|f| f.multiplier).map(|m| m.to as i64).unwrap_or_default() * 100;
												Gain {
													symbol: char::from_u32('A' as u32 + (tw.tile_id as u32).saturating_sub(1)).unwrap_or('?'),
													count: 1,
													amount,
													multi: 1,
													..Default::default()
												}
											}).collect();
											let gains: Vec<Gain> = if is_last {gains_collector} else {Vec::new()};
											let mut mults_opt: Option<Vec<Vec<i32>>> = None;
											freespin.spin_data.cash_tiles.clone().unwrap_or_default().iter().for_each(|tile| {
												let mults_temp = mults_opt.get_or_insert(vec![vec![0; 3]; 5]);
												let v = tile.features.clone().and_then(|f| f.multiplier).map(|m| m.to as i32);
												v.map(|val| {
													mults_temp[tile.x as usize][(tile.y-3) as usize] = val;
												});
											});
											let mut mults1 = vec![vec![0; 3]; 5];
											freespin.spin_data.collector_tiles.clone().unwrap_or_default().iter().for_each(|tile| {
												if let Some(mults_temp) = &mut mults_opt {
													let v_from = tile.features.clone().and_then(|f| f.multiplier).map(|m| m.from as i32);
													v_from.map(|val| {
														mults_temp[tile.x as usize][(tile.y-3) as usize] = val;
													});
												}
												let v = tile.features.clone().and_then(|f| f.multiplier).map(|m| m.to as i32);
												v.map(|val| {
													mults1[tile.x as usize][(tile.y-3) as usize] = val;
												});
											});
											let mults = if let Some(mults) = mults_opt {mults} else {Vec::new()};
											let accum = (freespin.win.total * 100.0) as i64;
											let total = total_win + (freespin.win.total * 100.0) as i64;
											let next_act = if is_last  {ActionKind::COLLECT} else {ActionKind::RESPIN};
											let respins = if let Some(free_spins) = &freespin.spin_data.free_spins {(free_spins.total_spins - free_spins.current_spin) as i32} else {0};
											let link = ThunderExpressInfo {total, respins, accum, mults, mults1,..Default::default() };
											let response: SpinData<ThunderExpressInfo, ReSpinInfo> = SpinData {
												id: id::GAME_DATA,
												balance: result_context.user.balance.after_bet.as_ref().map(|a| a.cash * 100.0).unwrap_or(0.0) as i64,
												credit_type: 100,
												result: GameResult {
													total,
													grid,
													special: Some(link),
													gains,
													..Default::default()
												},
												curr_lines: 5,
												curr_bet: request.bet,
												curr_denom: request.denom,
												curr_reels: 5,
												next_act,
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
											round_tansactions.push(FuGaSoTuple {input: PlayerRequest::ReSpin, output: vec![output] });
										}
									}
								}

								Some(round_tansactions)
							} else {None}
						} else {None}
					} else {None}
				} else {None}
			}
			_ => None,
		};
		//add to result tansactions
		if let Some(converted_tr) = converted_tr_opt {results.extend(converted_tr)};
    }
	//save
    std::fs::create_dir_all(format!("packets_result/{GAME_FUGASO_FOLDER}")).unwrap();
    serde_json::to_writer(File::create(format!("packets_result/{GAME_FUGASO_FOLDER}/{name}")).expect("error file open"), &results,).expect("error write file");
}

#[test]
//#[ignore]
#[allow(unused)]
fn test_convert() {
    convert("00-no_win.json");
    //convert("01-win.json");
    convert("02-fs.json");
    convert("03-fs_tree_train.json");
    //convert("04-collect_collect_multi.json");
    //convert("bet_200_line_1_c99f22c4e09b4390b9f77369fa0371f2.json");
    //_check_structure("bet_200_line_1_c99f22c4e09b4390b9f77369fa0371f2.json");
}


pub mod string_u64 {
    use serde::{self, Deserialize, Deserializer, Serializer};
    pub fn serialize<S>(val: &u64, s: S) -> Result<S::Ok, S::Error> where S: Serializer, {
        s.serialize_str(&val.to_string())
    }
    pub fn deserialize<'de, D>(d: D) -> Result<u64, D::Error> where D: Deserializer<'de>, {
        let v = serde_json::Value::deserialize(d)?;
        match v {
            serde_json::Value::String(s) => s.parse::<u64>().map_err(serde::de::Error::custom),
            serde_json::Value::Number(n) => n.as_u64().ok_or_else(|| serde::de::Error::custom("invalid number")),
            other => Err(serde::de::Error::custom(format!("expected string or number, got {other:?}"))),
        }
    }
    pub mod option {
        use super::*;
        use serde::Deserialize;
        pub fn serialize<S>(val: &Option<u64>, s: S) -> Result<S::Ok, S::Error> where S: Serializer, {
            match val {
                Some(v) => s.serialize_str(&v.to_string()),
                None => s.serialize_none(),
            }
        }
        pub fn deserialize<'de, D>(d: D) -> Result<Option<u64>, D::Error> where D: Deserializer<'de>, {
            let v = Option::<serde_json::Value>::deserialize(d).unwrap_or(None);
            match v {
                Some(serde_json::Value::String(s)) => {if s.is_empty() {Ok(None)} else {s.parse::<u64>().map(Some).map_err(serde::de::Error::custom)}},
                Some(serde_json::Value::Number(n)) => Ok(n.as_u64()),
                Some(serde_json::Value::Null) => Ok(None),
                Some(other) => Err(serde::de::Error::custom(format!("expected string or number, got {other:?}"))),
                None => Ok(None),
            }
        }
    }
}

pub mod string_vec_u64 {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use serde::ser::SerializeSeq;
    pub fn serialize<S>(vals: &Vec<u64>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, {
        let mut seq = serializer.serialize_seq(Some(vals.len()))?;
        for v in vals {
            seq.serialize_element(&v.to_string())?;
        }
        seq.end()
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u64>, D::Error> where D: Deserializer<'de>, {
        let vals = Vec::<serde_json::Value>::deserialize(deserializer)?;
        vals.into_iter().map(|v| match v {
			serde_json::Value::String(s) => s.parse::<u64>().map_err(serde::de::Error::custom),
			serde_json::Value::Number(n) => n.as_u64().ok_or_else(|| serde::de::Error::custom("invalid number")),
			other  => Err(serde::de::Error::custom(format!("expected string or number, got {other:?}"))),
		}).collect()
    }
    pub mod option {
        use super::*;
        use serde::Deserialize;
        pub fn serialize<S>(vals: &Option<Vec<u64>>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, {
            match vals {
                Some(vec) => {
                    let mut seq = serializer.serialize_seq(Some(vec.len()))?;
                    for v in vec {
                        seq.serialize_element(&v.to_string())?;
                    }
                    seq.end()
                }
                None => serializer.serialize_none(),
            }
        }
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u64>>, D::Error> where D: Deserializer<'de>, {
            let opt = Option::<Vec<serde_json::Value>>::deserialize(deserializer).unwrap_or(None);
            match opt {
                Some(vals) => {
					Ok(Some(
						vals.into_iter().map(|v| match v {
							serde_json::Value::String(s) => s.parse::<u64>().map_err(serde::de::Error::custom),
							serde_json::Value::Number(n) => n.as_u64().ok_or_else(|| serde::de::Error::custom("invalid number")),
							other => Err(serde::de::Error::custom(format!("expected string or number, got {other:?}"))),
						}).collect::<Result<Vec<_>, _>>()?,
					))
				},
                None => Ok(None),
            }
        }
    }
}

pub mod string_f64 {
    use serde::{self, Deserialize, Deserializer, Serializer};
    pub fn serialize<S>(val: &f64, s: S) -> Result<S::Ok, S::Error> where S: Serializer, {s.serialize_str(&val.to_string())}
    pub fn deserialize<'de, D>(d: D) -> Result<f64, D::Error> where D: Deserializer<'de>, {
        let v = serde_json::Value::deserialize(d)?;
        match v {
            serde_json::Value::String(s) => s.parse::<f64>().map_err(serde::de::Error::custom),
            serde_json::Value::Number(n) => n.as_f64().ok_or_else(|| serde::de::Error::custom("invalid number")),
            other  => Err(serde::de::Error::custom(format!("expected string or number, got {other:?}"))),
        }
    }
    pub mod option {
        use super::*;
        use serde::Deserialize;
        pub fn serialize<S>(val: &Option<f64>, s: S) -> Result<S::Ok, S::Error> where S: Serializer, {
            match val {
                Some(v) => s.serialize_str(&v.to_string()),
                None => s.serialize_none(),
            }
        }
        pub fn deserialize<'de, D>(d: D) -> Result<Option<f64>, D::Error> where D: Deserializer<'de>, {
            let v = Option::<serde_json::Value>::deserialize(d).unwrap_or(None);
            match v {
                Some(serde_json::Value::String(s)) => {if s.is_empty() {Ok(None)} else {s.parse::<f64>().map(Some).map_err(serde::de::Error::custom)}}
                Some(serde_json::Value::Number(n)) => Ok(n.as_f64()),
                Some(serde_json::Value::Null) => Ok(None),
                Some(other) => Err(serde::de::Error::custom(format!("expected string or number, got {other:?}"))),
                None => Ok(None),
            }
        }
    }
}

pub mod string_vec_f64 {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use serde::ser::SerializeSeq;
    pub fn serialize<S>(vals: &Vec<f64>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, {
        let mut seq = serializer.serialize_seq(Some(vals.len()))?;
        for v in vals {
            seq.serialize_element(&v.to_string())?;
        }
        seq.end()
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error> where D: Deserializer<'de>, {
        let vals = Vec::<serde_json::Value>::deserialize(deserializer)?;
        vals.into_iter().map(|v| match v {
			serde_json::Value::String(s) => s.parse::<f64>().map_err(serde::de::Error::custom),
			serde_json::Value::Number(n) => n.as_f64().ok_or_else(|| serde::de::Error::custom("invalid number")),
			other  => Err(serde::de::Error::custom(format!("expected string or number, got {other:?}"))),
		}).collect()
    }
    pub mod option {
        use super::*;
        use serde::Deserialize;
        pub fn serialize<S>(vals: &Option<Vec<f64>>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, {
            match vals {
                Some(vec) => {
                    let mut seq = serializer.serialize_seq(Some(vec.len()))?;
                    for v in vec {
                        seq.serialize_element(&v.to_string())?;
                    }
                    seq.end()
                }
                None => serializer.serialize_none(),
            }
        }
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<f64>>, D::Error> where D: Deserializer<'de>, {
            let opt = Option::<Vec<serde_json::Value>>::deserialize(deserializer).unwrap_or(None);
            match opt {
                Some(vals) => Ok(Some(
                    vals.into_iter().map(|v| match v {
						serde_json::Value::String(s) => s.parse::<f64>().map_err(serde::de::Error::custom),
						serde_json::Value::Number(n) => n.as_f64().ok_or_else(|| serde::de::Error::custom("invalid number")),
						other  => Err(serde::de::Error::custom(format!("expected string or number, got {other:?}"))),
					}).collect::<Result<Vec<_>, _>>()?,
                )),
                None => Ok(None),
            }
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct GameBet {
	#[serde(with = "string_f64")]
	pub slot: f64 /* 1.00 */,
	#[serde(with = "string_f64")]
	pub total: f64 /* 1.00 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BuyBets {
	#[serde(with = "string_f64")]
	pub base: f64 /* 0.10, 0.20, 0.40, 0.60, 0.80, 1.00, 1.50, 10.00, 100.00, 12.00, 14.00, 150.00, 16.00, 18.00, 2.00, 2.50 */,
	#[serde(with = "string_f64")]
	pub total: f64 /* 0.15, 0.30, 0.60, 0.90, 1.20, 1.50, 10.50, 105.00, 12.00, 120.00, 13.50, 135.00, 15.00, 150.00, 18.00 */,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum BuyFeaturesKeyEnum {
	#[default]
	#[serde(rename = "free-spins-70")]
	FreeSpins70,
	#[serde(rename = "free-spins-150")]
	FreeSpins150,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct GameBuy {
	pub bets: Vec<BuyBets>,
	pub id: i64 /* 293, 294 */,
	pub key: BuyFeaturesKeyEnum /* free-spins-150, free-spins-70 */,
	#[serde(with = "string_f64")]
	pub multiplier: f64 /* 150.00, 70.00, 1.50 */,
	#[serde(with = "string_f64")]
	pub rtp: f64 /* 95.85 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct GameGaffWinMultipliers {
	#[serde(skip_serializing_if = "Option::is_none", rename = "freeSpin")]
	pub free_spin: Option<i64> /* 10, 12, 15, 16, 17, 18, 19, 21, 24, 26, 27, 28, 33, 37, 45, 47, 50, 55, 57, 60, 62, 63, 67, 74, 81, 97, 113, 117, 132, 261, 273 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub respin: Option<i64> /* 0 */,
	#[serde(with = "string_f64")]
	pub spin: f64 /* 11.5, 110.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 20, 22, 24, 25, 26, 28, 30 */,
	#[serde(with = "string_f64")]
	pub total: f64 /* 11.5, 110.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 24 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct GameGaff {
	pub seed: i64, /* 408525, 597814, 662053, 1281871, 1419097, 1621981, 3433128, 3790376, 4020685, 4224954, 5462912 */
	#[serde(rename = "winMultipliers")]
	pub win_multipliers: GameGaffWinMultipliers,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Rtp {
	#[serde(with = "string_f64")]
	pub game: f64 /* 95.70 */,
}
// settings
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct JackpotMultipliers {
	#[serde(rename = "Grand")]
	pub grand: i64 /* 1000 */,
	#[serde(rename = "Major")]
	pub major: i64 /* 150 */,
	#[serde(rename = "Mini")]
	pub mini: i64 /* 25 */,
	#[serde(rename = "Minor")]
	pub minor: i64 /* 50 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct GameFeatures {
	#[serde(rename = "CashTiles", with = "string_vec_u64")]
	pub cash_tiles: Vec<u64> /* 1, 2, 3, 4, 5, 6, 7, 8, 10, 20 */,
	#[serde(rename = "JackpotMultipliers")]
	pub jackpot_multipliers: JackpotMultipliers,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NormalElem {
	#[serde(rename = "3", with = "string_f64")]
	pub normal_elem_3: f64 /* 1.00 */,
	#[serde(rename = "4", with = "string_f64")]
	pub normal_elem_4: f64 /* 3.00 */,
	#[serde(rename = "5", with = "string_f64")]
	pub normal_elem_5: f64 /* 10.00 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Normal {
	#[serde(rename = "1")]
	pub normal_1: NormalElem,
	#[serde(rename = "2")]
	pub normal_2: NormalElem,
	#[serde(rename = "3")]
	pub normal_3: NormalElem,
	#[serde(rename = "4")]
	pub normal_4: NormalElem,
	#[serde(rename = "5")]
	pub normal_5: NormalElem,
	#[serde(rename = "6")]
	pub normal_6: NormalElem,
	#[serde(rename = "7")]
	pub normal_7: NormalElem,
	#[serde(rename = "8")]
	pub normal_8: NormalElem,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Scatters {
	pub id: Vec<i64> /* [10,11,12,13,14,15,16] */,
	pub pays: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Pays {
	#[serde(rename = "5", with = "string_f64")]
	pub pays_5: f64 /* 75.00 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Wilds {
	pub id: Vec<i64> /* [9] */,
	pub pays: Pays,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Tiles {
	pub normal: Normal,
	pub scatters: Scatters,
	pub wilds: Wilds,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Payout {
	#[serde(rename = "lineDefinitions")]
	pub line_definitions: Vec<Vec<i64>> /* [[3,3,3,3,3],[4,4,4,4,4],[5,5,5,5,5],[3,4,5,4,3],[5,4,3,4,5]] */,
	#[serde(rename = "lineType")]
	pub line_type: String /* LTR */,
	pub tiles: Tiles,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Position {
	pub x: i64 /* 5 */,
	pub y: i64 /* 3 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Settings {
	#[serde(rename = "gameFeatures")]
	pub game_features: GameFeatures,
	pub payout: Payout,
	pub reels: Vec<Vec<i64>> /* [[7,6,6,7,7,1,1,1,3],[2,1,1,2,2,3,2,1,1],[6,8,4,6,8,6,6,7,8],[7,1,1,2,2,1,1,9,2],[3,8,3,8,8,8,3,3,6]] */,
	#[serde(rename = "reelsOffset")]
	pub reels_offset: Vec<i64> /* [3,3,3,3,3] */,
	pub size: Position,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct GaffAccumulatedWin {
	pub interim: i64 /* 0, 1, 10, 12, 15, 16, 17, 18, 19, 21, 24, 26, 27, 28, 33, 37, 45, 47, 50, 55, 57, 60, 62, 63, 67, 74, 81, 97, 113, 117, 132, 261, 273 */,
	pub total: i64 /* 0, 1, 10, 12, 15, 16, 17, 18, 19, 21, 24, 26, 27, 28, 33, 37, 45, 47, 50, 55, 57, 60, 62, 63, 68, 74, 81, 97, 113, 117, 132, 261, 273 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct GaffTiles {
	#[serde(rename = "1", skip_serializing_if = "Option::is_none")]
	pub tiles_1: Option<i64> /* 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 */,
	#[serde(rename = "10", skip_serializing_if = "Option::is_none")]
	pub tiles_10: Option<i64> /* 1 */,
	#[serde(rename = "11", skip_serializing_if = "Option::is_none")]
	pub tiles_11: Option<i64> /* 1, 2, 3, 4, 5, 6 */,
	#[serde(rename = "12", skip_serializing_if = "Option::is_none")]
	pub tiles_12: Option<i64> /* 1 */,
	#[serde(rename = "13", skip_serializing_if = "Option::is_none")]
	pub tiles_13: Option<i64> /* 1 */,
	#[serde(rename = "16", skip_serializing_if = "Option::is_none")]
	pub tiles_16: Option<i64> /* 9, 10, 11, 12, 13, 14, 15 */,
	#[serde(rename = "2", skip_serializing_if = "Option::is_none")]
	pub tiles_2: Option<i64> /* 1, 2, 3, 4, 5, 6, 7, 8, 9, 11 */,
	#[serde(rename = "3", skip_serializing_if = "Option::is_none")]
	pub tiles_3: Option<i64> /* 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 */,
	#[serde(rename = "4", skip_serializing_if = "Option::is_none")]
	pub tiles_4: Option<i64> /* 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 */,
	#[serde(rename = "5", skip_serializing_if = "Option::is_none")]
	pub tiles_5: Option<i64> /* 1, 2, 3, 4, 5, 6, 7, 8, 9 */,
	#[serde(rename = "6", skip_serializing_if = "Option::is_none")]
	pub tiles_6: Option<i64> /* 1, 2, 3, 4, 5, 6, 7, 8, 9 */,
	#[serde(rename = "7", skip_serializing_if = "Option::is_none")]
	pub tiles_7: Option<i64> /* 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 */,
	#[serde(rename = "8", skip_serializing_if = "Option::is_none")]
	pub tiles_8: Option<i64> /* 1, 2, 3, 4, 5, 6, 7, 8, 9 */,
	#[serde(rename = "9", skip_serializing_if = "Option::is_none")]
	pub tiles_9: Option<i64> /* 1, 2, 3, 4, 5, 6, 7, 9 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SpinsGaffWinMultipliers {
	#[serde(skip_serializing_if = "Option::is_none", rename = "cashBonus")]
	pub cash_bonus: Option<i64> /* 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 24, 26 */,
	#[serde(rename = "tileWins", with = "string_f64")]
	pub tile_wins: f64 /* 11.5, 110.5, 2.5, 3.5, 5.5, 7.5, 0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 15, 16, 20, 25, 30 */,
	#[serde(with = "string_f64")]
	pub total: f64 /* 11.5, 110.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SpinsGaff {
	#[serde(rename = "accumulatedWin")]
	pub accumulated_win: GaffAccumulatedWin,
	pub tiles: GaffTiles,
	#[serde(rename = "winMultipliers")]
	pub win_multipliers: SpinsGaffWinMultipliers,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DataAccumulatedWin {
	#[serde(with = "string_f64")]
	pub interim: f64 /* 0.00, 1.00, 10.00, 113.00, 117.00, 12.00, 132.00, 15.00, 16.00, 17.00, 18.00, 19.00, 21.00, 24.00, 26.00, 261.00, 27.00, 273.00, 28.00, */,
	#[serde(with = "string_f64")]
	pub total: f64 /* 0.00, 1.00, 10.00, 113.00, 117.00, 12.00, 132.00, 15.00, 16.00, 17.00, 18.00, 19.00, 21.00, 24.00, 26.00, 261.00, 27.00, 273.00, 28.00, */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Freespins {
	#[serde(rename = "currentSpin")]
	pub current_spin: i64 /* 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27 */,
	#[serde(rename = "newSpins")]
	pub new_spins: i64 /* 0, 1, 2, 3 */,
	#[serde(rename = "totalSpins")]
	pub total_spins: i64 /* 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 23, 25, 26, 27 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Amount {
	#[serde(with = "string_f64")]
	pub from: f64 /* 1.00, 10.00, 2.00, 20.00, 25.00, 3.00, 4.00, 5.00, 50.00, 6.00, 7.00, 8.00 */,
	#[serde(with = "string_f64")]
	pub to: f64 /* 1.00, 10.00, 2.00, 20.00, 25.00, 3.00, 4.00, 5.00, 50.00, 6.00, 7.00, 8.00 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Multiplier {
	pub from: i64 /* 1, 2, 3, 4, 5, 6, 7, 8, 10, 20, 25, 50 */,
	pub max: i64 /* 20, 25, 50 */,
	pub to: i64 /* 1, 2, 3, 4, 5, 6, 7, 8, 10, 20, 25, 50 */,
	#[serde(rename = "useFrom")]
	pub use_from: bool /* false */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Lock {
	pub from: i64 /* -1 */,
	pub to: i64 /* -1 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Features {
	pub amount: Option<Amount>,
	pub modifier: Option<Vec<HashMap<String, String>>> /*  */,
	pub multiplier: Option<Multiplier>,
	pub lock: Option<Lock>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ExtraTiles {
	pub features: Option<Features>,
	#[serde(rename = "tileId")]
	pub tile_id: i64 /* 10, 11 */,
	pub x: i64 /* 0, 1, 2, 3, 4 */,
	pub y: i64 /* 3, 4, 5 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TileWins {
	#[serde(with = "string_f64")]
	pub amount: f64 /* 1.00, 10.00, 2.50, 25.00, 3.00, 4.00, 7.50, 75.00 */,
	#[serde(rename = "lineId")]
	pub line_id: i64 /* 0, 1, 2, 3, 4 */,
	pub multiplier: i64 /* 1 */,
	pub positions: Vec<Position>,
	#[serde(rename = "tileId")]
	pub tile_id: i64 /* 1, 2, 3, 4, 5, 6, 7, 8, 9 */,
	#[serde(rename = "totalAmount", with = "string_f64")]
	pub total_amount: f64 /* 1.00, 10.00, 2.50, 25.00, 3.00, 4.00, 7.50, 75.00 */,
	pub ways: i64 /* 1 */,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum SpindataActivatorEnum {
	#[default]
	#[serde(rename = "bonus")]
	Bonus,
	#[serde(rename = "superBonus")]
	Superbonus,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SpinsSpinData {
	#[serde(skip_serializing_if = "Option::is_none", rename = "accumulatedWin")]
	pub accumulated_win: Option<DataAccumulatedWin>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub activator: Option<Vec<SpindataActivatorEnum>> /* bonus, superBonus */,
	#[serde(skip_serializing_if = "Option::is_none", rename = "cashTiles")]
	pub cash_tiles: Option<Vec<ExtraTiles>>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "collectorTiles")]
	pub collector_tiles: Option<Vec<ExtraTiles>>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "featureType")]
	pub feature_type: Option<SpindataActivatorEnum> /* bonus, superBonus */,
	#[serde(skip_serializing_if = "Option::is_none", rename = "freeSpins")]
	pub free_spins: Option<Freespins>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "lockTiles")]
	pub lock_tiles: Option<Vec<ExtraTiles>>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "randomTiles")]
	pub random_tiles: Option<Vec<ExtraTiles>>,
	pub reels: Vec<Vec<i64>> /* [[7,6,6,3,3,7,7,6,6],[3,2,1,1,3,3,2,5,5],[4,4,10,5,8,8,7,7,10],[2,2,1,7,7,1,1,5,2],[4,8,3,3,6,6,8,8,9]] */,
	#[serde(rename = "reelsOffset")]
	pub reels_offset: Vec<i64> /* [3,3,3,3,3] */,
	#[serde(rename = "reelsPayout")]
	pub reels_payout: Vec<Vec<i64>> /* [[7,6,6,3,3,7,7,6,6],[3,2,1,1,3,3,2,5,5],[4,4,10,5,8,8,7,7,10],[2,2,1,7,7,1,1,5,2],[4,8,3,3,6,6,8,8,9]] */,
	#[serde(rename = "tileWins")]
	pub tile_wins: Vec<TileWins>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Win {
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "cashBonus", with = "string_f64::option")]
	pub cash_bonus: Option<f64> /* 1.00, 10.00, 11.00, 113.00, 117.00, 12.00, 13.00, 132.00, 14.00, 15.00, 16.00, */,
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "tileWins", with = "string_f64::option")]
	pub tile_wins: Option<f64> /* 0.00, 1.00, 10.00, 11.00, 11.50, 110.50, 12.00, 13.00, 15.00, 16.00, 2.00, 2.50, 20.00, */,
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "freeSpin", with = "string_f64::option")]
	pub free_spin: Option<f64> /* 10.00, 113.00, 117.00, 12.00, 132.00, 15.00, 16.00, 17.00, 18.00, 19.00, 21.00, 24.00, 26.00, 261.00, 27.00,  */,
	#[serde(default, skip_serializing_if = "Option::is_none", with = "string_f64::option")]
	pub respin: Option<f64> /* 0.00 */,
	#[serde(default, skip_serializing_if = "Option::is_none", with = "string_f64::option")]
	pub spin: Option<f64> /* 0.00, 1.00, 10.00, 11.00, 11.50, 110.50, 12.00, 13.00, 14.00, 15.00, 16.00, 17.00, 2.00, 2.50, 20.00, 22.00, 24.00, 25.00,  */,
	#[serde(with = "string_f64")]
	pub total: f64 /* 0.00, 1.00, 10.00, 11.00, 11.50, 110.50, 113.00, 117.00, 12.00, 13.00, 132.00, 14.00, 15.00, 16.00, 17.00, 18.00, 19.00,  */,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default, PartialEq)]
pub enum SpinsTypeEnum {
	#[default]
	#[serde(rename = "spin")]
	Spin,
	#[serde(rename = "respin")]
	Respin,
	#[serde(rename = "freeSpin")]
	Freespin,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Spins {
	pub gaff: SpinsGaff,
	pub multiplier: i64 /* 1 */,
	#[serde(rename = "spinData")]
	pub spin_data: SpinsSpinData,
	#[serde(rename = "type")]
	pub spins_type: SpinsTypeEnum /* freeSpin, respin, spin */,
	pub win: Win,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum GameIdEnum {
	#[default]
	#[serde(rename = "super-grand-link-express-hold-and-win")]
	SuperGrandLinkExpressHoldAndWin,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum GameNameEnum {
	#[default]
	#[serde(rename = "Super Grand Link Express: Hold & Win")]
	SuperGrandLinkExpressHoldAndWin,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum GameTypeEnum {
	#[default]
	#[serde(rename = "slot")]
	Slot,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Game {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet: Option<GameBet>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "buyChances")]
	pub buy_chances: Option<Vec<GameBuy>>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "buyFeatures")]
	pub buy_features: Option<Vec<GameBuy>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gaff: Option<GameGaff>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "hasState")]
	pub has_state: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<GameIdEnum> /* super-grand-link-express-hold-and-win */,
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "maxMultiplier", with = "string_f64::option")]
	pub max_multiplier: Option<f64> /* 4500 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<GameNameEnum> /* Super Grand Link Express: Hold & Win */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rtp: Option<Rtp>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub settings: Option<Settings>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub spins: Option<Vec<Spins>>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "stateType")]
	pub state_type: Option<Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub states: Option<Vec<HashMap<String, String>>> /*  */,
	#[serde(rename = "type", skip_serializing_if = "Option::is_none")]
	pub game_type: Option<GameTypeEnum> /* slot */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub version: Option<String> /* 1.0.0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub win: Option<Win>,
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "winMultiplier", with = "string_f64::option")]
	pub win_multiplier: Option<f64> /* 0.00, 1.00, 10.00, 11.00, 11.50, 110.50, 113.00, 117.00, 12.00, 13.00, 132.00, 14.00, 15.00, 16.00, 17.00, 18.00, 19.00, 2.00, */,
}
// result
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Replay {
	pub hash: String /* 3b7f3-85-bdd542, 3b7f3-85-bdd545, 3b7f3-85-bdd54c, 3b7f3-85-bdd551, 3b7f3-85-bdd554, 3b7f3-85-bdd556, 3b7f3-85-bdd559,*/
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum RoundTypeEnum {
	#[default]
	#[serde(rename = "basic")]
	Basic,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Round {
	pub delayed: bool /* false */,
	pub id: i64 /* 12440898, 12440901, 12440908, 12440913, 12440916, 12440918, 12440921, 12440923, 12440927, 12440929, 12440932, */,
	#[serde(rename = "type")]
	pub round_type: RoundTypeEnum /* basic */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct StopOn {
	#[serde(default, with = "string_u64::option")]
	pub default: Option<u64> /* 100 */,
	pub enabled: bool /* true */,
	#[serde(default, with = "string_vec_u64::option")]
	pub options: Option<Vec<u64>> /* 10, 20, 30, 50, 100, 200, 500, 1000, 5000, 10000, 50000 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ComponentParams {
	pub allowed: bool /* true */,
	pub rounds: Option<StopOn>,
	#[serde(rename = "stopOnFeature")]
	pub stop_on_feature: Option<StopOn>,
	#[serde(rename = "stopOnLossLimit")]
	pub stop_on_loss_limit: Option<StopOn>,
	#[serde(rename = "stopOnWinMultiplier")]
	pub stop_on_win_multiplier: Option<StopOn>,
	pub refresh: Option<bool> /* false */,
	#[serde(rename = "refreshInterval")]
	pub refresh_interval: Option<i64> /* 0 */,
	#[serde(rename = "showMaxWin")]
	pub show_max_win: Option<bool> /* true */,
	#[serde(rename = "showOnStart")]
	pub show_on_start: Option<bool> /* false */,
	#[serde(rename = "showPaytableAmounts")]
	pub show_paytable_amounts: Option<bool> /* false */,
	#[serde(rename = "showRtp")]
	pub show_rtp: Option<bool> /* true */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Components {
	pub autoplay: ComponentParams,
	pub balance: ComponentParams,
	#[serde(rename = "fastPlay")]
	pub fast_play: ComponentParams,
	pub fullscreen: ComponentParams,
	#[serde(rename = "gameSpeed")]
	pub game_speed: ComponentParams,
	pub help: ComponentParams,
	pub history: ComponentParams,
	#[serde(rename = "holdToPlay")]
	pub hold_to_play: ComponentParams,
	#[serde(rename = "maxBet")]
	pub max_bet: ComponentParams,
	#[serde(rename = "maxWinDetails")]
	pub max_win_details: ComponentParams,
	#[serde(rename = "minBetDuration")]
	pub min_bet_duration: ComponentParams,
	#[serde(rename = "minBetNotification")]
	pub min_bet_notification: ComponentParams,
	#[serde(rename = "netPosition")]
	pub net_position: ComponentParams,
	#[serde(rename = "previousNoWinNotifications")]
	pub previous_no_win_notifications: ComponentParams,
	#[serde(rename = "previousWinNotifications")]
	pub previous_win_notifications: ComponentParams,
	#[serde(rename = "responsibleGamingLink")]
	pub responsible_gaming_link: ComponentParams,
	#[serde(rename = "roundId")]
	pub round_id: ComponentParams,
	pub rtp: ComponentParams,
	#[serde(rename = "sessionTimer")]
	pub session_timer: ComponentParams,
	#[serde(rename = "slamStop")]
	pub slam_stop: ComponentParams,
	pub splash: ComponentParams,
	#[serde(rename = "uniformButtons")]
	pub uniform_buttons: ComponentParams,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum LayoutNameEnum {
	#[default]
	#[serde(rename = "standard")]
	Standard,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum LayoutTypeEnum {
	#[default]
	#[serde(rename = "main")]
	Main,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Layout {
	pub name: LayoutNameEnum /* standard */,
	#[serde(rename = "type")]
	pub layout_type: LayoutTypeEnum /* main */,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum UiThemeEnum {
	#[default]
	#[serde(rename = "octoplay")]
	Octoplay,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Ui {
	pub components: Components,
	pub layout: Layout,
	pub theme: UiThemeEnum /* octoplay */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BalanceBet {
	#[serde(with = "string_f64")]
	pub cash: f64 /* 10000.00, 10000.50, 10001.00, 10001.50, 10002.00, 10002.50, 10003.50, 10004.50, 10005.50, 10006.50, 10007.50, 10008.50, 10009.50, 10010.50, 10011.50, 10012.50, */
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Balance {
	#[serde(skip_serializing_if = "Option::is_none", rename = "afterBet")]
	pub after_bet: Option<BalanceBet>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "afterSettle")]
	pub after_settle: Option<BalanceBet>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "beforeBet")]
	pub before_bet: Option<BalanceBet>,
	#[serde(default, skip_serializing_if = "Option::is_none", with = "string_f64::option")]
	pub cash: Option<f64> /* 10000.00 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserBets {
	#[serde(with = "string_vec_f64")]
	pub available: Vec<f64> /* 0.10, 0.20, 0.40, 0.60, 0.80, 1.00, 1.50, 10.00, 100.00, 12.00, 14.00, 150.00, */,
	#[serde(with = "string_f64")]
	pub default: f64 /* 1.00 */,
	#[serde(with = "string_f64")]
	pub last: f64 /* 1.00 */,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum CurrencyCodeEnum {
	#[default]
	#[serde(rename = "EUR")]
	EUR,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Currency {
	pub code: CurrencyCodeEnum /* EUR */,
	pub symbol: String /* € */,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum LanguageFullEnum {
	#[default]
	#[serde(rename = "en-GB")]
	EnGB,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum LanguageShortEnum {
	#[default]
	#[serde(rename = "en")]
	En,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Language {
	#[serde(rename = "fullCode")]
	pub full_code: LanguageFullEnum /* en-GB */,
	#[serde(rename = "shortCode")]
	pub short_code: LanguageShortEnum /* en */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Session {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<i64> /* 243699 */,
	#[serde(with = "string_f64", rename = "netPosition")]
	pub net_position: f64 /* -0.50, -1.00, -1.50, -10.00, -10.50, -100.50, -1000.00, -1000.50, -1001.00, -1001.50, -1002.00, -1002.50, -1003.00, */
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
	pub balance: Balance,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bets: Option<UserBets>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub currency: Option<Currency>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<i64> /* 243699 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<Language>,
	pub notifications: Vec<HashMap<String, String>> /*  */,
	pub session: Session,
	#[serde(rename = "timeNextAllowedBet")]
	pub time_next_allowed_bet: String /* 2025-10-20T07:19:59.997Z, 2025-10-20T07:20:27.622Z, 2025-10-20T07:20:29.232Z */,
	pub token: String /* 7a6ea6fe-fa00-41f7-b88f-f1a1c749ca11 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Result {
	#[serde(rename = "freeRounds")]
	pub free_rounds: Vec<HashMap<String, String>> /*  */,
	pub game: Game,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub replay: Option<Replay>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub round: Option<Round>,
	#[serde(rename = "serverTime")]
	pub server_time: String /* 2025-10-20T07:20:00.101Z, 2025-10-20T07:20:27.136Z, 2025-10-20T07:20:28.744Z, 2025-10-20T07:20:30.308Z, 2025-10-20T07:20:32.461Z, */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ui: Option<Ui>,
	pub user: User,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Error {
	pub code: i64 /* 1 */,
	pub details: HashMap<String, String>,
	pub message: String /* A technical error has occurred. If the issue persists, please contact customer support. */,
	pub recoverable: bool /* false */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SourceHoldAndWin {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<Error>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub result: Option<Result>,
}

impl SourceHoldAndWin {
	pub fn new() -> Self {
		Self::default()
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PlayerBet {
	#[serde(with = "string_f64")]
	pub slot: f64 /* 1.00 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PlayerBuyFeature {
	#[serde(with = "string_f64")]
	pub cost: f64 /* 1.00 */,
	pub id: u64 /* 1.00 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PlayerContext {
	#[serde(skip)]
	pub extra: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Player {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet: Option<PlayerBet>,
	#[serde(rename = "buyFeature")]
	pub buy_feature: Option<PlayerBuyFeature>,
	pub context: PlayerContext,
	pub fingerprint: String /* 70c41b9c-21b6-4990-8ac1-33e1b8eb8208, e801f833-f439-4700-92ea-81f6cbeef3c1 */,
	pub game: GameIdEnum /* super-grand-link-express-hold-and-win */,
	pub language: LanguageFullEnum /* en-GB */,
	pub mode: String /* demo */,
	pub token: String /* 7a6ea6fe-fa00-41f7-b88f-f1a1c749ca11, 88930dc2-ffb5-4a45-afd3-33d0e73af7c7 */,
}

impl Player {
	pub fn new() -> Self {
		Self::default()
	}
}