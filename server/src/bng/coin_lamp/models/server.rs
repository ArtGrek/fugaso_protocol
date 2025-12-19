use serde::{Serialize, Deserialize};
use strum_macros::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum ServerCommandEnum {
	#[default]
	#[serde(rename = "login")]
	Login,
	#[serde(rename = "play")]
	Play,
	#[serde(rename = "start")]
	Start,
	#[serde(rename = "sync")]
	Sync,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum ContextActionsEnum {
	#[default]
	#[serde(rename = "bonus_init")]
	BonusInit,
	#[serde(rename = "bonus_spins_stop")]
	BonusSpinsStop,
	#[serde(rename = "buy_spin")]
	BuySpin,
	#[serde(rename = "respin")]
	Respin,
	#[serde(rename = "spin")]
	Spin,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display)]
pub enum BonusBsVEnum {
	Float(f64),
	Int(i64),
	String(String),
}

impl Default for BonusBsVEnum {
	fn default() -> Self {
		BonusBsVEnum::Int(0)
	}
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum ContextCurrentEnum {
	#[default]
	#[serde(rename = "bonus")]
	Bonus,
	#[serde(rename = "spins")]
	Spins,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum ContextLastActionEnum {
	#[default]
	#[serde(rename = "bonus_init")]
	BonusInit,
	#[serde(rename = "bonus_spins_stop")]
	BonusSpinsStop,
	#[serde(rename = "buy_spin")]
	BuySpin,
	#[serde(rename = "init")]
	Init,
	#[serde(rename = "respin")]
	Respin,
	#[serde(rename = "spin")]
	Spin,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum LastargsSelectedModeEnum {
	#[default]
	#[serde(rename = "1")]
	Enum1,
	#[serde(rename = "2")]
	Enum2,
	#[serde(rename = "3")]
	Enum3,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum ServerModesEnum {
	#[default]
	#[serde(rename = "auto")]
	Auto,
	#[serde(rename = "freebet")]
	Freebet,
	#[serde(rename = "play")]
	Play,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum SymbolsNameEnum {
	#[default]
	#[serde(rename = "el_0")]
	El0,
	#[serde(rename = "el_bonus")]
	ElBonus,
	#[serde(rename = "el_bonus_sticky")]
	ElBonusSticky,
	#[serde(rename = "el_collect")]
	ElCollect,
	#[serde(rename = "el_mystery")]
	ElMystery,
	#[serde(rename = "el_mystery_jackpot")]
	ElMysteryJackpot,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum SymbolsTypeEnum {
	#[default]
	#[serde(rename = "line")]
	Line,
	#[serde(rename = "scat")]
	Scat,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum StatusCodeEnum {
	#[default]
	#[serde(rename = "FUNDS_EXCEED")]
	FundsExceed,
	#[serde(rename = "GAME_REOPENED")]
	GameReopened,
	#[serde(rename = "OK")]
	Ok,
	#[serde(rename = "PLAYER_DISCONNECTED")]
	PlayerDisconnected,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum StatusTypeEnum {
	#[default]
	#[serde(rename = "crit")]
	Crit,
	#[serde(rename = "exceed")]
	Exceed,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CollectValues {
	pub bs_v: f64 /* 390.0 */,
	pub pos: Vec<i64> /* [1,2] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Mults {
	pub mult: i64 /* 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 */,
	pub pos: Vec<i64> /* [1,5] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MysteryValues {
	pub bs_v: f64 /* 60 */,
	pub id: i64 /* 1, 3 */,
	pub pos: Vec<i64> /* [3,5] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bonus {
	pub bac: Vec<i64> /* [1,0], [1,1], [2,0], [2,1], [2,2], [2,3], [2,4], [3,0], [3,1], [3,2], [3,3], [3,4], [3,5], [3,6], [3,7], [3,8], [3,9], [4,0], [4,10], [4,11], [4,12], [4,13], [4,14], [4,1], [4,2], [4,3], [4,4], [4,5], [4,6], [4,7], [4,8], [4,9], [5,0], [5,10], [5,11], [5,12], [5,13], [5,14], [5,15], [5,16], [5,17], [5,18], [5,19], [5,1], [5,2], [5,3], [5,4], [5,5], [5,6], [5,7], [5,8], [5,9], [6,0], [6,10], [6,11], [6,12], [6,13], [6,14], [6,15], [6,16], [6,17], [6,18], [6,19], [6,1], [6,20], [6,21], [6,22], [6,23], [6,24], [6,25], [6,26], [6,27], [6,28], [6,29], [6,2], [6,3], [6,4], [6,5], [6,6], [6,7], [6,8], [6,9], [7,0] */,
	pub back_to: String /* spins */,
	pub bet_per_line: i64 /* 1 */,
	pub board: Vec<Vec<i64>> /* [[0,0,0,0,1,0],[0,0,0,0,1,0],[0,0,0,0,1,0],[0,0,0,0,1,0]] */,
	pub bonus_game_type: i64 /* 1, 2, 3 */,
	pub bs_count: i64 /* 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bs_pos: Option<Vec<Vec<i64>>> /* [[0,4],[1,4],[2,4],[3,4]] */,
	pub bs_v: Vec<Vec<BonusBsVEnum>> /* [[0,0,0,0,20,0],[0,0,0,0,10.0,0],[0,0,0,0,20,0],[0,0,0,0,80,0]] */,
	pub bs_values: Vec<Vec<f64>> /* [[0,0,0,0,1,0],[0,0,0,0,0.5,0],[0,0,0,0,1,0],[0,0,0,0,4,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub collect_values: Option<Vec<CollectValues>>,
	pub current_win: i64 /* 0 */,
	pub lines: i64 /* 1 */,
	pub mps_: Vec<Vec<i64>> /* [[1,1,1,1,1,1],[1,1,1,1,1,1],[1,1,1,1,1,1],[1,1,1,1,1,1]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mult_new: Option<Vec<Vec<i64>>> /* [[3,5],[0,3]] */,
	pub mults_: Vec<Mults>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mystery_values: Option<Vec<MysteryValues>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub new_bs: Option<Vec<Vec<i64>>> /* [[0,2]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub orig_board: Option<Vec<Vec<i64>>> /* [[0,0,0,0,1,0],[0,0,0,0,1,0],[0,0,0,0,1,0],[0,0,0,0,1,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub orig_bs_v: Option<Vec<Vec<BonusBsVEnum>>> /* [[0,0,10.0,0,20,0],[60,0,0,0,10.0,0],[40,0,0,30.0,20,0],[0,60,0,0,80,0]] */,
	pub round_bet: i64 /* 20 */,
	pub round_win: i64 /* 0 */,
	pub rounds_count: i64 /* 0 */,
	pub rounds_granted: i64 /* 3 */,
	pub rounds_left: i64 /* 0, 1, 2, 3 */,
	pub total_win: i64 /* 0 */,
	pub unlock_row_idx: i64 /* 0, 1, 2, 3 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LastArgs {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_factor: Option<i64> /* 20 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_per_line: Option<i64> /* 1 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lines: Option<i64> /* 1 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<LastargsSelectedModeEnum> /* 1, 2, 3 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Progress {
	pub bet: String /* 20 */,
	pub data: Vec<Vec<i64>> /* [[1,1,6]] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Spins {
	pub bac: Vec<i64> /* [1,0], [1,1], [2,0], [2,1], [2,2], [2,3], [2,4], [3,0], [3,1], [3,2], [3,3], [3,4], [3,5], [3,6], [3,7], [3,8], [3,9], [4,0], [4,10], [4,11], [4,12], [4,13], [4,14], [4,1], [4,2], [4,3], [4,4], [4,5], [4,6], [4,7], [4,8], [4,9], [5,0], [5,10], [5,11], [5,12], [5,13], [5,14], [5,15], [5,16], [5,17], [5,18], [5,19], [5,1], [5,2], [5,3], [5,4], [5,5], [5,6], [5,7], [5,8], [5,9], [6,0], [6,10], [6,11], [6,12], [6,13], [6,14], [6,15], [6,16], [6,17], [6,18], [6,19], [6,1], [6,20], [6,21], [6,22], [6,23], [6,24], [6,25], [6,26], [6,27], [6,28], [6,29], [6,2], [6,3], [6,4], [6,5], [6,6], [6,7], [6,8], [6,9], [7,0] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bac_win: Option<bool> /* false, true */,
	pub bet_per_line: i64 /* 1, 15 */,
	pub board: Vec<Vec<i64>> /* [[0,4,0],[0,0,1],[0,1,0],[5,0,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus_game_type: Option<i64> /* 0, 1, 2, 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bs_sticky_pos_new_per_spin: Option<Vec<Vec<i64>>> /* [[1,1]] */,
	pub bs_v: Vec<Vec<BonusBsVEnum>> /* [[0,0,0],[0,0,1500],[0,"minor",0],[0,0,0]] */,
	pub bs_values: Vec<Vec<f64>> /* [[0,0,0],[0,0,5],[0,20,0],[0,0,0]] */,
	pub lines: i64 /* 1 */,
	pub mps_: Vec<Vec<i64>> /* [[1,1,1],[1,1,1],[1,1,1],[1,1,1]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mult_new: Option<Vec<Vec<i64>>> /* [[3,2],[1,2]] */,
	pub mults_: Vec<Mults>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub new_bs: Option<Vec<Vec<i64>>> /* [[1,0],[3,2],[1,2]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub paid: Option<bool> /* false, true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub previous_board: Option<Vec<Vec<i64>>> /* [[0,1,0],[0,1,0],[0,1,0],[0,1,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub previous_bs_sticky_pos: Option<Vec<Vec<i64>>> /* [[2,2]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub previous_bs_v: Option<Vec<Vec<BonusBsVEnum>>> /* [[0,20,0],[0,10.0,0],[0,20,0],[0,80,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub previous_bs_values: Option<Vec<Vec<f64>>> /* [[0,1,0],[0,0.5,0],[0,1,0],[0,4,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub progress: Option<Vec<Progress>>,
	pub round_bet: i64 /* 20, 300 */,
	pub round_win: i64 /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<LastargsSelectedModeEnum> /* 1, 2, 3 */,
	pub total_win: i64 /* 0 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Context {
	pub actions: Vec<ContextActionsEnum> /* bonus_init, bonus_spins_stop, buy_spin, respin, spin */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus: Option<Bonus>,
	pub current: ContextCurrentEnum /* bonus, spins */,
	pub last_action: ContextLastActionEnum /* bonus_init, bonus_spins_stop, buy_spin, init, respin, spin */,
	pub last_args: LastArgs,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_win: Option<i64> /* 0 */,
	pub round_finished: bool /* false, true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub spins: Option<Spins>,
	pub version: i64 /* 1 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct OriginData {
	pub autogame: bool /* true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub command: Option<String> /* play */,
	pub feature: bool /* false, true */,
	pub mobile: String /* 0 */,
	pub portrait: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "c684aeb9-2d0a-48cc-a741-397ebda1956a" */,
	pub quickspin: i64 /* 2 */,
	pub set_denominator: i64 /* 1 */,
	pub sound: bool /* false */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CurrencyFormat {
	pub currency_style: String /* symbol */,
	pub denominator: i64 /* 100 */,
	pub style: String /* money */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Jackpots {
	pub grand: i64 /* 2000 */,
	pub major: i64 /* 50 */,
	pub mini: i64 /* 10 */,
	pub minor: i64 /* 20 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PaytableElem {
	pub multiplier: i64 /* 0 */,
	pub occurrences: i64 /* 4 */,
	#[serde(rename = "type")]
	pub paytable_elem_type: String /* lb */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Paytable {
	#[serde(rename = "0")]
	pub paytable_0: Vec<PaytableElem>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Symbols {
	pub id: i64 /* 0, 1, 2, 3, 4, 5 */,
	pub name: SymbolsNameEnum /* el_0, el_bonus, el_bonus_sticky, el_collect, el_mystery, el_mystery_jackpot */,
	#[serde(rename = "type")]
	pub symbols_type: SymbolsTypeEnum /* line, scat */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Settings {
	pub bet_factor: Vec<i64> /* [20] */,
	pub bets: Vec<i64> /* [1,2,3,4,5,6,7,8,10,15,20,25,30,40,50,75,100,150,200,250,300] */,
	pub bonus_sticky_symbol_values: Vec<i64> /* [5,6,7,8,9] */,
	pub bonus_symbol_v: Vec<BonusBsVEnum> /* [0.5,1,1.5,2,3,4,"mini","minor","major"] */,
	pub buy_bonus_price: Vec<i64> /* [40,100,200] */,
	pub cols: i64 /* 4 */,
	pub currency_format: CurrencyFormat,
	pub jackpots: Jackpots,
	pub keys_to_unlock_rows: Vec<i64> /* [15,11,8] */,
	pub lines: Vec<i64> /* [1] */,
	pub paytable: Paytable,
	pub respins_granted: i64 /* 3 */,
	pub rows: i64 /* 3 */,
	pub symbols: Vec<Symbols>,
	pub symbols_scat: Vec<i64> /* [1,2,3,4,5] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Status {
	pub code: StatusCodeEnum /* FUNDS_EXCEED, GAME_REOPENED, OK, PLAYER_DISCONNECTED */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub traceback: Option<String> /* NOT_ENOUGH_MONEY */,
	#[serde(rename = "type", skip_serializing_if = "Option::is_none")]
	pub status_type: Option<StatusTypeEnum> /* crit, exceed */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<i64> /* -1 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
	pub balance: i64 /* 100000 */,
	pub balance_version: i64 /* 1 */,
	pub currency: String /* FUN */,
	pub huid: String /* "demo-e55b3f9a0a5f4e42ac8114faaf0d413f" */,
	pub show_balance: bool /* true */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Server {
	pub command: ServerCommandEnum /* login, play, start, sync */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub context: Option<Context>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modes: Option<Vec<ServerModesEnum>> /* auto, freebet, play */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_data: Option<OriginData>,
	pub request_id: String /* "62c3fd19-2e34-4905-8519-89e7f4236bc7" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roundnum: Option<String> /* "2505181000005461430" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub server_ver: Option<String> /* 1.44.11-9348d0f1 */,
	pub session_id: String /* "577040c7bf0b4dc18036a41bc4527fb7" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub settings: Option<Settings>,
	pub status: Status,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<User>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<i64> /* -1 */,
}

impl Server {
	pub fn new() -> Self {
		Self::default()
	}
}

