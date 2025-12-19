use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::game::settings::{BOARD_HEIGHT, BOARD_WIDTH, BOARD_LINES_COUNT};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Game {
    pub command: String,
    pub context: Context,
    pub modes: Vec<String>,
    pub origin_data: OriginData,
    pub request_id: String,
    pub session_id: String,
    pub settings: Settings,
    pub status: Status,
    pub user: User,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Context {
    pub actions: Vec<String>,
    pub current: String,
    pub last_action: String,
    pub last_args: LastArgs,
    pub last_win: Option<i64>,
    pub round_finished: bool,
    pub bonus: Option<Bonus>,
    pub spins: Spins,
    pub version: i32,
}

impl Context {
    pub fn _new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct LastArgs {
    pub bet_factor:  Option<i32>,
    pub bet_per_line: Option<i32>,
    pub lines: Option<i32>,
    pub selected_mode: Option<String>,
}

impl LastArgs {
    pub fn _new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Bonus {
    pub bet_per_line: i32,
    pub lines: i32,
    pub round_bet: i32,
    pub is_lucky_spin: bool, /* ? */
    pub last_respin: bool, /* ? */
    pub bac: BacInner,
    pub bonus_mechanic: Vec<i32>, /* [boost, double, collect] */
    pub bonus_scenario: i32, /* buy or win */
    pub orig_board: [[i32; BOARD_HEIGHT]; BOARD_WIDTH],
    pub board: [[i32; BOARD_HEIGHT]; BOARD_WIDTH],
    pub board_opened: [[bool; BOARD_HEIGHT]; BOARD_WIDTH],
    pub bs_values: [[f64; BOARD_HEIGHT]; BOARD_WIDTH],
    pub orig_bs_v: [[serde_json::Value; BOARD_HEIGHT]; BOARD_WIDTH],
    pub bs_v: [[serde_json::Value; BOARD_HEIGHT]; BOARD_WIDTH],
    pub bs_count: i32, /* bonus symbol count */
    pub mystery_count: i32,
    pub mystery_pos: Option<Vec<[i32; 2]>>,
    pub mystery_values: Option<Vec<MysteryValue>>,
    pub jackpot_values: Option<[i64; 3]>,
    pub new_bs: Option<Vec<[i32; 2]>>, /* new bonus symbols positions */
    pub copy_new_bs: Option<Vec<[i32; 2]>>, /* total bonus symbols positions */
    pub boost_values: Option<Vec<BoostValue>>,
    pub double_values: Option<Vec<BoostValue>>,
    pub collect_values: Option<Vec<BoostValue>>,
    pub rounds_granted: i32,
    pub rounds_left: i32,
    pub round_win: i64,
    pub total_win: i64,
    pub back_to: String,
}

impl Bonus {
    pub fn _new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Spins {
    pub paid: Option<bool>,
    pub selected_mode: Option<String>,
    pub bet_per_line: i32,
    pub lines: i32,
    pub round_bet: i32,
    pub spin_type: Option<i32>, 
    pub bs_count: i32,
    pub is_lucky_spin: bool, 
    pub bac: BacInner,
    pub bac_win: Option<bool>,
    pub bonus_mechanic: Option<Vec<i32>>, 
    pub board: [[i32; BOARD_HEIGHT]; BOARD_WIDTH],
    pub round_win: i64,
    pub total_win: Option<i64>,
    pub winlines: Option<Vec<Winline>>,
}

impl Spins {
    pub fn _new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct BacInner {
    pub field1: [i32; 2],
    pub field2: [i32; 2],
    pub field3: [i32; 2],
}

impl BacInner {
    pub fn _new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct BoostValue {
    pub bs_v: serde_json::Value,
    pub pos: [i32; 2],
}

impl BoostValue {
    pub fn _new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MysteryValue {
    pub bs_v: serde_json::Value,
    pub id: i32,
    pub pos: [i32; 2],
}

impl MysteryValue {
    pub fn _new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Winline {
    pub amount: i64,
    pub line: i32,
    pub occurrences: i32,
    pub positions: Vec<[i32; 2]>,
    pub symbol: i32,
    #[serde(rename = "type")]
    pub winline_type: String,
}

impl Winline {
    pub fn _new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct OriginData {
    pub autogame: bool,
    pub feature: bool,
    pub mobile: String,
    pub portrait: bool,
    pub quickspin: i32,
    pub set_denominator: i32,
    pub sound: bool,
}

impl OriginData {
    pub fn _new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Settings {
    pub bet_factor: Vec<i32>,
    pub bets: Vec<i32>,
    pub big_win: Vec<i32>,
    pub bonus_symbol_v: Vec<serde_json::Value>,
    pub buy_bonus_price_1: i32,
    pub buy_bonus_price_2: i32,
    pub cols: i32,
    pub currency_format: CurrencyFormat,
    pub jackpots: Jackpots,
    pub lines: Vec<i32>,
    pub payer_values: Vec<i32>,
    pub paylines: [[i32; BOARD_WIDTH]; BOARD_LINES_COUNT],
    pub paytable: HashMap<String, Vec<PaytableEntry>>,
    pub reelsamples: HashMap<String, [Vec<i32>; BOARD_WIDTH]>,
    pub reels: HashMap<String, [Vec<i32>; BOARD_WIDTH]>,
    pub respins_granted: i32,
    pub rows: i32,
    pub symbols: Vec<Symbol>,
    pub symbols_line: Vec<i32>,
    pub symbols_scat: Vec<i32>,
    pub symbols_wild: Vec<i32>,
}

impl Settings {
    pub fn _new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct CurrencyFormat {
    pub currency_style: String,
    pub denominator: i32,
    pub style: String,
}

impl CurrencyFormat {
    pub fn _new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Jackpots {
    pub grand: i64,
    pub major: i64,
    pub minor: i64,
    pub mini: i64,
}

impl Jackpots {
    pub fn _new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct PaytableEntry {
    pub multiplier: i32,
    pub occurrences: i32,
    #[serde(rename = "type")]
    pub entry_type: String,
}

impl PaytableEntry {
    pub fn _new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Symbol {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub symbol_type: String,
}

impl Symbol {
    pub fn _new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Status {
    pub code: String,
    #[serde(rename = "type")]
    pub status_type: Option<String>
}

impl Status {
    pub fn _new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct User {
    pub balance: i64,
    pub balance_version: i32,
    pub currency: String,
    pub huid: String,
    pub show_balance: bool,
}

impl User {
    pub fn new() -> Self {
        Self::default()
    }
}
