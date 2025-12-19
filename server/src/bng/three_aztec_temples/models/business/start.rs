use actix_web::Error;
//use super::super::super::settings::{BOARD_HEIGHT, BOARD_WIDTH, };
use crate::bng::three_aztec_temples::models::server::{Bac, Context, LastArgs, Spins};
use super::super::{server, network::request};
use super::super::enums::{CurrentActionsEnum, ActionsEnum, StatusCodesEnum, MultiValueEnum, };
use super::super::mock::MockData;

pub async fn execute(a_request: &request::start::Start, a_game: &mut server::Server, _is_test: bool, _a_mock_data: &MockData, ) -> Result<(), Error> {
    a_game.command = a_request.command.clone();
    a_game.request_id = a_request.request_id.clone();
    if a_game.context.is_none() {a_game.context = Some(Context {
        actions: vec![ActionsEnum::Spin, ActionsEnum::BuySpin], 
        bonus: None, 
        current: CurrentActionsEnum::Spins, 
        last_action: ActionsEnum::Init, 
        last_args: LastArgs { bet_factor: None, bet_per_line: None, lines: None, selected_mode: None } ,
        last_win: None, 
        round_finished: true, 
        spins: Spins {
            bac: Bac { bac_1: vec![1,0], bac_2: vec![1,0], bac_3: vec![1,0], ..Default::default() },
            bac_pos: None, 
            bac_win: None, 
            bet_per_line: 1, 
            board: vec![vec![6,6,13],vec![1,12,7],vec![2,2,11],vec![10,10,10],vec![9,9,9]], 
            bonus_mechanic: None, 
            bs_count: Some(6), 
            bs_v: vec![vec![MultiValueEnum::Int(0),MultiValueEnum::Int(0),MultiValueEnum::Int(0)],
                       vec![MultiValueEnum::Int(0),MultiValueEnum::Int(0),MultiValueEnum::Int(0)],
                       vec![MultiValueEnum::Int(0),MultiValueEnum::Int(0),MultiValueEnum::Int(0)],
                       vec![MultiValueEnum::String("major".to_string()),MultiValueEnum::Int(40),MultiValueEnum::Int(160)],
                       vec![MultiValueEnum::Int(0),MultiValueEnum::Int(0),MultiValueEnum::Int(0)]], 
            bs_values: vec![vec![MultiValueEnum::Int(0),MultiValueEnum::Int(0),MultiValueEnum::Int(0)],
                            vec![MultiValueEnum::Int(0),MultiValueEnum::Int(0),MultiValueEnum::Int(0)],
                            vec![MultiValueEnum::Int(0),MultiValueEnum::Int(0),MultiValueEnum::Int(0)],
                            vec![MultiValueEnum::Int(100),MultiValueEnum::Int(2),MultiValueEnum::Int(8)],
                            vec![MultiValueEnum::Int(0),MultiValueEnum::Int(0),MultiValueEnum::Int(0)]], 
            lines: 25, 
            lucky_spin_win: None, 
            origin_board: None, 
            round_bet: 20, 
            round_win: 0, 
            selected_mode: None, 
            total_win: Some(0), 
            winlines: None,
            //board_is_executed: vec![vec![false; BOARD_HEIGHT]; BOARD_WIDTH],
        },
        version: 1 
    })}
    //a_game.roundnum = None;
    //a_game.modes 
    // a_game.server_ver
    // a_game.session_id
    //a_game.settings
    a_game.status.set(StatusCodesEnum::Ok, None, None, None);
    //if let Some(ref mut l_user) = a_game.user {l_user.balance_version += 1;};
    // a_game.user_id

    Ok(())
}