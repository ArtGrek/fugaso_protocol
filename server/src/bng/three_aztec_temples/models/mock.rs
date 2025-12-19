use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MockData {
    pub session_id: Option<String>,
    pub huid: Option<String>,
    pub roundnum: Option<String>,
    pub board: Option<Vec<Vec<i64>>>,
    
    pub spins_category: Option<u32>,
    pub spins_board_positions: Option<Vec<u32>>,
    pub spins_coins_appearances: Option<Vec<u32>>,
    pub spins_coins_values: Option<Vec<Vec<u32>>>,
    pub spins_specials_appearances: Option<Vec<u32>>,
    pub spins_bonus_win: Option<u32>,
    pub spins_bac_coins_appearances: Option<Vec<u32>>,
    pub spins_bac_coins_values: Option<Vec<Vec<u32>>>,
    
    pub bonus_category: Option<u32>,
    pub bonus_specials_init_values: Option<Vec<Vec<u32>>>,
    pub bonus_specials_init_mults: Option<Vec<Vec<u32>>>,
    pub bonus_coins_appearances: Option<Vec<Vec<u32>>>,
    pub bonus_coins_values: Option<Vec<Vec<u32>>>,
    pub bonus_specials_appearances: Option<Vec<Vec<u32>>>,
    pub bonus_specials_values: Option<Vec<Vec<u32>>>,
    pub bonus_specials_mults: Option<Vec<Vec<u32>>>,
    pub bonus_mystery_appearances: Option<Vec<Vec<u32>>>,
    pub bonus_mystery_ids: Option<Vec<Vec<u32>>>,
    pub bonus_mystery_values: Option<Vec<Vec<u32>>>,
    pub bonus_mystery_mults: Option<Vec<Vec<u32>>>,

}