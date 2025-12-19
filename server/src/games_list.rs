use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct LockedList<T> {
    pub by_player_game_id: Arc<Mutex<HashMap<String, Arc<Mutex<T>>>>>,
    pub by_session_id:     Arc<Mutex<HashMap<String, Arc<Mutex<T>>>>>,
}

impl<T> LockedList<T> {
    pub fn new() -> Self {
        LockedList {
            by_player_game_id: Arc::new(Mutex::new(HashMap::new())),
            by_session_id:     Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn insert(&self, a_player_game_id: String, a_session_id: String, item: T) {
        let arc_item = Arc::new(Mutex::new(item));
        let mut pid = self.by_player_game_id.lock().await;
        if let Some(old_arc) = pid.insert(a_player_game_id.clone(), arc_item.clone()) {
            let mut sid = self.by_session_id.lock().await;
            if let Some(old_session) = sid.iter().find_map(|(sess, v)| if Arc::ptr_eq(v, &old_arc) { Some(sess.clone()) } else { None }) { sid.remove(&old_session); }
        }
        pid.insert(a_player_game_id, arc_item.clone());
        drop(pid);
        let mut sid = self.by_session_id.lock().await;
        sid.insert(a_session_id, arc_item);
    }

    pub async fn get_by_player_game_id(&self, a_player_game_id: &String, new_session_id: String) -> Option<Arc<Mutex<T>>> {
        let pid = self.by_player_game_id.lock().await;
        let item_opt = pid.get(a_player_game_id).cloned();
        drop(pid);
        if let Some(item_arc) = item_opt.clone() {
            let mut sid = self.by_session_id.lock().await;
            if let Some(old_session) = sid.iter().find_map(|(sess, v)| if Arc::ptr_eq(v, &item_arc) { Some(sess.clone()) } else { None }) { sid.remove(&old_session); }
            sid.insert(new_session_id, item_arc.clone());
        }
        item_opt
    }

    pub async fn get_by_session_id(&self, key: &String) -> Option<Arc<Mutex<T>>> {
        let sid = self.by_session_id.lock().await;
        sid.get(key).cloned()
    }
}


#[derive(Debug, Clone)]
pub enum Games {
    ChinaFestival(crate::bng::china_festival::models::server::Server),
    CoinLamp(crate::bng::coin_lamp::models::server::Server),
    ThreeAztecTemples(crate::bng::three_aztec_temples::models::server::Server),
    ThorHitTheBonus(crate::enj::thor_hit_the_bonus::models::server::Server),
}
