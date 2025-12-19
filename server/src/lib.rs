pub mod http_errors;
pub mod games_list;
pub mod bng;
pub mod enj;
use actix_web::{web, HttpResponse};
pub use games_list::LockedList;
pub use games_list::Games;

pub async fn execute_external(a_body: web::Bytes, a_shared_game_list: web::Data<LockedList<Games>>,) -> actix_web::Result<HttpResponse> {
    Ok(bng::actions::execute(a_body, a_shared_game_list, false, &Default::default()).await?)
}

use once_cell::sync::Lazy;
use std::sync::atomic::AtomicUsize;
pub static ROUND_ID_COUNTER: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(2505161000003766632));