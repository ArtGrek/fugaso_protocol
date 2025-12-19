use actix_web::Error;
//use serde_json::Value;
//use rand::Rng;
//use crate::http_errors::http_responses_error;
use super::super::{server, network::request};
//use super::super::super::settings::{BOARD_HEIGHT, BOARD_WIDTH, LINES_COUNT};

pub async fn execute(_a_request: &request::start::Start, _a_game: &mut server::Server) -> Result<(), Error> {

Ok(())
}