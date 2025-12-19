
use actix_web::Error;
use crate::game::models::model;

pub async fn get_account() -> Result<model::User, Error> {
    let l_option_user: Option<model::User> = Some(model::User::new());

    match l_option_user {
        Some(mut l_user) => {
            l_user.balance = 100000;
            l_user.balance_version = 1;
            l_user.currency = "FUN".to_string();
            l_user.huid = "demo-63a5f4c1a4614233a2524d5dc4bc4f8c".to_string();
            l_user.show_balance = true;
            Ok(l_user)
        }
        None => Err(actix_web::error::ErrorInternalServerError("Failed to create account")), 
    }
}
