use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rand::Rng;

use super::{MultiValueEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Reels {
    pub categories: HashMap<String, Vec<Vec<i64>>>,
    pub bonus_appearance: HashMap<String, BonusAppearanceItem>,
    pub bonus_win: HashMap<String, i64>,
    pub bonus_init: HashMap<String, HashMap<String, MultiValueEnum>>,
    pub bonus_respin: HashMap<String, HashMap<String, BonusRespinItem>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BonusAppearanceItem {
    pub id: i64,
    pub pos: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BonusRespinItem {
    pub coins: HashMap<String, HashMap<String, MultiValueEnum>>,
    pub symbols: HashMap<String, SymbolData>,
    pub mystery: HashMap<String, HashMap<String, SymbolData>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SymbolData {
    pub id: String,
    pub values: HashMap<String, MultiValueEnum>,
}

impl Reels {
    pub fn pick_reels(&self) -> Option<&Vec<Vec<i64>>> {self.categories.pick()}
    pub fn pick_category_reels(&self, heght: usize) -> Option<(usize, Vec<Vec<i64>>)> {
        let (category, reels) = self.categories.pick_with_index()?;
        let mut boards = Vec::with_capacity(reels.len());
        for reel in reels {
            let len = reel.len();
            if len == 0 { boards.push(Vec::new()); continue; }
            let start = rand::thread_rng().gen_range(0..len);
            let mut board = Vec::with_capacity(heght);
            for i in 0..heght {board.push(reel[(start + i) % len]);}
            boards.push(board);
        }
        Some((category, boards))
    }
    pub fn pick_bonus_appearance(&self) -> Option<&BonusAppearanceItem> {self.bonus_appearance.pick()}
	pub fn pick_bonus_win(&self, mechanic: &str) -> bool {
        let rand_num = rand::thread_rng().gen_range(0..10000);
		self.bonus_win.get(mechanic).map_or(false, |&prob| rand_num < prob)
	}
	pub fn pick_bonus_init(&self, mechanic: &str) -> Option<&MultiValueEnum> {self.bonus_init.get(mechanic)?.pick()}
    pub fn pick_bonus_respin(&self, mechanic: &str, length: &str) -> (Option<&MultiValueEnum>, Option<(&str, &MultiValueEnum)>, Option<(&str, &MultiValueEnum)>) {
        let respin_mechanic = match self.bonus_respin.get(mechanic) {Some(val) => val, None => return (None, None, None),};
        let respin_length = match respin_mechanic.get(length) {Some(val) => val, None => return (None, None, None),};
        let rand_num_coins = rand::thread_rng().gen_range(0..10000);
        let coins_result = respin_length.coins.iter()
            .filter_map(|(key_str, value_map)| key_str.parse::<u32>().ok().map(|threshold| (threshold, value_map)))
            .filter(|(threshold, _)| rand_num_coins < *threshold)
            .min_by_key(|(threshold, _)| *threshold)
            .and_then(|(_, value_map)| value_map.pick());
        let rand_num_symbols = rand::thread_rng().gen_range(0..10000);
        let symbol_result = respin_length.symbols.iter()
            .filter_map(|(key_str, symbol_data)| key_str.parse::<u32>().ok().map(|threshold| (threshold, symbol_data)))
            .filter(|(threshold, _)| rand_num_symbols < *threshold)
            .min_by_key(|(threshold, _)| *threshold)
            .and_then(|(_, symbol_data)| {symbol_data.values.pick().map(|val| (symbol_data.id.as_str(), val))});
        let rand_num_mystery = rand::thread_rng().gen_range(0..10000);
        let mystery_result = respin_length.mystery.iter()
            .filter_map(|(outer_key, inner_map)| outer_key.parse::<u32>().ok().map(|threshold| (threshold, inner_map)))
            .filter(|(threshold, _)| rand_num_mystery < *threshold)
            .min_by_key(|(threshold, _)| *threshold)
            .and_then(|(_, inner_map)| {inner_map.pick().and_then(|symbol_data| {symbol_data.values.pick().map(|val| (symbol_data.id.as_str(), val))})});
        (coins_result, symbol_result, mystery_result)
    }

}

trait RandomPickByKey {
    type Item;
    fn pick(&self) -> Option<&Self::Item>;
    fn pick_with_index(&self) -> Option<(usize, &Self::Item)>;
}

impl<T> RandomPickByKey for HashMap<String, T> {
    type Item = T;
    fn pick(&self) -> Option<&Self::Item> {
        let rand_num = rand::thread_rng().gen_range(0..10000);
        let mut sorted: Vec<(&str, &T)> = self.iter().map(|(k, v)| (k.as_str(), v)).collect();
        sorted.sort_by_key(|(k, _)| k.parse::<u32>().unwrap_or(u32::MAX));
        for (k, v) in sorted {if rand_num < k.parse::<u32>().unwrap_or(0) {return Some(v);}}
        None
    }
    fn pick_with_index(&self) -> Option<(usize, &Self::Item)> {
        let rand_num = rand::thread_rng().gen_range(0..10000);
        let mut sorted: Vec<(&str, &T)> = self.iter().map(|(k, v)| (k.as_str(), v)).collect();
        sorted.sort_by_key(|(k, _)| k.parse::<u32>().unwrap_or(u32::MAX));
        for (idx, (k, v)) in sorted.iter().enumerate() {
            if rand_num < k.parse::<u32>().unwrap_or(0) {return Some((idx, *v));}
        }
        None
    }
}