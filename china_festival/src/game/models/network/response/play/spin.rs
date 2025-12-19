use serde::{Deserialize, Serialize};
use crate::game::models::model;
use crate::game::settings::{BOARD_HEIGHT, BOARD_WIDTH};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Spin {
    pub command: String,
    pub context: Context,
    pub modes: Vec<String>,
    pub origin_data: OriginData,
    pub request_id: String,
    pub session_id: String,
    pub status: Status,
    pub user: User,
}

impl From<model::Game> for Spin {
    fn from(obj: model::Game) -> Self {
        Spin {
            command: obj.command,
            context: obj.context.into(),
            modes: obj.modes,
            origin_data: obj.origin_data.into(),
            request_id: obj.request_id,
            session_id: obj.session_id,
            status: obj.status.into(),
            user: obj.user.into()
        }
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
    pub spins: Spins,
    pub version: i32,
}

impl From<model::Context> for Context {
    fn from(obj: model::Context) -> Self {
        Context {
            actions: obj.actions,
            current: obj.current,
            last_action: obj.last_action,
            last_args: obj.last_args.into(),
            last_win: obj.last_win,
            round_finished: obj.round_finished,
            spins: obj.spins.into(),
            version: obj.version
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct LastArgs {
    pub bet_per_line: Option<i32>,
    pub lines: Option<i32>,
}

impl From<model::LastArgs> for LastArgs {
    fn from(obj: model::LastArgs) -> Self {
        LastArgs {
            bet_per_line: obj.bet_per_line,
            lines: obj.lines,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Spins {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<bool>,
    pub bet_per_line: i32,
    pub lines: i32,
    pub round_bet: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spin_type: Option<i32>, 
    pub bs_count: i32, 
    pub is_lucky_spin: bool, 
    pub bac: BacInner,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bac_win: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonus_mechanic: Option<Vec<i32>>, 
    pub board: [[i32; BOARD_HEIGHT]; BOARD_WIDTH],
    pub round_win: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_win: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winlines: Option<Vec<Winline>>,
}

impl From<model::Spins> for Spins {
    fn from(obj: model::Spins) -> Self {
        Spins {
            paid: obj.paid,
            bet_per_line: obj.bet_per_line,
            lines: obj.lines,
            round_bet: obj.round_bet,
            spin_type: obj.spin_type,
            bs_count: obj.bs_count,
            is_lucky_spin: obj.is_lucky_spin,
            bac: obj.bac.into(),
            bac_win: obj.bac_win,
            bonus_mechanic: obj.bonus_mechanic,
            board: obj.board,
            round_win: obj.round_win,
            total_win: obj.total_win,
            winlines: obj.winlines.map(|vec| vec.into_iter().map(Into::into).collect()),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct BacInner {
    #[serde(rename = "1")]
    pub field1: [i32; 2],
    #[serde(rename = "2")]
    pub field2: [i32; 2],
    #[serde(rename = "3")]
    pub field3: [i32; 2],
}

impl From<model::BacInner> for BacInner {
    fn from(obj: model::BacInner) -> Self {
        BacInner {
            field1: obj.field1,
            field2: obj.field2,
            field3: obj.field3
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct BoostValue {
    pub bs_v: serde_json::Value,
    pub pos: [i32; 2],
}

impl From<model::BoostValue> for BoostValue {
    fn from(obj: model::BoostValue) -> Self {
        BoostValue {
            bs_v: obj.bs_v,
            pos: obj.pos
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MysteryValue {
    pub bs_v: serde_json::Value,
    pub id: i32,
    pub pos: [i32; 2],
}

impl From<model::MysteryValue> for MysteryValue {
    fn from(obj: model::MysteryValue) -> Self {
        MysteryValue {
            bs_v: obj.bs_v,
            id: obj.id,
            pos: obj.pos
        }
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

impl From<model::Winline> for Winline {
    fn from(obj: model::Winline) -> Self {
        Winline {
            amount: obj.amount,
            line: obj.line,
            occurrences: obj.occurrences,
            positions: obj.positions,
            symbol: obj.symbol,
            winline_type: obj.winline_type
        }
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

impl From<model::OriginData> for OriginData {
    fn from(obj: model::OriginData) -> Self {
        OriginData {
            autogame: obj.autogame,
            feature: obj.feature,
            mobile: obj.mobile,
            portrait: obj.portrait,
            quickspin: obj.quickspin,
            set_denominator: obj.set_denominator,
            sound: obj.sound,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Status {
    pub code: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub status_type: Option<String>
}

impl From<model::Status> for Status {
    fn from(obj: model::Status) -> Self {
        Status {
            code: obj.code,
            status_type: obj.status_type,
        }
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

impl From<model::User> for User {
    fn from(obj: model::User) -> Self {
        User {
            balance: obj.balance,
            balance_version: obj.balance_version,
            currency: obj.currency,
            huid: obj.huid,
            show_balance: obj.show_balance,
        }
    }
}


