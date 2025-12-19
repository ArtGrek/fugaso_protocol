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
pub enum SymbolsTypeEnum {
	#[default]
	#[serde(rename = "line")]
	Line,
	#[serde(rename = "scat")]
	Scat,
	#[serde(rename = "wild")]
	Wild,
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
	#[serde(rename = "OTHER_ERROR")]
	OtherError,
	#[serde(rename = "PLAYER_DISCONNECTED")]
	PlayerDisconnected,
	#[serde(rename = "SERVER_ERROR")]
	ServerError,
	#[serde(rename = "SESSION_LOST")]
	SessionLost,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum StatusTracebackEnum {
	#[default]
	#[serde(rename = "NOT_ENOUGH_MONEY")]
	NotEnoughMoney,
	#[serde(rename = "crit (0) PlayerGUID is empty on connect [parsePlayerGUID(/opt/source/server/modules/common_server/cm/TCmConnect.cpp:452)]")]
	Crit0PlayerguidIsEmptyOnConnectParseplayerguidOptSourceServerModulesCommonServerCmTcmconnectCpp452,
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
pub struct Bac {
	#[serde(rename = "1")]
	pub bac_1: Vec<i64> /* [0,0], [1,0], [1,1], [10,0], [10,10], [10,11], [10,12], [10,13], [10,14], [10,15], [10,16], [10,17], [10,18], [10,19], [10,1], [10,2], [10,3], [10,4], [10,5], [10,6], [10,7], [10,8], [10,9], [11,0], [11,10], [11,11], [11,12], [11,13], [11,14], [11,15], [11,16], [11,17], [11,18], [11,19], [11,1], [11,20], [11,21], [11,22], [11,23], [11,24], [11,2], [11,3], [11,4], [11,5], [11,6], [11,7], [11,8], [11,9], [12,0], [12,10], [12,11], [12,12], [12,13], [12,14], [12,15], [12,16], [12,17], [12,18], [12,19], [12,1], [12,20], [12,21], [12,22], [12,23], [12,24], [12,25], [12,26], [12,27], [12,28], [12,29], [12,2], [12,3], [12,4], [12,5], [12,6], [12,7], [12,8], [12,9], [13,0], [2,0], [2,1], [2,2], [3,0], [3,1], [3,2], [3,3], [4,0], [4,1], [4,2], [4,3], [5,0], [5,1], [5,2], [5,3], [5,4], [6,0], [6,1], [6,2], [6,3], [6,4], [6,5], [7,0], [7,1], [7,2], [7,3], [7,4], [7,5], [7,6], [7,7], [8,0], [8,1], [8,2], [8,3], [8,4], [8,5], [8,6], [8,7], [8,8], [8,9], [9,0], [9,10], [9,11], [9,12], [9,13], [9,14], [9,1], [9,2], [9,3], [9,4], [9,5], [9,6], [9,7], [9,8], [9,9] */,
	#[serde(rename = "2")]
	pub bac_2: Vec<i64> /* [0,0], [1,0], [1,1], [10,0], [10,10], [10,11], [10,12], [10,13], [10,14], [10,15], [10,16], [10,17], [10,18], [10,19], [10,1], [10,2], [10,3], [10,4], [10,5], [10,6], [10,7], [10,8], [10,9], [11,0], [11,10], [11,11], [11,12], [11,13], [11,14], [11,15], [11,16], [11,17], [11,18], [11,19], [11,1], [11,20], [11,21], [11,22], [11,23], [11,24], [11,2], [11,3], [11,4], [11,5], [11,6], [11,7], [11,8], [11,9], [12,0], [12,10], [12,11], [12,12], [12,13], [12,14], [12,15], [12,16], [12,17], [12,18], [12,19], [12,1], [12,20], [12,21], [12,22], [12,23], [12,24], [12,25], [12,26], [12,27], [12,28], [12,29], [12,2], [12,3], [12,4], [12,5], [12,6], [12,7], [12,8], [12,9], [13,0], [2,0], [2,1], [2,2], [3,0], [3,1], [3,2], [3,3], [4,0], [4,1], [4,2], [4,3], [5,0], [5,1], [5,2], [5,3], [5,4], [6,0], [6,1], [6,2], [6,3], [6,4], [6,5], [7,0], [7,1], [7,2], [7,3], [7,4], [7,5], [7,6], [7,7], [8,0], [8,1], [8,2], [8,3], [8,4], [8,5], [8,6], [8,7], [8,8], [8,9], [9,0], [9,10], [9,11], [9,12], [9,13], [9,14], [9,1], [9,2], [9,3], [9,4], [9,5], [9,6], [9,7], [9,8], [9,9] */,
	#[serde(rename = "3")]
	pub bac_3: Vec<i64> /* [0,0], [1,0], [1,1], [10,0], [10,10], [10,11], [10,12], [10,13], [10,14], [10,15], [10,16], [10,17], [10,18], [10,19], [10,1], [10,2], [10,3], [10,4], [10,5], [10,6], [10,7], [10,8], [10,9], [11,0], [11,10], [11,11], [11,12], [11,13], [11,14], [11,15], [11,16], [11,17], [11,18], [11,19], [11,1], [11,20], [11,21], [11,22], [11,23], [11,24], [11,2], [11,3], [11,4], [11,5], [11,6], [11,7], [11,8], [11,9], [12,0], [12,10], [12,11], [12,12], [12,13], [12,14], [12,15], [12,16], [12,17], [12,18], [12,19], [12,1], [12,20], [12,21], [12,22], [12,23], [12,24], [12,25], [12,26], [12,27], [12,28], [12,29], [12,2], [12,3], [12,4], [12,5], [12,6], [12,7], [12,8], [12,9], [13,0], [2,0], [2,1], [2,2], [3,0], [3,1], [3,2], [3,3], [4,0], [4,1], [4,2], [4,3], [5,0], [5,1], [5,2], [5,3], [5,4], [6,0], [6,1], [6,2], [6,3], [6,4], [6,5], [7,0], [7,1], [7,2], [7,3], [7,4], [7,5], [7,6], [7,7], [8,0], [8,1], [8,2], [8,3], [8,4], [8,5], [8,6], [8,7], [8,8], [8,9], [9,0], [9,10], [9,11], [9,12], [9,13], [9,14], [9,1], [9,2], [9,3], [9,4], [9,5], [9,6], [9,7], [9,8], [9,9] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BoostValues {
	pub bs_v: i64 /* 15000 */,
	pub pos: Vec<i64> /* [2,1] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CollectValues {
	pub bs_v: f64 /* 117000.0 */,
	pub pos: Vec<i64> /* [4,1] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MysteryValues {
	pub bs_v: BonusBsVEnum /* major, mini, minor */,
	pub id: i64 /* 11, 12, 13, 14 */,
	pub pos: Vec<i64> /* [1,0] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bonus {
	pub bac: Bac,
	pub back_to: String /* spins */,
	pub bet_per_line: i64 /* 1, 150 */,
	pub board: Vec<Vec<i64>> /* [[14,3,3],[1,14,1],[2,11,3],[14,14,8],[4,13,14]] */,
	pub bonus_mechanic: Vec<i64> /* [1,2,3], [1,2], [1,3], [1], [2,3], [2], [3] */,
	pub bonus_scenario: i64 /* 0, 1, 2 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub boost_values: Option<Vec<BoostValues>>,
	pub bs_count: i64 /* 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 */,
	pub bs_v: Vec<Vec<BonusBsVEnum>> /* [[19500.0,0,0],[0,19500.0,0],[0,15000,0],[19500.0,19500.0,0],[0,117000.0,24000]] */,
	pub bs_values: Vec<Vec<f64>> /* [[6.5,0,0],[0,6.5,0],[0,5,0],[6.5,6.5,0],[0,39.0,8]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub collect_values: Option<Vec<CollectValues>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub copy_new_bs: Option<Vec<Vec<i64>>> /* [[3,0],[0,0],[3,1],[4,2],[1,1],[2,1],[4,1]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub double_values: Option<Vec<CollectValues>>,
	pub is_lucky_spin: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jackpot_values: Option<Vec<i64>> /* [300,600,2000], [45000,90000,300000] */,
	pub last_respin: bool /* false */,
	pub lines: i64 /* 25 */,
	pub mystery_count: i64 /* 0, 1, 2, 3, 4, 5 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mystery_pos: Option<Vec<Vec<i64>>> /* [[1,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mystery_values: Option<Vec<MysteryValues>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub new_bs: Option<Vec<Vec<i64>>> /* [[3,0],[0,0],[3,1],[4,2],[1,1]] */,
	pub orig_board: Vec<Vec<i64>> /* [[3,3,3],[1,1,1],[2,11,3],[5,8,8],[4,13,5]] */,
	pub orig_bs_v: Vec<Vec<BonusBsVEnum>> /* [[4500.0,0,0],[0,4500.0,0],[0,15000,0],[4500.0,4500.0,0],[0,0,9000]] */,
	pub round_bet: i64 /* 20, 3000 */,
	pub round_win: i64 /* 0 */,
	pub rounds_granted: i64 /* 3 */,
	pub rounds_left: i64 /* 0, 1, 2, 3 */,
	pub total_win: i64 /* 0 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LastArgs {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_factor: Option<i64> /* 20 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_per_line: Option<i64> /* 1, 150 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lines: Option<i64> /* 25 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<LastargsSelectedModeEnum> /* 1, 2 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Winlines {
	pub amount: i64 /* 3000 */,
	pub line: i64 /* 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25 */,
	pub occurrences: i64 /* 3, 4, 5 */,
	pub positions: Vec<Vec<i64>> /* [[0,1],[1,1],[2,1],[3,1],[4,1]] */,
	pub symbol: i64 /* 1, 2, 3, 4, 5, 6, 7, 8, 9 */,
	#[serde(rename = "type")]
	pub winlines_type: String /* lb */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Spins {
	pub bac: Bac,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bac_win: Option<bool> /* false, true */,
	pub bet_per_line: i64 /* 1, 12, 15, 150 */,
	pub board: Vec<Vec<i64>> /* [[2,2,1],[3,6,6],[9,9,9],[2,8,8],[5,2,2]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus_mechanic: Option<Vec<i64>> /* [1,2,3], [1,2], [1,3], [1], [2,3], [2], [3] */,
	pub bs_count: i64 /* 0 */,
	pub is_lucky_spin: bool /* false */,
	pub lines: i64 /* 25 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub paid: Option<bool> /* false, true */,
	pub round_bet: i64 /* 20, 240, 300, 3000 */,
	pub round_win: i64 /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<LastargsSelectedModeEnum> /* 1, 2 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub spin_type: Option<i64> /* 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub total_win: Option<i64> /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub winlines: Option<Vec<Winlines>>,
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
	pub spins: Spins,
	pub version: i64 /* 1 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct OriginData {
	pub autogame: bool /* false, true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub command: Option<String> /* play */,
	pub feature: bool /* false, true */,
	pub mobile: String /* 0 */,
	pub portrait: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "0dcf8e72-6bc6-4411-9c5a-990b4060e741" */,
	pub quickspin: i64 /* 0, 2 */,
	pub set_denominator: i64 /* 1 */,
	pub sound: bool /* false, true */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CurrencyFormat {
	pub currency_style: String /* symbol */,
	pub denominator: i64 /* 100 */,
	pub style: String /* money */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Jackpots {
	pub grand: i64 /* 5000 */,
	pub major: i64 /* 100 */,
	pub mini: i64 /* 15 */,
	pub minor: i64 /* 30 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PaytableElem {
	pub multiplier: i64 /* 2, 4, 20 */,
	pub occurrences: i64 /* 3, 4, 5 */,
	#[serde(rename = "type")]
	pub paytable_elem_type: String /* lb */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Paytable {
	#[serde(rename = "1")]
	pub paytable_1: Vec<PaytableElem>,
	#[serde(rename = "2")]
	pub paytable_2: Vec<PaytableElem>,
	#[serde(rename = "3")]
	pub paytable_3: Vec<PaytableElem>,
	#[serde(rename = "4")]
	pub paytable_4: Vec<PaytableElem>,
	#[serde(rename = "5")]
	pub paytable_5: Vec<PaytableElem>,
	#[serde(rename = "6")]
	pub paytable_6: Vec<PaytableElem>,
	#[serde(rename = "7")]
	pub paytable_7: Vec<PaytableElem>,
	#[serde(rename = "8")]
	pub paytable_8: Vec<PaytableElem>,
	#[serde(rename = "9")]
	pub paytable_9: Vec<PaytableElem>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Reelsamples {
	pub buy_feature: Vec<Vec<i64>> /* [[1,2,3,4,5,6,7,8,9,11,12,13],[1,2,3,4,5,6,7,8,9,11,12,13],[1,2,3,4,5,6,7,8,9,11,12,13],[1,2,3,4,5,6,7,8,9,11,12,13],[1,2,3,4,5,6,7,8,9,11,12,13]] */,
	pub spins: Vec<Vec<i64>> /* [[1,2,3,4,5,6,7,8,9,11,12,13],[1,2,3,4,5,6,7,8,9,10,11,12,13],[1,2,3,4,5,6,7,8,9,10,11,12,13],[1,2,3,4,5,6,7,8,9,10,11,12,13],[1,2,3,4,5,6,7,8,9,10,11,12,13]] */,
	pub spins_0: Vec<Vec<i64>> /* [[1,2,3,4,5,6,7,8,9],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_1: Vec<Vec<i64>> /* [[1,5,9],[2,10,6,7],[8,3,4],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_10: Vec<Vec<i64>> /* [[4],[10,4],[10,4],[10,4],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_11: Vec<Vec<i64>> /* [[5],[10,5],[10,5],[10,5],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_12: Vec<Vec<i64>> /* [[3,4,5,6,7,8,9],[1,2,3,6,8,9,10],[1,2,4,5,6,7,10],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_13: Vec<Vec<i64>> /* [[1,4,5,6,7,8,9],[1,2,3,4,7,9,10],[2,3,5,6,7,8,10],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_14: Vec<Vec<i64>> /* [[1,2,5,6,7,8,9],[1,2,3,4,5,8,10],[3,4,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_15: Vec<Vec<i64>> /* [[1,2,3,6,7,8,9],[2,3,4,5,6,9,10],[1,4,5,7,8,9,10],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_2: Vec<Vec<i64>> /* [[1,5,9],[2,6,7],[8,10,3,4],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_3: Vec<Vec<i64>> /* [[8,3,6],[1,10,4,9],[2,5,7],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_4: Vec<Vec<i64>> /* [[8,3,6],[1,4,9],[2,10,5,7],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_5: Vec<Vec<i64>> /* [[2,4,7],[8,10,3,5],[1,9,6],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_6: Vec<Vec<i64>> /* [[2,4,7],[8,3,5],[1,10,9,6],[1,2,3,4,5,6,7,8,9,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_7: Vec<Vec<i64>> /* [[1],[1,10],[1,10],[1,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_8: Vec<Vec<i64>> /* [[2],[2,10],[2,10],[2,10],[1,2,3,4,5,6,7,8,9,10]] */,
	pub spins_9: Vec<Vec<i64>> /* [[3],[10,3],[10,3],[10,3],[1,2,3,4,5,6,7,8,9,10]] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Symbols {
	pub id: i64 /* 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 */,
	pub name: String /* el_01, el_02, el_03, el_04, el_05, el_06, el_07, el_08, el_09, el_bonus, el_bonus_feature_1, el_bonus_feature_2, el_bonus_feature_3, el_bonus_mystery, el_wild */,
	#[serde(rename = "type")]
	pub symbols_type: SymbolsTypeEnum /* line, scat, wild */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Settings {
	pub bet_factor: Vec<i64> /* [20] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bets: Option<Vec<i64>> /* [1,2,3,4,5,6,7,8,10,12,15,20,25,30,35,40,45,50,75,100,125,150], [1,2,3,4,5,6,7,9,10,15,16,20,25,30,35,50,55,75,80,100,125] */,
	pub big_win: Vec<i64> /* [15,20,30,50] */,
	pub bonus_symbol_v: Vec<BonusBsVEnum> /* [0.5,1,1.5,2,3,5,6,8,10,"mini","minor","major"] */,
	pub buy_bonus_price_1: i64 /* 100 */,
	pub buy_bonus_price_2: i64 /* 300 */,
	pub cols: i64 /* 5 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub currency_format: Option<CurrencyFormat>,
	pub jackpots: Jackpots,
	pub lines: Vec<i64> /* [25] */,
	pub payer_values: Vec<i64> /* [1,2,3,5,6,8,10,15,30,100] */,
	pub paylines: Vec<Vec<i64>> /* [[1,1,1,1,1],[0,0,0,0,0],[2,2,2,2,2],[0,1,2,1,0],[2,1,0,1,2],[1,0,0,0,1],[1,2,2,2,1],[0,0,1,2,2],[2,2,1,0,0],[1,2,1,0,1],[1,0,1,2,1],[0,1,1,1,0],[2,1,1,1,2],[0,1,0,1,0],[2,1,2,1,2],[1,1,0,1,1],[1,1,2,1,1],[0,0,2,0,0],[2,2,0,2,2],[0,2,2,2,0],[2,0,0,0,2],[1,2,0,2,1],[1,0,2,0,1],[0,2,0,2,0],[2,0,2,0,2]] */,
	pub paytable: Paytable,
	pub reelsamples: Reelsamples,
	pub respins_granted: i64 /* 3 */,
	pub rows: i64 /* 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rtp: Option<String> /* 95.70% */,
	pub symbols: Vec<Symbols>,
	pub symbols_line: Vec<i64> /* [1,2,3,4,5,6,7,8,9] */,
	pub symbols_scat: Vec<i64> /* [11,12,13,14,15] */,
	pub symbols_wild: Vec<i64> /* [10] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Status {
	pub code: StatusCodeEnum /* FUNDS_EXCEED, GAME_REOPENED, OK, OTHER_ERROR, PLAYER_DISCONNECTED, SERVER_ERROR, SESSION_LOST */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub traceback: Option<StatusTracebackEnum> /* NOT_ENOUGH_MONEY, crit (0) PlayerGUID is empty on connect [parsePlayerGUID(/opt/source/server/modules/common_server/cm/TCmConnect.cpp:452)] */,
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
	pub huid: String /* "demo-88fb2ae9a9fb434c9407a6322c941377" */,
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
	pub request_id: String /* "7df5da8e-afbd-4969-9c85-d67386ade182" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roundnum: Option<String> /* "2505161000003735594" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub server_ver: Option<String> /* 1.44.11-9348d0f1 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session_id: Option<String> /* "54d657fdddea4c76800b216371ea868e" */,
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

