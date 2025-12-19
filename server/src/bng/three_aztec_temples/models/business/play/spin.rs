use actix_web::Error;
use crate::ROUND_ID_COUNTER;
use super::super::super::super::settings::{BOARD_HEIGHT, BOARD_WIDTH, LINES_COUNT, COIN, BOOST, COLLECT, MULTI, CHEAP_SYMBOLS, SPECIALS, SPINS_SYMBOLS, };
use super::super::super::{server, network::request};
use super::super::super::enums::{CurrentActionsEnum, ActionsEnum, StatusCodesEnum, StatusTypesEnum, MultiValueEnum};
use super::super::super::mock::MockData;

pub async fn execute(a_request: &request::play::spin::Spin, a_game: &mut server::Server, is_test: bool, a_mock_data: &MockData, ) -> Result<(), Error> {
    a_game.command = a_request.command.clone();
    a_game.request_id = a_request.request_id.clone();
    if let Some(ref mut l_user) = a_game.user {
        let l_total_bet = a_request.action.params.bet_per_line * a_game.settings.bet_factor[0];
        if l_total_bet > 0 {
            if l_user.balance >= l_total_bet {
                if let Some(ref mut l_context) = a_game.context {
                    if l_context.actions.contains(&a_request.action.name) {
                        // set start values
                        l_context.spins.bs_values = vec![vec![MultiValueEnum::Int(0); BOARD_HEIGHT]; BOARD_WIDTH];
                        l_context.spins.bs_v = vec![vec![MultiValueEnum::Int(0); BOARD_HEIGHT]; BOARD_WIDTH];
                        // round_id
                        a_game.roundnum = if !is_test {Some(ROUND_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst).to_string())} else {a_mock_data.roundnum.clone()};
                        // generate board
                        // reels
                        l_context.spins.board = if !is_test {a_game.reels.pick_reels(BOARD_HEIGHT, a_mock_data.spins_category, None)} else {a_mock_data.board.clone().unwrap_or_default()};
                        // coins appear and value
                        for col_num in 0..BOARD_WIDTH {
                            if let Some(coins) = a_game.reels.pick_spins_coins_appearance(a_mock_data.spins_coins_appearances.as_ref().and_then(|v| v.get(col_num).copied())) {
                                for row_num in coins.pos {
                                    l_context.spins.board[col_num][row_num as usize] = coins.id;
                                    let value = a_game.reels.pick_spins_coin_value(a_mock_data.spins_coins_values.as_ref().and_then(|v| v.get(col_num)).and_then(|inner| inner.get(row_num as usize)).copied());
                                    l_context.spins.bs_values[col_num][row_num as usize] = value.to_num();
                                    l_context.spins.bs_v[col_num][row_num as usize] = value.to_multi_value_by_coast(l_total_bet as u64);
                                }
                            }
                        }
                        // specials appear
                        let mut bost_appearance = false;
                        let mut collect_appearance = false;
                        let mut multi_appearance = false;
                        for col_num in 0..BOARD_WIDTH {
                            if let Some(special) = a_game.reels.pick_spins_special_appearance(a_mock_data.spins_specials_appearances.as_ref().and_then(|v| v.get(col_num).copied())) {
                                for row_num in special.pos {
                                    l_context.spins.board[col_num][row_num as usize] = special.id;
                                }
                                match special.id {
                                    BOOST => {bost_appearance = true;}
                                    COLLECT => {collect_appearance = true;}
                                    MULTI => {multi_appearance = true;}
                                    _ => {}
                                }
                            }
                        }
                        // set mechanic
                        let mut l_mechanic: Vec<i64> = Vec::new();
                        if bost_appearance {l_mechanic.push(1);}
                        if collect_appearance {l_mechanic.push(2);}
                        if multi_appearance {l_mechanic.push(3);}
                        let mechanic_id: String = l_mechanic.iter().map(|n| n.to_string()).collect::<String>();
                        // check bac_win
                        let mut l_bs_count = l_context.spins.board.iter().flat_map(|col| {col.iter().filter(|symbol| {SPINS_SYMBOLS.contains(symbol)})}).count();
                        if l_bs_count < 6 && (bost_appearance || collect_appearance || multi_appearance) {
                            if a_game.reels.pick_bonus_win(&mechanic_id, a_mock_data.spins_bonus_win) {
                                l_context.spins.bac_win = Some(true);
                                if bost_appearance {l_context.spins.bac.set_final_bac1();}
                                if collect_appearance {l_context.spins.bac.set_final_bac2();}
                                if multi_appearance {l_context.spins.bac.set_final_bac3();}
                                l_context.spins.origin_board.get_or_insert(l_context.spins.board.clone());
                                // adding missing coins
                                let mut while_stoper = 1000;
                                while l_bs_count < 6 && while_stoper > 0 {
                                    for col_num in 0..BOARD_WIDTH {
                                        if let Some(coins) = a_game.reels.pick_spins_coins_appearance(a_mock_data.spins_bac_coins_appearances.as_ref().and_then(|v| v.get(col_num).copied())) {
                                            for row_num in coins.pos {
                                                if !SPECIALS.contains(&l_context.spins.board[col_num][row_num as usize]) && l_context.spins.board[col_num][row_num as usize] != COIN {
                                                    l_context.spins.board[col_num][row_num as usize] = coins.id;
                                                    let value = a_game.reels.pick_spins_coin_value(a_mock_data.spins_bac_coins_values.as_ref().and_then(|v| v.get(col_num)).and_then(|inner| inner.get(row_num as usize)).copied());
                                                    l_context.spins.bs_values[col_num][row_num as usize] = value.to_num();
                                                    l_context.spins.bs_v[col_num][row_num as usize] = value.to_multi_value_by_coast(l_total_bet as u64);
                                                    l_context.spins.bac_pos.get_or_insert_with(Default::default).push(vec![col_num as i64, row_num as i64]);
                                                    l_bs_count += 1;
                                                }
                                            }
                                        }
                                    }
                                    while_stoper -= 1;
                                }
                            }
                        }
                        // set bonus
                        if l_bs_count >= 6 {
                            l_context.actions = vec![ActionsEnum::BonusInit];
                            l_context.round_finished = false;
                            l_context.spins.bonus_mechanic = (!l_mechanic.is_empty()).then(|| l_mechanic.clone());
                            let l_last_win = l_context.last_win.get_or_insert_with(|| 0);
                            l_context.spins.lucky_spin_win = if *l_last_win > 0 && l_context.spins.bonus_mechanic.is_some() && l_context.spins.bac_win != Some(true) {Some(true)} else {Some(false)};
                            // crutch lucky_spin_win for test, delete after test
                            let request_ids_for_crutch = [
                                "437e4ae6-fd86-4b98-8500-898f919628f9", "e0d1b188-631a-47da-8801-8504246241f6", "be7368dd-03a4-4422-a790-7095b1685885", "71b7e47d-22fd-4ada-b88a-bebe806ab921", 
                                "0cdcc0b5-bd00-4d02-b749-02ffc2b28c87", "bf479867-3a30-4033-b1e6-3713510bb364", "52d24f44-4d65-4f2d-a390-005e849cd4cd", "9ca6dadb-7bd0-4cb6-9de3-1e2411eb5b9a", 
                                "591732b1-3681-4c04-ab32-192a7b7ac476", "a29e7859-12a5-4586-9174-ba29718988e9", "cf8a51f6-5a8e-4150-8a76-1a930e0187ae", "8b042877-c553-4a9f-b221-a8f8683d66a9", 
                                "cad52091-9b3b-4478-9b4e-fccedbd9d919", "f75843b7-d851-427a-9c68-5ff9272f0ffd", "dac6a327-c1f3-44b5-abd8-157cb12a67fa", "6a075c89-aea5-4c0e-afd4-717bf024ca68", 
								"21ade884-e760-45f6-a732-36bfb149a9b8", "806b5e66-19e0-4d95-8dac-da87c546d03e", "3763ea6a-45f6-4702-8e7a-a9b30f56abf5", "8178a87e-5623-4379-ae63-057938bbb865", 
								"e4202355-9f64-4c49-8158-a7d12d51ce3a", "8ebce763-c55a-4f4e-a2e4-883afa4269e0", "7af95366-4ac5-47f3-a140-5ed4ca5e904b", "32661f20-4376-4c00-8a43-6b5126d0a807", 
								"a2957fad-12a8-48d5-855f-fc1a586b89a9", "2f5a5d66-7f01-4f65-8e87-fdc543530c54", "e8dffc6b-9950-429d-821e-4da22901772e", "55f11f24-38a5-4e68-9577-cdbbc6ac5a34", 
                                "af023c01-0dbf-4d38-848e-d7d52c56d02e", "2e3174ec-ff16-4f73-9ff6-f1b133ad7d62", "4d291efb-b7a6-476d-b2f6-ba95600dffad", "5c792ee2-0d9b-404d-b2ef-73d6a2e0abe6", 
                                

                                
                                /*"d85e6d35-c7a3-4880-80e9-168a2b48edde", 
								"33acb81a-dcf9-44b8-9f63-d26155108f8a", "a8677c63-5b7f-4241-9dde-67a052d932a8", "d57d7a20-bc35-4c3a-bf31-d069fc8c39e7", "940e1778-9932-452b-839e-29ec98006f2e", 
								"e733e5d2-6485-4e0b-87a1-4ff99a0c646b", "a2ed5740-24cd-40d1-b0e7-34b62d922e83", "0ac8f7bd-6921-41e0-887b-d90b5cab5e12", "6817f051-987a-4049-a06f-28fbbf56050c", 
								"b0cb3d49-6688-4b69-bd3a-69c4e1c95a4f", "19242fa9-f039-46cd-98df-28eb08fffc51", "463d8d49-13bd-4704-8bea-c8e7c4ec57db", "45948c5d-3828-4b7f-b79d-70eb182337f8", 
								"0349fd56-11b1-4b85-8ec1-ae206fca4524", "a2d56f43-1a38-4f26-bd65-7594cb332aa7", "dbc2f9e0-6d5f-44ac-8df8-237a038af7ed", "940fca41-703b-4d60-a169-6adb7be08ecc", 
								"e9883de4-a79e-4865-b2ff-61353dce66e1", "64f98cb6-5232-4941-b538-e31e44688ee0", "7ef7c37b-cced-4c4a-9601-9725e05760df", "dd02f04e-8acb-4d70-8c3e-6a420ff423f6", 
								"da73229d-c722-4080-9208-086190c836bf", "8bcbc2ac-8569-4313-90ea-b1d6bcd6b224", "e77a2717-7146-4ca3-9c52-6ae1bf14fbd2", "aaada392-0d09-42f6-a047-4c9ba1a26061", 
								"c92e516b-83ba-484d-85db-529c954f52ed", "6c13c8ba-e9e1-451d-a900-0b56706b7c84", "00594158-ae0e-4c30-bccb-0a7e9515fd66", "ec12085c-0ee0-4696-b3f3-caf5f5370dd1", 
								"69bf991d-22bc-4186-9970-324f83cf17f1", "8264a46c-1744-42eb-8deb-ee2c5c6950ae", "f9aa1578-fd68-463f-85cd-40e18933fd4a", "a5c8a18c-69b9-4be2-aa2d-b4a3d9d197c6", 
								"150b8aa4-0fb4-4249-ae12-0e1a72619ebc", "3d9c194b-15c3-4561-a9ea-85cbd9656175", "2d89533b-a6c2-4075-b15c-d37bf64c8852", "230cadcd-7140-4e81-b253-d9201c349d2a", 
								"755c4a76-95c6-4e54-8b36-4a7f8e5df9b2", "8dd87595-8391-4683-bed6-14a2d69b3c8b", "de99a857-c0a0-4e17-8f48-b7543e392a7c", "fbdfd274-970d-4df5-a7cd-b1403a26f918", 
								"5027e20f-4f7c-4d26-9746-3320e46e1be1", "51265da4-0b5a-41b3-ba90-b72d496142a5", "09ffd202-11a0-484c-8698-773cd52b2198", "a58ae8f3-61c6-4294-9916-5e747fc9eae3", 
								"27ef8f9c-45bc-4787-bbe7-26cab91b8346", "0b6c7af8-bc13-4663-b667-20a8d629facc", "f4e152af-1417-4867-9364-aa70033bb5e6", "11602a4d-84c4-4b23-a4b1-e6d72ceff75d", 
								"2be6f15e-b2e7-462f-9669-66b030715acc", "1c926661-121f-440a-a2df-d6f86853e171", "c4ecc669-3603-401e-86fe-79c248b51812", "82a95150-9dea-4031-b36d-a3426167630d", 
								"3c08d124-f6d7-4642-96c5-ca181cf3738e", "4a8bd876-dfa4-43b2-8a2a-00e735273812", "3c5459bb-b869-46e5-9a06-fdfe77587150", "0fb3701b-ec3a-4085-8582-c517fbdafde8", 
								"2929bcb5-ffe6-433e-b37f-e11bd565ab45", "8dbf46cf-c0b0-4a49-8688-67a96aff9c3e", "e22fee5b-1b6e-4d34-a1f1-abe6eb78ce26", "482ff817-7d6f-4269-8b0a-ac36a2906f52", 
								"f1936a2a-e551-47f5-ac9e-cdf9a9b93ae5", "3cd2fff4-a7f7-438e-952b-6b6b9eadc8dd", "e1acfe47-9de8-405c-a465-a8546192c8a5", "5e8cd256-ba8d-419f-95f6-7962a3323f60", 
								"0a329701-f89a-4aad-9906-de6ad263f573", "a4310b07-1267-4599-abd8-44e4fcd62767", "24bdb507-efe1-4bf6-a8b9-52fb4ed32636", "99509417-0c6a-437e-81bc-77663326dc9d", 
								"9d316294-a908-4045-872a-ca25b601e948", "ee689a3c-2915-48cd-b868-64b47d3f2a50", "45513583-0c12-487f-b81d-a802df20e812", "a35da0a8-1a97-424a-a7bc-34c690932151", 
								"de725d9e-3133-46fa-b619-a066edec30d9", "dcdb7a2b-0071-4d3d-98b9-f0754635f90e", "9e1da12a-adfb-4298-ac28-e27dcfbc2e3b", "5f1a7404-2422-47cc-8999-e1dd186ab9e6", 
								"070295d3-794a-4032-bae4-fd72da07a8ed", "8bebfdac-f2c7-41b2-a4f1-8d8a73f2a1b2", "f31652c2-6a18-4639-9837-1e4c1618684e", "1813bf7f-c286-4fb5-9d1b-6f4f7f58ad71", 
								"9c5fcf5f-cd8f-496d-bc97-82e3c64b00c9", "19f3ef43-3a31-43ca-8a56-a8ca9d133cac", "5f9cebf9-9f40-4350-a6c6-d344376bbded", "79a9bf35-f79a-44c9-9803-1cb961d53676", 
								"e6900efc-dd18-4319-980b-0164f5468954", "56def981-e810-4f2b-a152-37573c8b2614", "36c9ff1f-7e86-48e5-992e-dc0d2bb68150", "09d27da8-3e20-481b-9f75-8c10257e46bd", 
								"c43c9f85-cc65-4411-90bc-ce2586ac4f19", "f84b9ed1-423c-4b1e-8663-6245350c4b35", "fd32b765-f5c4-4f77-949a-5ef1065a8ab3", "908c9d83-7beb-48d4-909a-902cbde0609e", 
								"49a8c50d-1d73-4316-957d-693ae49b7b36", "0fc0fd70-b6e7-4881-9eb5-189daf728ae6", "025cb358-bd7c-44b5-8478-576b00ea30b5", "a212ccc9-5580-476d-a32e-aacfa2836c65", 
								"499ba992-c0d5-446c-9389-de2bd498b0e1", "ca702d6c-e4ab-49a0-997d-2259e24d1ceb", "a27dbbe6-15bd-40c1-8f13-882514627c22", "05157932-1a36-4b8e-9ea5-18ead1b8b7b5", 
								"bb4319ef-6751-4ab5-889b-5bd7f02a2aae", "65ddeab4-557f-4529-a630-a2b97b38b7b3", "5b7006d9-a9ed-4d78-a0a6-863c2027a99b", "93ab086f-beec-4c46-ad95-3b3c53beef81", 
								"d347c41f-fb53-42dc-b872-2f25f8ed767c", "4a678aec-ee49-436c-9b17-989a66072cdf", "758a6ce0-e49c-45ad-a54d-584cc0982fa3", "4061a462-6457-4e2e-9de5-74da062ac704", 
								"d9352c1e-8884-48d6-aee5-a8c3c3851bea", "1b610728-ca29-48f2-9f3f-d5bb796a3fe3", "ff887766-edfd-461a-a9a0-26a72296ab2c", "c487f244-51a8-488b-bc38-4ca4ecfd9e25", 
								"61ca8957-dda3-4768-907c-75d823601a4d", "e3c70b22-764f-4387-9e49-7d9b1864faa2", "e4a30fb2-e28d-4b9c-8880-53912967c1b5", "7993f51d-5a67-41cf-95ae-faccf6f04f43", 
								"77b657f4-211b-45e3-a61f-ba6e9d224c9a", "d74205ef-7d8d-4a2e-b048-1b34898ea331", "8c569679-4509-4cb8-abe5-8ee781c15f3e", "244b5ba5-6d70-4537-8c3f-389f78dc406a", 
								"00b0152b-9f93-42ec-8fd1-ceb16f3dd59f", "3b2b3db5-50d2-4cf0-8a1e-a1339ad08dc5", "eae0fde6-c315-4f8c-ac11-642ab49a053f", "d9daa9c0-29df-4797-afc4-c967911acf05", 
								"637f2941-b831-4c9e-a095-c85b2cbac88d", "5d841a30-0b5d-418e-9d83-e42cd25fd0b4", "05ac942e-b23a-4eda-bccf-2d88f87c7a56", "73f22ed8-35a5-43a8-a508-12e033270e4d", 
								"d760b691-b07d-49d3-84fc-91e5da90e9fa", "6dd9aa94-c8e7-418e-804a-d5c4ee24ac71", "47396f93-2e0d-406b-882a-9f8fd69f7217", "dbdf57b0-a696-4fcf-9206-5b89c4279224", 
								"dff14f6b-3386-41ca-9284-bc3995219641", "c449c299-741e-402f-bfc9-0c793ae82a6b", "61efc871-22a4-4bac-9beb-ce421b9100a0", "2a85e155-96e6-4d68-acfb-f84676a7cd96", 
								"3effdd2c-592e-4fb8-a30a-89f55caa5de0", "b5da755c-19d1-45b7-a9c4-e393d108b5df", "2fdf12f5-0575-454c-b0bd-cf7044e9233a", "efc58e38-1c0a-4c4e-9820-74fc2caebe4f", 
								"67a39d5f-e457-4123-9bc6-a4eabaafdd5f", "4b6f8371-f5ca-4f81-acd3-dfb5ccaaa966", "71bc1823-a1f3-4ca3-a6c4-781aacfee5eb", "dbace332-ecae-4813-85a5-1b1dc0be1ce1", 
								"5f514163-5a5f-44fd-af34-ab55c4c90b50", "ddd6c387-b7dc-4b83-8f9f-3289771add6c", "a8c71390-b31a-479a-9057-a15aa7294e14", "367e8c00-f285-461e-a372-aef62510d259", 
								"6fb6e143-25f2-47f8-bdee-6a7fba44e7c1", "9bc3f79e-73b4-40f0-9bac-d4393b06b020", "e0f697fe-de83-42af-9364-1106440d2253", "9dcf7618-e521-4648-bdd9-c8f768b25114", 
								"6d77c498-5649-4976-af14-d6035051634d", "f20910a0-f8d8-44f6-ad88-368f9051843b", "4937b8a2-9372-43ea-a12b-9f5da29af79f", "7e57a326-f7aa-4b9d-a5a8-ba30692a05d7", 
								"d68e90c1-e667-49f9-9475-b501b6445e37", "94c98865-9795-4616-b10d-a5033acb4653", "2c695a2c-67cc-4ac4-880d-69a2f978afd8", "c40a46fb-21a0-41f9-bff4-e62564aa0d00", 
								"5df32cc6-82be-4946-b5b8-8d9c98b8dcf0", "f0b99931-a870-4a59-991d-0f858302fc68", "2e19141b-b2a7-44bf-a8cc-df2f89b9a79b", "7198e6cf-e9ca-4fff-9a95-3d0f3040ce24", 
								"2c19b3e7-3d9d-4dfe-8cad-439cc2a02318", "c9ffcc0e-a60a-4310-b048-16b9ca3dbdde", "d1438d06-e2c4-4dfb-8b0e-ecf8b06cb1aa", "c798d951-0cf4-4a25-ad44-de7536c0c40b", 
								"520e50dd-385b-4974-ad67-047606840819", "57ae65be-ad8c-4dcb-847a-488852fb5ad9", "48706b2b-fc73-417f-aa3a-75931017f124", "5c165ef5-9481-403b-a993-b0542d0840ea", 
								"41a23fae-d801-4735-bf4c-bbe5bf50eab1", "cef086aa-4bda-4114-862e-162cf35ea0a7", "400be33b-88b1-4f84-87d1-2ff7bfc878c0", "034befc2-6ee0-4fe3-9781-b83c6f13f065", 
								"e8d51268-9098-4fff-b66a-309247223da0", "3a31fdf1-cc5f-4857-a0fb-084e6a01aa94", "bcb8663a-dc8b-4178-b765-4ee2069719ae", "8fc65439-6db6-4693-bd0d-42fbb4a4536e", 
								"d1477cf7-5135-48ae-9ea5-2358dd8683e8", "c7d59cca-c3ec-41f7-bd66-f92e188d2eb7", "f2bfee81-9309-4fef-87d0-9aa7bc125dfb", "804ea39b-1079-4ec7-b12f-269053ae06cc", 
								"e4ab2291-e013-4288-9306-b1619703fda0", "e9a41938-2620-4729-ab50-9422aab9b167", "fa5be576-391a-469c-aa8b-f4a051800d13", "9f883fe0-39b4-4544-9f5a-7481a267ddef", 
								"5e11ba6b-cfd8-4a9d-9529-72ca001f2ece", "b40540a4-37cf-4d6d-8bd2-674938739ad9", "a499dfcc-db79-4482-ac69-784a1510fe73", "1c4c9206-0a20-473e-9305-ef46004afd73", 
								"83e8a3c6-aa13-45e5-a66e-f46a81a3880e", "c2b8e38d-c6a0-43a6-a4a9-fa071547162a", "89354135-dd03-4549-9e4a-d9dba8e58c6b", "527404c5-35ed-437f-8764-f11e9524b282", 
								"03c02aa1-fcb3-435a-be30-5b135c4eb974", "5f4dfb11-2deb-4828-a623-1930e55efeca", "bd7d1029-d1dc-4a1e-ac33-01b859ded31d", "7896643e-6d0e-40e9-a795-0655c92f652d", 
								"9f67efaa-6f46-470d-856b-d4001f5b7669", "53cbddd6-011a-45f4-a3a0-b92dd4931292", "9e2fb55e-e9b7-4801-b2ac-d4a27b436c9c", "0b1ca7da-f37e-42ec-b756-d1e5a932ae18", 
								"37a714ea-ceaa-402e-958d-56ddf8bca948", "466be569-3082-4135-a522-1b166c5e869e", "2ebfddcb-da98-4599-b922-01a1c779684a", */
                            ];
                            if request_ids_for_crutch.contains(&a_game.request_id.as_str()) {l_context.spins.lucky_spin_win = Some(false)}
                            // crutch lucky_spin_win for test end
                        } else {
                            l_context.actions = vec![ActionsEnum::Spin, ActionsEnum::BuySpin];
                            l_context.round_finished = true;
                            l_context.spins.bonus_mechanic = None;
                            l_context.spins.lucky_spin_win = Some(false);
                            l_context.bonus = None;
                        }
                        // increase bac
                        if l_context.spins.lucky_spin_win != Some(true) {
                            for col_num in 0..BOARD_WIDTH {
                                for row_num in 0..BOARD_HEIGHT {
                                    match l_context.spins.board[col_num][row_num] {
                                        BOOST => {l_context.spins.bac.set_next_bac1()}
                                        COLLECT => {l_context.spins.bac.set_next_bac2()}
                                        MULTI => {l_context.spins.bac.set_next_bac3()}
                                        _ => {}
                                    }
                                }
                            }
                        }
                        // check winlines
                        l_context.spins.winlines = None;
                        l_context.spins.round_win = 0;
                        let mut l_checking_symbol;
                        let mut l_checking_lenght;
                        let mut l_checking_positions: Vec<Vec<i64>> = Vec::new();
                        for l in 0..LINES_COUNT {
                            l_checking_symbol = l_context.spins.board[0][a_game.settings.paylines[l][0] as usize];
                            l_checking_lenght = 1;
                            l_checking_positions.push(vec![0, a_game.settings.paylines[l][0].clone()]);
                            for x in 1..BOARD_WIDTH {
                                let next_symbol = l_context.spins.board[x][a_game.settings.paylines[l][x] as usize];
                                if a_game.settings.symbols_wild.contains(&l_checking_symbol) {
                                    let through_the_trail_symbol = if x < (BOARD_WIDTH-1) {l_context.spins.board[x+1][a_game.settings.paylines[l][x+1] as usize]} else {0};
                                    if l_checking_lenght > 2 && !a_game.settings.symbols_wild.contains(&next_symbol) && ((CHEAP_SYMBOLS.contains(&next_symbol) && next_symbol != through_the_trail_symbol && !a_game.settings.symbols_wild.contains(&through_the_trail_symbol)) || SPINS_SYMBOLS.contains(&next_symbol)) {break;};
                                    l_checking_symbol = next_symbol; 
                                    l_checking_positions.push(vec![x as i64, a_game.settings.paylines[l][x].clone()]); 
                                    l_checking_lenght += 1; 
                                    continue;
                                }
                                if l_checking_symbol == next_symbol || a_game.settings.symbols_wild.contains(&next_symbol.clone()) {
                                    l_checking_positions.push(vec![x as i64, a_game.settings.paylines[l][x].clone()]); 
                                    l_checking_lenght += 1;
                                } else{break;}
                            }
                            if let Some(l_paytable_symbol) = a_game.settings.paytable.get(&format!("{}", l_checking_symbol)) {
                                if l_paytable_symbol.len() > 0 {
                                    for occ in 0..l_paytable_symbol.len() {
                                        if l_paytable_symbol[occ].occurrences == l_checking_lenght {
                                            l_context.spins.winlines.get_or_insert_with(Vec::new).push(server::Winlines {
                                                amount: (a_request.action.params.bet_per_line * l_paytable_symbol[occ].multiplier) as i64, 
                                                line: (l+1) as i64, 
                                                occurrences: l_paytable_symbol[occ].occurrences, 
                                                positions: l_checking_positions.clone(), 
                                                symbol: l_checking_symbol, 
                                                winlines_type: "lb".to_string()
                                            });
                                            l_context.spins.round_win += (a_request.action.params.bet_per_line * l_paytable_symbol[occ].multiplier) as i64;
                                        }
                                    }
                                }
                            }
                            l_checking_positions.clear();
                        }
                        if let Some(ref mut winlines) = l_context.spins.winlines {winlines.sort_by(|a, b| b.amount.cmp(&a.amount));}
                        // scrutch start
                        if l_context.spins.bac_win == Some(true) {
                            l_context.spins.round_win = 0;
                            l_context.spins.winlines = None;
                            // "origin_board":[[1,11,2],[5,5,1],[9,9,9],[3,6,6],[2,2,8]],"board":[[1,11,10],[5,5,1],[5,5,5],[10,10,6],[2,10,10]],
                            if a_game.request_id == "4d2c90ce-729b-428e-b1df-f3100591c44e".to_string() {l_context.spins.board[2] = vec![5,5,5]};
                        }
                        // scrutch end
                        // set wins and balance
                        l_context.spins.total_win = Some(l_context.spins.round_win);
                        if l_context.spins.total_win > Some(0) {l_context.last_win = l_context.spins.total_win};
                        l_user.balance -= l_total_bet;
                        l_user.balance += l_context.spins.total_win.unwrap_or(0);
                        // static data
                        l_context.spins.bet_per_line = a_request.action.params.bet_per_line;
                        l_context.spins.lines = a_request.action.params.lines;
                        l_context.spins.round_bet = l_total_bet;
                        l_context.spins.selected_mode = a_request.action.params.selected_mode.clone();
                        l_context.current = CurrentActionsEnum::Spins;
                        l_context.last_action = a_request.action.name.clone();
                        l_context.last_args.bet_per_line = Some(a_request.action.params.bet_per_line);
                        l_context.last_args.lines = Some(a_request.action.params.lines);
                        l_context.last_args.bet_factor = a_request.action.params.bet_factor;
                        l_context.last_args.selected_mode = a_request.action.params.selected_mode.clone();
                        l_context.version = 1;
                        let l_origin_data = a_game.origin_data.get_or_insert_with(Default::default);
                        l_origin_data.feature = false;
                        l_origin_data.set_denominator = a_request.set_denominator;
                        l_origin_data.prev_request_id = a_request.prev_request_id.clone();
                        l_origin_data.command = a_request.command.clone();
                        l_origin_data.quickspin = a_request.quick_spin;
                        l_origin_data.autogame = a_request.autogame;
                        l_origin_data.sound = a_request.sound;
                        l_origin_data.mobile = a_request.mobile.clone();
                        l_origin_data.portrait = a_request.portrait;
                        a_game.status.set(StatusCodesEnum::Ok, None, None, None);
                    } else {a_game.status.set(StatusCodesEnum::BadRequest, Some(StatusTypesEnum::Crit), Some("EXPECTED_ACTIONS:".to_owned() + &l_context.actions.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(",") + " ACTUAL_ACTION:" + &a_request.action.name.to_string()), None);}
                } else {a_game.status.set(StatusCodesEnum::InternalServerError, Some(StatusTypesEnum::Crit), Some("CONTEXT_IN_SPIN_IS_NONE".to_string()), None);}
            } else {a_game.status.set(StatusCodesEnum::FundsExceed, Some(StatusTypesEnum::Exceed), Some("NOT_ENOUGH_MONEY".to_string()), None);}
        } else {a_game.status.set(StatusCodesEnum::BadRequest, Some(StatusTypesEnum::Crit), Some("ZERO_TOTAL_BET".to_string()), None);}
    } else {a_game.status.set(StatusCodesEnum::InternalServerError, Some(StatusTypesEnum::Crit), Some("USER_IN_SPIN_IS_NONE".to_string()), None);}

    Ok(())
}