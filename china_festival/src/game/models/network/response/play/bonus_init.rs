use serde::{Deserialize, Serialize};
use crate::game::models::model;
use crate::game::settings::{BOARD_HEIGHT, BOARD_WIDTH};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct BonusInit {
    pub command: String,
    pub context: Context,
    pub modes: Vec<String>,
    pub origin_data: OriginData,
    pub request_id: String,
    pub session_id: String,
    pub status: Status,
    pub user: User,
}

impl From<model::Game> for BonusInit {
    fn from(obj: model::Game) -> Self {
        BonusInit {
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
    pub bonus: Option<Bonus>,
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
            bonus: obj.bonus.map(Into::into),
            spins: obj.spins.into(),
            version: obj.version
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct LastArgs {}

impl From<model::LastArgs> for LastArgs {
    fn from(_obj: model::LastArgs) -> Self {
        LastArgs {}
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Bonus {
    pub round_bet: i32,
    pub bet_per_line: i32,
    pub lines: i32,
    pub is_lucky_spin: bool, /* ? */
    pub last_respin: bool, /* ? */
    pub bac: BacInner,
    pub bonus_mechanic: Vec<i32>, /* [1-boost, 2-double, 3-collect] */
    pub bonus_scenario: i32, /* 1-buy or 0-win */
    pub orig_board: [[i32; BOARD_HEIGHT]; BOARD_WIDTH],
    pub board: [[i32; BOARD_HEIGHT]; BOARD_WIDTH],
    pub bs_values: [[f64; BOARD_HEIGHT]; BOARD_WIDTH],
    pub orig_bs_v: [[serde_json::Value; BOARD_HEIGHT]; BOARD_WIDTH],
    pub bs_v: [[serde_json::Value; BOARD_HEIGHT]; BOARD_WIDTH],
    pub bs_count: i32, /* bonus symbol count */
    pub mystery_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mystery_pos: Option<Vec<[i32; 2]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mystery_values: Option<Vec<MysteryValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jackpot_values: Option<[i64; 3]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_bs: Option<Vec<[i32; 2]>>, /* new bonus symbols positions */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_new_bs: Option<Vec<[i32; 2]>>, /* total bonus symbols positions */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_values: Option<Vec<BoostValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_values: Option<Vec<BoostValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collect_values: Option<Vec<BoostValue>>,
    pub rounds_granted: i32,
    pub rounds_left: i32,
    pub round_win: i64,
    pub total_win: i64,
    pub back_to: String,
}

impl From<model::Bonus> for Bonus {
    fn from(obj: model::Bonus) -> Self {
        Bonus {
            round_bet: obj.round_bet,
            bet_per_line: obj.bet_per_line,
            lines: obj.lines,
            is_lucky_spin: obj.is_lucky_spin,
            last_respin: obj.last_respin,
            bac: obj.bac.into(),
            bonus_mechanic: obj.bonus_mechanic,
            bonus_scenario: obj.bonus_scenario,
            orig_board: obj.orig_board,
            board: obj.board,
            bs_values: obj.bs_values,
            orig_bs_v: obj.orig_bs_v,
            bs_v: obj.bs_v,
            bs_count: obj.bs_count,
            mystery_count: obj.mystery_count,
            mystery_pos: obj.mystery_pos,
            mystery_values: obj.mystery_values.map(|vec| vec.into_iter().map(Into::into).collect()),
            jackpot_values: obj.jackpot_values,
            new_bs: obj.new_bs,
            copy_new_bs: obj.copy_new_bs,
            boost_values: obj.boost_values.map(|vec| vec.into_iter().map(Into::into).collect()),
            double_values: obj.double_values.map(|vec| vec.into_iter().map(Into::into).collect()),
            collect_values: obj.collect_values.map(|vec| vec.into_iter().map(Into::into).collect()),
            rounds_granted: obj.rounds_granted,
            rounds_left: obj.rounds_left,
            round_win: obj.round_win,
            total_win: obj.total_win,
            back_to: obj.back_to,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Spins {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_mode: Option<String>,
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
    pub winlines: Option<Vec<Winline>>,
}

impl From<model::Spins> for Spins {
    fn from(obj: model::Spins) -> Self {
        Spins {
            paid: obj.paid,
            selected_mode: obj.selected_mode.clone(),
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


