use fugaso_core::protocol::PlayerRequest;
use fugaso_math::{math::Request, protocol::DatabaseStore};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FuGaSoTuple<S: DatabaseStore + Default + Serialize + Sync + Send, R: Default> {
    #[serde(rename = "in")]
    pub input: PlayerRequest<Request>,
    #[serde(rename = "out")]
    pub output: Vec<fugaso_core::protocol::Response<S, R>>,
}