use serde::{Serialize, Deserialize};

use super::enums::{ModesEnum, CommandsEnum, CurrentActionsEnum, ActionsEnum, BonusModesEnum, StatusCodesEnum, StatusTypesEnum, CurrenciesEnum, SymbolsTypesEnum, MultiValueEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bac {
	#[serde(rename = "1")]
	pub bac_1: Vec<i64> /* [1,0], [2,0], [2,1], [3,0], [3,1], [3,2], [4,0], [4,1], [4,2], [4,3], [5,0], [5,1], [5,2], [5,3], [6,0], [6,1], [6,2], [6,3], [7,0], [7,1], [7,2], [7,3], [7,4], [8,0], [8,1], [8,2], [8,3], [8,4], [9,0] */,
	#[serde(rename = "2")]
	pub bac_2: Vec<i64> /* [1,0], [2,0], [2,1], [3,0], [3,1], [3,2], [4,0], [4,1], [4,2], [4,3], [5,0], [5,1], [5,2], [5,3], [6,0], [6,1], [6,2], [6,3], [7,0], [7,1], [7,2], [7,3], [7,4], [8,0], [8,1], [8,2], [8,3], [8,4], [9,0] */,
	#[serde(rename = "3")]
	pub bac_3: Vec<i64> /* [1,0], [2,0], [2,1], [3,0], [3,1], [3,2], [4,0], [4,1], [4,2], [4,3], [5,0], [5,1], [5,2], [5,3], [6,0], [6,1], [6,2], [6,3], [7,0], [7,1], [7,2], [7,3], [7,4], [8,0], [8,1], [8,2], [8,3], [8,4], [9,0] */,
    
	#[serde(skip)]
    pub idx1: usize,
    #[serde(skip)]
    pub idx2: usize,
    #[serde(skip)]
    pub idx3: usize,
}

const BAC_VALUES: &[[i64; 2]] = &[
    [1,0], 
	[2,0], [2,1],
    [3,0], [3,1], [3,2],
    [4,0], [4,1], [4,2], [4,3],
    [5,0], [5,1], [5,2], [5,3],
    [6,0], [6,1], [6,2], [6,3],
    [7,0], [7,1], [7,2], [7,3], [7,4],
    [8,0], [8,1], [8,2], [8,3], [8,4],
    [9,0],
];
impl Bac {
	pub fn set_initial_bac1(&mut self) { self.bac_1 = BAC_VALUES[0].to_vec(); self.idx1 = 0; }
	pub fn set_next_bac1(&mut self) { if self.idx1 + 1 < BAC_VALUES.len() { self.idx1 += 1; self.bac_1 = BAC_VALUES[self.idx1].to_vec(); } }
    pub fn set_final_bac1(&mut self) {self.idx1 = BAC_VALUES.len() - 1; self.bac_1 = BAC_VALUES[self.idx1].to_vec();}

	pub fn set_initial_bac2(&mut self) { self.bac_2 = BAC_VALUES[0].to_vec(); self.idx2 = 0; }
	pub fn set_next_bac2(&mut self) { if self.idx2 + 1 < BAC_VALUES.len() { self.idx2 += 1; self.bac_2 = BAC_VALUES[self.idx2].to_vec(); } }
    pub fn set_final_bac2(&mut self) {self.idx2 = BAC_VALUES.len() - 1; self.bac_2 = BAC_VALUES[self.idx2].to_vec();}

	pub fn set_initial_bac3(&mut self) { self.bac_3 = BAC_VALUES[0].to_vec(); self.idx3 = 0; }
	pub fn set_next_bac3(&mut self) { if self.idx3 + 1 < BAC_VALUES.len() { self.idx3 += 1; self.bac_3 = BAC_VALUES[self.idx3].to_vec(); } }
    pub fn set_final_bac3(&mut self) {self.idx3 = BAC_VALUES.len() - 1; self.bac_3 = BAC_VALUES[self.idx3].to_vec();}

}


#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BoostValues {
	pub bs_v: MultiValueEnum /* 60 */,
	pub pos: Vec<i64> /* [2,1] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CollectValues {
	pub bs_v: MultiValueEnum /* 120.0 */,
	pub pos: Vec<i64> /* [3,1] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MultiValues {
	pub bs_v: MultiValueEnum /* 60 */,
	pub mult_value: i64 /* 2, 3, 5 */,
	pub pos: Vec<i64> /* [1,2] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MysteryValues {
	pub bs_v: MultiValueEnum /* 0 */,
	pub id: i64 /* 10, 11, 12, 13 */,
	pub pos: Vec<i64> /* [0,1] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bonus {
	pub bac: Bac,
	pub back_to: CurrentActionsEnum /* spins */,
	pub bet_per_line: i64 /* 1 */,
	pub board: Vec<Vec<i64>> /* [[10,7,7],[7,7,4],[5,10,10],[4,12,4],[10,10,1]] */,
	pub bonus_game_type: i64 /* 1, 2, 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus_mechanic: Option<Vec<i64>> /* [1,2,3], [1,2], [1,3], [1], [2,3], [2], [3] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub boost_values: Option<Vec<BoostValues>>,
	pub bs_count: i64 /* 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bs_pos: Option<Vec<Vec<i64>>> /* [[0,0],[2,1],[2,2],[3,1],[4,0],[4,1]] */,
	pub bs_v: Vec<Vec<MultiValueEnum>> /* [[10.0,0,0],[0,0,0],[0,20,50.0],[0,120.0,0],[30.0,10.0,0]] */,
	pub bs_values: Vec<Vec<MultiValueEnum>> /* [[0.5,0,0],[0,0,0],[0,1,2.5],[0,6.0,0],[1.5,0.5,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub collect_values: Option<Vec<CollectValues>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub init_bs_count: Option<bool> /* true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jackpot_positions: Option<Vec<Vec<MultiValueEnum>>> /* [[0,0,"mini"],[0,0,0],[0,0,0],[0,0,0],[0,0,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jackpot_values: Option<Vec<i64>> /* [200,400,2000] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jackpots_boost_values: Option<Vec<Vec<i64>>> /* [[0,0,0],[0,0,0],[100,0,0],[0,0,0],[0,0,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jackpots_multiplier_values: Option<Vec<Vec<i64>>> /* [[0,0,2],[0,0,0],[0,0,0],[0,0,0],[0,0,0]] */,
	pub last_respin: bool /* false */,
	pub lines: i64 /* 25 */,
	pub lucky_spin_win: bool /* false, true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub multi_values: Option<Vec<MultiValues>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mystery_values: Option<Vec<MysteryValues>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub new_bs: Option<Vec<Vec<i64>>> /* [[3,2]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_board: Option<Vec<Vec<i64>>> /* [[5,14,10],[4,4,13],[10,4,4],[10,4,4],[10,10,4]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_bs_v: Option<Vec<Vec<MultiValueEnum>>> /* [[10.0,0,0],[0,0,0],[0,20,50.0],[0,0,0],[30.0,10.0,0]] */,
	pub round_bet: i64 /* 20 */,
	pub round_win: i64 /* 0 */,
	pub rounds_granted: i64 /* 3 */,
	pub rounds_left: i64 /* 0, 1, 2, 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<BonusModesEnum> /* 1, 2 */,
	pub total_win: i64 /* 0 */,
	// intermediate
	#[serde(skip)]
	pub board_is_executed: Vec<Vec<bool>>,
	#[serde(skip)]
	pub bs_mults: Vec<Vec<i64>>,
	#[serde(skip)]
	pub origin_bs_values: Vec<Vec<MultiValueEnum>>,
    // crutch
	#[serde(skip)]
	pub spin_bs_values: Vec<Vec<MultiValueEnum>> /* [[0,0,0],[0,0,0],[0,0,0],[100,2,8],[0,0,0]] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LastArgs {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_factor: Option<i64> /* 20 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_per_line: Option<i64> /* 1 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lines: Option<i64> /* 25 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<BonusModesEnum> /* 1, 2 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Winlines {
	pub amount: i64 /* 5 */,
	pub line: i64 /* 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25 */,
	pub occurrences: i64 /* 3, 4, 5 */,
	pub positions: Vec<Vec<i64>> /* [[0,1],[1,1],[2,1]] */,
	pub symbol: i64 /* 1, 2, 3, 4, 5, 6, 7, 8, 9 */,
	#[serde(rename = "type")]
	pub winlines_type: String /* lb */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Spins {
	pub bac: Bac,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bac_pos: Option<Vec<Vec<i64>>> /* [[4,1],[4,0],[3,0],[2,0],[0,2]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bac_win: Option<bool> /* false, true */,
	pub bet_per_line: i64 /* 1, 35 */,
	pub board: Vec<Vec<i64>> /* [[6,6,13],[1,12,7],[2,2,11],[10,10,10],[9,9,9]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus_mechanic: Option<Vec<i64>> /* [1,2,3], [1,2], [1,3], [1], [2,3], [2], [3] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bs_count: Option<i64> /* 6 */,
	pub bs_v: Vec<Vec<MultiValueEnum>> /* [[0,0,0],[0,0,0],[0,0,0],["major",1400,5600],[0,0,0]] */,
	pub bs_values: Vec<Vec<MultiValueEnum>> /* [[0,0,0],[0,0,0],[0,0,0],[100,2,8],[0,0,0]] */,
	pub lines: i64 /* 25 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lucky_spin_win: Option<bool> /* false, true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_board: Option<Vec<Vec<i64>>> /* [[5,5,1],[4,4,13],[9,4,4],[4,4,4],[6,4,4]] */,
	pub round_bet: i64 /* 20, 700 */,
	pub round_win: i64 /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<BonusModesEnum> /* 1, 2 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub total_win: Option<i64> /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub winlines: Option<Vec<Winlines>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Context {
	pub actions: Vec<ActionsEnum> /* bonus_init, bonus_spins_stop, buy_spin, respin, spin */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus: Option<Bonus>,
	pub current: CurrentActionsEnum /* bonus, spins */,
	pub last_action: ActionsEnum /* bonus_init, bonus_spins_stop, buy_spin, init, respin, spin */,
	pub last_args: LastArgs,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_win: Option<i64> /* 0 */,
	pub round_finished: bool /* false, true */,
	pub spins: Spins,
	pub version: i64 /* 1 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct OriginData {
	pub autogame: bool /* true */,
	pub command: CommandsEnum /* play */,
	pub feature: bool /* false, true */,
	pub mobile: String /* 0 */,
	pub portrait: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "ae4106cc-dca2-4910-b2d6-a05f38d71f07" */,
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
	pub major: i64 /* 100 */,
	pub mini: i64 /* 10 */,
	pub minor: i64 /* 20 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PaytableElem {
	pub multiplier: i64 /* 5, 10, 50 */,
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

impl Paytable {
    pub fn get(&self, name: &str) -> Option<&Vec<PaytableElem>> {
        match name {
            "1" => Some(&self.paytable_1),
            "2" => Some(&self.paytable_2),
            "3" => Some(&self.paytable_3),
            "4" => Some(&self.paytable_4),
            "5" => Some(&self.paytable_5),
            "6" => Some(&self.paytable_6),
            "7" => Some(&self.paytable_7),
            "8" => Some(&self.paytable_8),
            "9" => Some(&self.paytable_9),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Reelsamples {
	pub spins: Vec<Vec<i64>> /* [[1,2,3,4,5,6,7,8,9,10,11,12,13],[1,2,3,4,5,6,7,8,9,10,11,12,13],[1,2,3,4,5,6,7,8,9,10,11,12,13],[1,2,3,4,5,6,7,8,9,10,11,12,13],[1,2,3,4,5,6,7,8,9,10,11,12,13]] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Symbols {
	pub id: i64 /* 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14 */,
	pub name: String /* el_01, el_02, el_03, el_04, el_05, el_06, el_07, el_08, el_bonus, el_bonus_boost, el_bonus_collect, el_bonus_multi, el_bonus_mystery, el_wild */,
	#[serde(rename = "type")]
	pub symbols_type: SymbolsTypesEnum /* line, scat, wild */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Settings {
	pub bet_factor: Vec<i64> /* [20] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bets: Option<Vec<i64>> /* [1,2,3,4,5,8,10,15,25,35,50,75,100,150,200,250,300,350,400,450,500] */,
	pub big_win: Vec<i64> /* [15,25,50,80] */,
	pub bonus_symbols: Vec<MultiValueEnum> /* 10.0, 100.0, 1000.0, 10000.0, 10080.0, 1010.0, 10140.0, 1020.0, 10200.0, 10280.0, 1030.0, 10300.0, 10360.0, 1040.0, 10400.0, 10420.0, 1050.0, 10500.0, 10550.0, 10560.0, 1060.0, 10600.0, 10620.0, 10640.0, 10650.0, 1070.0, 1080.0, 10800.0, 1090.0, 10920.0, 10950.0, 110.0, 1100.0, 11000.0, 11020.0, 11060.0, 11090.0, 1110.0, 11100.0, 11120.0, 11140.0, 11150.0, 11160.0, 1120.0, 11220.0, 11280.0, 1130.0, 11300.0, 11360.0, 1140.0, 11400.0, 11420.0, 1150.0, 11540.0, 1160.0, 11600.0, 11640.0, 1170.0, 11700.0, 11760.0, 1180.0, 11850.0, 11880.0, 1190.0, 120.0, 1200.0, 12000.0, 12060.0, 1210.0, 12120.0, 12140.0, 1220.0, 12200.0, 1230.0, 12350.0, 1240.0, 12400.0, 1250.0, 1260.0, 12610.0, 1270.0, 12750.0, 1280.0, 1290.0, 12900.0, 130.0, 1300.0, 13080.0, 1310.0, 1320.0, 13200.0, 1330.0, 1340.0, 13400.0, 1350.0, 13560.0, 1360.0, 13660.0, 13680.0, 1370.0, 1380.0, 13840.0, 1390.0, 140.0, 1400.0, 14000.0, 14060.0, 1410.0, 14100.0, 1420.0, 14200.0, 1430.0, 1440.0, 14400.0, 1450.0, 14520.0, 14560.0, 1460.0, 1470.0, 1480.0, 14820.0, 1490.0, 150.0, 1500.0, 15000.0, 1510.0, 15100.0, 15140.0, 1520.0, 15240.0, 1530.0, 15300.0, 15380.0, 1540.0, 1550.0, 1560.0, 15600.0, 1570.0, 15750.0, 1580.0, 15850.0, 1590.0, 160.0, 1600.0, 16000.0, 1610.0, 1620.0, 1630.0, 16390.0, 1640.0, 16400.0, 16440.0, 1650.0, 16510.0, 16560.0, 1660.0, 16630.0, 1670.0, 1680.0, 1690.0, 16900.0, 170.0, 1700.0, 1710.0, 1720.0, 1730.0, 1740.0, 1750.0, 17500.0, 17550.0, 1760.0, 17620.0, 17690.0, 1770.0, 17700.0, 17780.0, 1780.0, 17820.0, 17830.0, 1790.0, 17940.0, 180.0, 1800.0, 1810.0, 1820.0, 18200.0, 1830.0, 1840.0, 1850.0, 18500.0, 18560.0, 1860.0, 18620.0, 18650.0, 1870.0, 18700.0, 18740.0, 18750.0, 1880.0, 18840.0, 1890.0, 190.0, 1900.0, 1910.0, 1920.0, 1930.0, 1940.0, 1950.0, 1960.0, 1970.0, 19700.0, 1980.0, 1990.0, 20.0, 200.0, 2000.0, 2010.0, 2020.0, 2030.0, 2040.0, 2050.0, 2060.0, 2070.0, 2080.0, 2090.0, 210.0, 2100.0, 2110.0, 2120.0, 2130.0, 2140.0, 2150.0, 2160.0, 2170.0, 2180.0, 2190.0, 220.0, 2200.0, 2210.0, 22150.0, 2220.0, 2230.0, 2240.0, 2250.0, 2260.0, 22600.0, 2270.0, 22720.0, 2280.0, 2290.0, 230.0, 2300.0, 2310.0, 2320.0, 2330.0, 2340.0, 2350.0, 2360.0, 2370.0, 2380.0, 2390.0, 240.0, 2400.0, 2410.0, 2420.0, 2430.0, 2440.0, 2450.0, 2460.0, 2470.0, 2480.0, 2490.0, 250.0, 2500.0, 2510.0, 2520.0, 2530.0, 2540.0, 2550.0, 2560.0, 2570.0, 2580.0, 2590.0, 260.0, 2600.0, 2610.0, 2620.0, 2630.0, 2640.0, 2650.0, 2660.0, 2670.0, 26700.0, 2680.0, 26820.0, 2690.0, 26980.0, 270.0, 2700.0, 2710.0, 27120.0, 2720.0, 27250.0, 2730.0, 27370.0, 2740.0, 2750.0, 2760.0, 2770.0, 2780.0, 2790.0, 280.0, 2800.0, 2810.0, 2820.0, 2830.0, 2840.0, 2850.0, 2860.0, 2870.0, 2880.0, 2890.0, 290.0, 2900.0, 2910.0, 2920.0, 29240.0, 2930.0, 2940.0, 2950.0, 2960.0, 2970.0, 2980.0, 2990.0, 30.0, 300.0, 3000.0, 3010.0, 3020.0, 3030.0, 3040.0, 3050.0, 3060.0, 3070.0, 3080.0, 3090.0, 310.0, 3100.0, 3110.0, 3120.0, 3140.0, 3150.0, 3160.0, 31600.0, 3170.0, 31740.0, 3180.0, 31840.0, 3190.0, 320.0, 3200.0, 3210.0, 3220.0, 32250.0, 3230.0, 32370.0, 3240.0, 3250.0, 3260.0, 3270.0, 3280.0, 3290.0, 330.0, 3300.0, 3310.0, 3320.0, 3330.0, 3340.0, 3350.0, 3360.0, 3370.0, 3380.0, 3390.0, 340.0, 3400.0, 3410.0, 3420.0, 3430.0, 3440.0, 34400.0, 3450.0, 3460.0, 3470.0, 3480.0, 3490.0, 350.0, 3500.0, 3510.0, 3520.0, 3530.0, 3540.0, 3550.0, 3560.0, 3570.0, 3580.0, 3590.0, 360.0, 3600.0, 3610.0, 3620.0, 3630.0, 3640.0, 3650.0, 3660.0, 3670.0, 3680.0, 3690.0, 370.0, 3700.0, 3710.0, 3720.0, 37280.0, 3730.0, 3740.0, 37480.0, 3750.0, 3760.0, 37620.0, 3770.0, 3780.0, 3790.0, 380.0, 3800.0, 3810.0, 3820.0, 3830.0, 3840.0, 3850.0, 3860.0, 3870.0, 3880.0, 3890.0, 390.0, 3900.0, 3920.0, 3930.0, 3940.0, 3950.0, 3960.0, 3970.0, 3980.0, 3990.0, 40.0, 400.0, 4000.0, 4010.0, 4020.0, 4030.0, 4040.0, 4050.0, 4060.0, 4070.0, 4080.0, 4090.0, 410.0, 4100.0, 4110.0, 4120.0, 4130.0, 4140.0, 4150.0, 4160.0, 4170.0, 4180.0, 4190.0, 420.0, 4200.0, 4220.0, 4230.0, 4240.0, 4250.0, 4260.0, 4270.0, 4280.0, 4290.0, 430.0, 4300.0, 4310.0, 4320.0, 4330.0, 4340.0, 4350.0, 4360.0, 4370.0, 4380.0, 4390.0, 440.0, 4400.0, 4410.0, 4420.0, 4430.0, 4440.0, 4450.0, 4460.0, 4480.0, 4490.0, 450.0, 4500.0, 4510.0, 4520.0, 4530.0, 4540.0, 4550.0, 4560.0, 4580.0, 4590.0, 460.0, 4600.0, 4620.0, 4630.0, 4640.0, 4650.0, 4660.0, 4670.0, 4680.0, 4690.0, 470.0, 4700.0, 4710.0, 4720.0, 4730.0, 4740.0, 4750.0, 4760.0, 4770.0, 4780.0, 4790.0, 480.0, 4800.0, 4810.0, 4820.0, 4830.0, 4840.0, 4850.0, 4860.0, 4870.0, 4880.0, 4890.0, 490.0, 4900.0, 4910.0, 4920.0, 4930.0, 4940.0, 4950.0, 4960.0, 4970.0, 4980.0, 4990.0, 50.0, 500.0, 5000.0, 5020.0, 5030.0, 5040.0, 5050.0, 5060.0, 5080.0, 5090.0, 510.0, 5100.0, 5110.0, 5120.0, 5140.0, 5150.0, 5160.0, 5170.0, 5180.0, 5190.0, 520.0, 5200.0, 5220.0, 5230.0, 5240.0, 5250.0, 5260.0, 5270.0, 5280.0, 5290.0, 530.0, 5300.0, 5310.0, 5320.0, 5330.0, 5340.0, 5350.0, 5360.0, 5370.0, 5380.0, 540.0, 5400.0, 5420.0, 5430.0, 5440.0, 5450.0, 5480.0, 550.0, 5500.0, 5510.0, 5520.0, 5530.0, 5540.0, 5550.0, 5560.0, 5580.0, 5590.0, 560.0, 5600.0, 5610.0, 5620.0, 5640.0, 5650.0, 5660.0, 5680.0, 5690.0, 570.0, 5700.0, 5720.0, 5740.0, 5750.0, 5760.0, 5780.0, 5790.0, 580.0, 5800.0, 5820.0, 5840.0, 5850.0, 5860.0, 5870.0, 5880.0, 5890.0, 590.0, 5900.0, 5910.0, 5920.0, 5930.0, 5940.0, 5950.0, 5960.0, 5980.0, 60.0, 600.0, 6000.0, 6010.0, 6020.0, 6030.0, 6040.0, 6050.0, 6060.0, 6070.0, 6080.0, 610.0, 6100.0, 6110.0, 6120.0, 6140.0, 6160.0, 6180.0, 6190.0, 620.0, 6200.0, 6220.0, 6240.0, 6250.0, 6260.0, 6280.0, 6290.0, 630.0, 6300.0, 6320.0, 6340.0, 6360.0, 6380.0, 6390.0, 640.0, 6400.0, 6420.0, 6430.0, 6440.0, 6450.0, 6460.0, 6480.0, 6490.0, 650.0, 6500.0, 6520.0, 6540.0, 6550.0, 6560.0, 6580.0, 660.0, 6600.0, 6610.0, 6620.0, 6640.0, 6650.0, 6660.0, 6680.0, 6690.0, 670.0, 6700.0, 6720.0, 6730.0, 6740.0, 6750.0, 6760.0, 6770.0, 6780.0, 6790.0, 680.0, 6800.0, 6810.0, 6830.0, 6840.0, 6850.0, 6860.0, 6880.0, 6890.0, 690.0, 6900.0, 6920.0, 6940.0, 6950.0, 6960.0, 6970.0, 6980.0, 70.0, 700.0, 7000.0, 7020.0, 7030.0, 7040.0, 7060.0, 7070.0, 7080.0, 710.0, 7100.0, 7120.0, 7130.0, 7150.0, 7160.0, 7170.0, 7180.0, 7190.0, 720.0, 7200.0, 7210.0, 7220.0, 7230.0, 7240.0, 7260.0, 7270.0, 7280.0, 7290.0, 730.0, 7300.0, 7310.0, 7320.0, 7340.0, 7350.0, 7360.0, 7380.0, 740.0, 7400.0, 7420.0, 7430.0, 7440.0, 7460.0, 7470.0, 7480.0, 7490.0, 750.0, 7500.0, 7520.0, 7540.0, 7550.0, 7560.0, 7580.0, 7590.0, 760.0, 7600.0, 7620.0, 7640.0, 7650.0, 7660.0, 770.0, 7720.0, 7760.0, 780.0, 7800.0, 7820.0, 7830.0, 7840.0, 7880.0, 790.0, 7900.0, 7920.0, 7940.0, 7960.0, 7980.0, 7990.0, 80.0, 800.0, 8000.0, 8030.0, 8040.0, 8050.0, 8060.0, 8080.0, 810.0, 8100.0, 8120.0, 8140.0, 8150.0, 8160.0, 8170.0, 820.0, 8200.0, 8270.0, 8280.0, 830.0, 8320.0, 8340.0, 8350.0, 840.0, 8410.0, 8420.0, 8430.0, 8440.0, 850.0, 8500.0, 8520.0, 8540.0, 8550.0, 8560.0, 860.0, 8640.0, 8650.0, 870.0, 880.0, 890.0, 8900.0, 8960.0, 90.0, 900.0, 9000.0, 910.0, 9100.0, 9150.0, 920.0, 9200.0, 930.0, 9300.0, 9360.0, 940.0, 9400.0, 9430.0, 9440.0, 9450.0, 9480.0, 950.0, 9500.0, 9560.0, 960.0, 9600.0, 9680.0, 970.0, 9700.0, 9710.0, 9760.0, 980.0, 9800.0, 9870.0, 990.0, 9920.0, 9950.0, major, mini, minor, 0, 20, 40, 60, 80, 100, 120, 140, 160, 180, 200, 220, 240, 260, 280, 300, 320, 340, 360, 380, 400, 420, 440, 460, 480, 500, 520, 540, 560, 580, 600, 620, 640, 660, 680, 700, 720, 740, 760, 780, 800, 820, 840, 860, 880, 900, 920, 940, 960, 980, 1000, 1020, 1040, 1060, 1080, 1100, 1120, 1140, 1160, 1180, 1200, 1220, 1240, 1260, 1280, 1300, 1320, 1340, 1360, 1380, 1400, 1420, 1440, 1460, 1480, 1500, 1520, 1540, 1560, 1580, 1600, 1620, 1640, 1660, 1680, 1700, 1720, 1740, 1760, 1780, 1800, 1820, 1840, 1860, 1880, 1900, 1920, 1940, 1960, 1980, 2000, 2020, 2040, 2060, 2080, 2100, 2120, 2140, 2160, 2180, 2200, 2220, 2240, 2260, 2280, 2300, 2320, 2340, 2360, 2380, 2400, 2420, 2440, 2460, 2480, 2500, 2520, 2540, 2560, 2580, 2600, 2620, 2640, 2660, 2680, 2700, 2720, 2740, 2760, 2800, 2820, 2840, 2860, 2880, 2900, 2920, 2940, 2960, 2980, 3000, 3020, 3040, 3060, 3080, 3120, 3140, 3160, 3180, 3200, 3220, 3240, 3260, 3280, 3300, 3320, 3340, 3360, 3400, 3420, 3440, 3460, 3480, 3500, 3520, 3540, 3560, 3580, 3600, 3620, 3640, 3660, 3680, 3700, 3720, 3740, 3760, 3780, 3800, 3820, 3840, 3880, 3900, 3920, 3940, 3960, 3980, 4000, 4020, 4040, 4060, 4080, 4100, 4120, 4140, 4160, 4180, 4200, 4220, 4240, 4260, 4280, 4300, 4320, 4340, 4360, 4400, 4420, 4440, 4460, 4480, 4500, 4520, 4560, 4600, 4620, 4640, 4660, 4700, 4720, 4760, 4800, 4820, 4840, 4860, 4880, 4900, 4920, 4940, 4960, 5000, 5020, 5040, 5060, 5080, 5100, 5120, 5140, 5200, 5220, 5260, 5280, 5300, 5400, 5420, 5480, 5500, 5520, 5540, 5600, 5620, 5640, 5660, 5700, 5760, 5800, 5820, 5840, 5900, 5960, 6000, 6040, 6060, 6120, 6140, 6160, 6200, 6220, 6260, 6280, 6360, 6400, 6420, 6440, 6500, 6520, 6600, 6660, 6760, 6860, 6960, 7000, 7060, 7080, 7100, 7160, 7220, 7240, 7300, 7400, 7440, 7500, 7520, 7540, 7580, 7620, 7640, 7700, 7800, 7840, 7880, 7900, 7940, 7980, 8000, 8080, 8120, 8200, 8320, 8800, 8900, 9000, 9040, 9800, 10000, 10140, 10200, 10280, 10340, 10500, 12700, 13200, 13360, 14800, 15300, 15440, 19160, 20100, 20240, 20380, 23500, 23620, 26000, 26120 */,
	pub boost_symbols: Vec<i64> /* [2,3,4,5,6,7,8,10] */,
	pub buy_bonus_price: Vec<i64> /* [100,300] */,
	pub cols: i64 /* 5 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub currency_format: Option<CurrencyFormat>,
	pub default_multiplier: i64 /* 4 */,
	pub jackpots: Jackpots,
	pub lines: Vec<i64> /* [25] */,
	pub multi_symbols: Vec<i64> /* [2,3,4,5] */,
	pub multiplier_values: Vec<i64> /* [2,3,5] */,
	pub paylines: Vec<Vec<i64>> /* [[1,1,1,1,1],[0,0,0,0,0],[2,2,2,2,2],[0,1,2,1,0],[2,1,0,1,2],[1,0,0,0,1],[1,2,2,2,1],[0,0,1,2,2],[2,2,1,0,0],[1,2,1,0,1],[1,0,1,2,1],[0,1,1,1,0],[2,1,1,1,2],[0,1,0,1,0],[2,1,2,1,2],[1,1,0,1,1],[1,1,2,1,1],[0,0,2,0,0],[2,2,0,2,2],[0,2,2,2,0],[2,0,0,0,2],[1,2,0,2,1],[1,0,2,0,1],[0,2,0,2,0],[2,0,2,0,2]] */,
	pub paytable: Paytable,
	pub reelsamples: Reelsamples,
	pub respins_granted: i64 /* 3 */,
	pub rows: i64 /* 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rtp: Option<String> /* 95.70% */,
	pub small_win: i64 /* 3 */,
	pub symbols: Vec<Symbols>,
	pub symbols_line: Vec<i64> /* [1,2,3,4,5,6,7,8] */,
	pub symbols_scat: Vec<i64> /* [10,11,12,13,14] */,
	pub symbols_wild: Vec<i64> /* [9] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Status {
	pub code: StatusCodesEnum /* FUNDS_EXCEED, GAME_REOPENED, OK, PLAYER_DISCONNECTED */,
	#[serde(rename = "type", skip_serializing_if = "Option::is_none")]
	pub status_type: Option<StatusTypesEnum> /* crit, exceed */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub traceback: Option<String> /* NOT_ENOUGH_MONEY */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<i64> /* -1 */,
}

impl Status {
    pub fn set(&mut self, code: StatusCodesEnum, status_type: Option<StatusTypesEnum>, traceback: Option<String>, user_id: Option<i64>,) {
        self.code = code;
        self.status_type = status_type;
        self.traceback = traceback;
        self.user_id = user_id;
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
	pub balance: i64 /* 100000 */,
	pub balance_version: i64 /* 1 */,
	pub currency: CurrenciesEnum /* FUN */,
	pub huid: String /* "demo-106758a99a3346fba872f844aa187a8c" */,
	pub show_balance: bool /* true */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Server {
	pub command: CommandsEnum /* login, play, start, sync */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub context: Option<Context>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modes: Option<Vec<ModesEnum>> /* auto, freebet, play */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_data: Option<OriginData>,
	pub request_id: String /* "28ae5ee1-a824-4634-8358-bcab13e3ce74" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roundnum: Option<String> /* "2505171000004302972" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub server_ver: Option<String> /* 1.44.11-9348d0f1 */,
	pub session_id: String /* "04d1923972bc43a9a629302732728d65" */,
	pub settings: Settings,
	pub status: Status,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<User>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<i64> /* -1 */,
	#[serde(skip)]
    pub reels: super::super::reels::Reels,
	#[serde(skip)]
    pub reels_buy1: super::super::reels::Reels,
	#[serde(skip)]
    pub reels_buy2: super::super::reels::Reels,
}

impl Server {
	pub fn new() -> Self {
		Self::default()
	}
}