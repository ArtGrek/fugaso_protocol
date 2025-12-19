use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rand::Rng;

use super::models::enums::MultiValueEnum;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Reels {
    pub reels: HashMap<String, Vec<Vec<i64>>>,
    pub coins_appearance: HashMap<String, AppearanceItem>,
    pub coin_value: HashMap<String, MultiValueEnum>,
    pub special_appearance: HashMap<String, AppearanceItem>,
    pub bonus_win: HashMap<String, u32>,
    pub bonus_init: HashMap<String, HashMap<String, SpecialData>>,
    pub bonus_respin: HashMap<String, HashMap<String, HashMap<String, BonusRespinItem>>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AppearanceItem {
    pub id: i64,
    pub pos: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BonusRespinItem {
    pub coins: HashMap<String, SpecialData>,
    pub specials: HashMap<String, SpecialData>,
    pub mystery: HashMap<String, MysterytData>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MysterytData {
    pub id: i64,
    pub symbols: HashMap<String, SpecialData>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SpecialData {
    pub id: Option<i64>,
    pub mult: Option<HashMap<String, i64>>,
    pub value: HashMap<String, MultiValueEnum>,
}

impl Reels {
    pub fn pick_reels(&self, heght: usize, mock_category: Option<u32>, mock_board_positions: Option<Vec<u32>>, ) -> Vec<Vec<i64>> {
        let reels = match self.reels.pick(mock_category) {Some(reels) => reels, None => return Vec::new(),};
        let mut boards = Vec::with_capacity(reels.len());
        for (reel_num, reel) in reels.iter().enumerate() {
            let len = reel.len();
            if len == 0 {boards.push(Vec::new()); continue;}
            let start = match mock_board_positions {
                Some(ref mock_board_positions) => {mock_board_positions[reel_num] as usize}, 
                None => {rand::thread_rng().gen_range(0..len)}
            };
            let mut board = Vec::with_capacity(heght);
            for i in 0..heght {board.push(reel[(start + i) % len]);}
            boards.push(board);
        }
        boards
    }
    pub fn pick_spins_coins_appearance(&self, mock_rand_num: Option<u32>, ) -> Option<AppearanceItem> {self.coins_appearance.pick(mock_rand_num).cloned()}
    pub fn pick_spins_coin_value(&self, mock_rand_num: Option<u32>, ) -> MultiValueEnum {self.coin_value.pick(mock_rand_num).cloned().unwrap_or_default()}
    pub fn pick_spins_special_appearance(&self, mock_rand_num: Option<u32>, ) -> Option<AppearanceItem> {self.special_appearance.pick(mock_rand_num).cloned()}
    
    pub fn pick_bonus_win(&self, mechanic: &str, mock_rand_num: Option<u32>, ) -> bool {
        let rand_num = if let Some(mock_num) = mock_rand_num { mock_num } else { rand::thread_rng().gen_range(0..10000) };
        self.bonus_win.get(mechanic).and_then(|probability| if *probability > rand_num { Some(true) } else { Some(false) }).unwrap_or(false)
    }

    pub fn pick_bonus_type(&self, mock_rand_num: Option<u32>, ) -> usize {self.bonus_respin.pick_index(mock_rand_num).unwrap_or(1)}
    pub fn pick_bonus_init_value(&self, mechanic: &str, special_id: &str, mock_rand_num: Option<u32>, ) -> MultiValueEnum {
        self.bonus_init.get(mechanic).and_then(|specials| specials.get(special_id)).and_then(|special_data| special_data.value.pick(mock_rand_num).cloned()).unwrap_or_default()
    }
    pub fn pick_bonus_init_multiplayer(&self, mechanic: &str, special_id: &str, mock_rand_num: Option<u32>, ) -> i64 {
        self.bonus_init.get(mechanic).and_then(|specials| specials.get(special_id)).and_then(|special_data| special_data.mult.clone()?.pick(mock_rand_num).cloned()).unwrap_or_default()
    }
    pub fn pick_bonus_respin_coin(&self,bonus_category: usize,mechanic: &str,length: &str,mock_rand_num_a: Option<u32>,mock_rand_num_v: Option<u32>,) -> Option<(i64, MultiValueEnum)> {
        let mut items: Vec<_> = self.bonus_respin.iter().collect();
        items.sort_by_key(|(k, _)| k.parse::<u32>().unwrap_or(u32::MAX));
        let respin_type = items.get(bonus_category - 1).map(|(_, v)| v)?;
        let respin_mechanic = respin_type.get(mechanic)?;
        let respin_length = respin_mechanic.get(length)?;
        let rand_num_coins = if let Some(mock_num) = mock_rand_num_a { mock_num } else { rand::thread_rng().gen_range(0..10000) };
        respin_length.coins.iter()
            .filter_map(|(key_str, value_map)| key_str.parse::<u32>().ok().map(|threshold| (threshold, value_map)))
            .filter(|(threshold, _)| rand_num_coins < *threshold)
            .min_by_key(|(threshold, _)| *threshold)
            .and_then(|(_, coin_data)| {
                let id_val = coin_data.id.clone()?;
                let value = coin_data.value.pick(mock_rand_num_v).cloned().unwrap_or(MultiValueEnum::Int(0));
                Some((id_val, value))
            })
    }
    pub fn pick_bonus_respin_special(&self,bonus_category: usize,mechanic: &str,length: &str,mock_rand_num_a: Option<u32>,mock_rand_num_v: Option<u32>,mock_rand_num_m: Option<u32>,) -> Option<(i64, MultiValueEnum, i64)> {
        let mut items: Vec<_> = self.bonus_respin.iter().collect();
        items.sort_by_key(|(k, _)| k.parse::<u32>().unwrap_or(u32::MAX));
        let respin_type = items.get(bonus_category - 1).map(|(_, v)| v)?;
        let respin_mechanic = respin_type.get(mechanic)?;
        let respin_length = respin_mechanic.get(length)?;
        let rand_num_specials = if let Some(mock_num) = mock_rand_num_a { mock_num } else { rand::thread_rng().gen_range(0..10000) };
        respin_length.specials.iter()
            .filter_map(|(key_str, special_data)| key_str.parse::<u32>().ok().map(|threshold| (threshold, special_data)))
            .filter(|(threshold, _)| rand_num_specials < *threshold)
            .min_by_key(|(threshold, _)| *threshold)
            .and_then(|(_, special_data)| {
                let id_val = special_data.id.clone()?;
                let value = special_data.value.pick(mock_rand_num_v).cloned().unwrap_or(MultiValueEnum::Int(0));
                let mult = special_data.mult.as_ref().and_then(|m| Some(m.pick(mock_rand_num_m).cloned().unwrap_or(0))).unwrap_or(0);
                Some((id_val, value, mult))
            })
    }
    pub fn pick_bonus_respin_mystery(&self,bonus_category: usize,mechanic: &str,length: &str,mock_rand_num_a: Option<u32>,mock_rand_num_i: Option<u32>,mock_rand_num_v: Option<u32>,mock_rand_num_m: Option<u32>,) -> Option<(i64, MultiValueEnum, i64)> {
        let mut items: Vec<_> = self.bonus_respin.iter().collect();
        items.sort_by_key(|(k, _)| k.parse::<u32>().unwrap_or(u32::MAX));
        let respin_type = items.get(bonus_category - 1).map(|(_, v)| v)?;
        let respin_mechanic = respin_type.get(mechanic)?;
        let respin_length = respin_mechanic.get(length)?;
        let rand_num_mystery = if let Some(mock_num) = mock_rand_num_a { mock_num } else { rand::thread_rng().gen_range(0..10000) };
        respin_length.mystery
            .iter()
            .filter_map(|(outer_key, inner_map)| outer_key.parse::<u32>().ok().map(|threshold| (threshold, inner_map)))
            .filter(|(threshold, _)| rand_num_mystery < *threshold)
            .min_by_key(|(threshold, _)| *threshold)
            .and_then(|(_, inner_map)| {
                inner_map.symbols.pick(mock_rand_num_i).and_then(|special_data| {
                    let id_val = special_data.id.clone()?;
                    let value = special_data.value.pick(mock_rand_num_v).cloned().unwrap_or(MultiValueEnum::Int(0));
                    let mult = special_data.mult.as_ref().and_then(|m| Some(m.pick(mock_rand_num_m).cloned().unwrap_or(0))).unwrap_or(0);
                    Some((id_val, value, mult))
                })
            })
    }
}

trait RandomPickByKey {
    type Item;
    fn pick(&self, mock_rand_num: Option<u32>, ) -> Option<&Self::Item>;
    fn pick_index(&self, mock_rand_num: Option<u32>, ) -> Option<usize>;
}

impl<T: std::fmt::Debug> RandomPickByKey for HashMap<String, T> {
    type Item = T;
    fn pick(&self, mock_rand_num: Option<u32>, ) -> Option<&Self::Item> {
        let rand_num = if let Some(mock_num) = mock_rand_num {mock_num} else {rand::thread_rng().gen_range(0..10000)};
        let mut sorted: Vec<(&str, &T)> = self.iter().map(|(k, v)| (k.as_str(), v)).collect();
        sorted.sort_by_key(|(k, _)| k.parse::<u32>().unwrap_or(u32::MAX));
        for (k, v) in sorted {if rand_num < k.parse::<u32>().unwrap_or(0) {return Some(v);}}
        None
    }
    fn pick_index(&self, mock_rand_num: Option<u32>, ) -> Option<usize> {
        let rand_num = if let Some(mock_num) = mock_rand_num {mock_num} else {rand::thread_rng().gen_range(0..10000)};
        let mut sorted: Vec<(&str, &T)> = self.iter().map(|(k, v)| (k.as_str(), v)).collect();
        sorted.sort_by_key(|(k, _)| k.parse::<u32>().unwrap_or(u32::MAX));
        for (idx, (k, _v)) in sorted.iter().enumerate() {
            if rand_num < k.parse::<u32>().unwrap_or(0) {return Some(idx);}
        }
        None
    }
}