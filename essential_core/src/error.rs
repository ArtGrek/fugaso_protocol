//use essential_rand::error::RandError;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

//use crate::account_service::AccountError;

#[derive(Debug, Clone)] // derive std::fmt::Debug on AppError
pub struct ServerError {
    pub line: u32,
    pub file: &'static str,
    pub message: String,
}

pub mod message {
    pub const DB_ERROR: &str = "db level error";
    pub const OPERATOR_ERROR: &str = "operator is not found";
    pub const OPERATOR_CURRENCY_ERROR: &str = "operator currency is not found";
    pub const OPERATOR_ID_ERROR: &str = "user operator id is null";
    pub const OPERATOR_REST_URL_ERROR: &str = "user operator rest url is null";
    pub const OPERATOR_URL_PARSE_ERROR: &str = "operator url parse error";
    pub const USER_SESSION_ERROR: &str = "user session is null";
    pub const CURRENCY_CODE_ERROR: &str = "currency code is null";
    pub const CURRENCY_SYMBOL_ERROR: &str = "currency symbol is null";
    pub const EURO_CURRENCY_ERROR: &str = "EUR currency is not found";
    pub const CURRENCY_OPERATOR_ERROR: &str = "currency operator is not found";
    pub const EXCHANGE_RATE_ERROR: &str = "exchange is not found for";
    pub const CONVERT_PERCENT_ERROR: &str = "error convert percent";
    pub const CONVERT_BET_ERROR: &str = "convert bets error";
    pub const GAME_LOAD_ERROR: &str = "game load error";
    pub const GAME_BET_ERROR: &str = "game bets error";
    pub const GAME_SWEEPSTAKE_ERROR: &str = "game sweepstake error";
    pub const GAME_MATH_ERROR: &str = "game math is not found";
    pub const DIST_EMPTY: &str = "distribution is empty";
    pub const JACKPOT_ERROR: &str = "jackpot not found";
    pub const JACKPOT_MAX_ERROR: &str = "jackpot max none";
    pub const JACKPOT_CONVERT_ERROR: &str = "jackpot convert error";
    pub const JACKPOT_WIN_AMOUNT_ERROR: &str = "jackpot win amount error";
    pub const ILLEGAL_ARGUMENT: &str = "illegal argument";
    pub const ERROR_HMAC_KEY: &str = "error generate hmac key";

    pub const BALANCE_NONE_ERROR: &str = "balance none error";

    pub const ACCOUNT_STATIC_ERROR: &str = "account static is not found";
    pub const ACCOUNT_CASH_OUT_ERROR: &str = "account cash out is not found";
    pub const RESPONSE_STATE_ERROR: &str = "response error state";

    pub fn exchange_rate_msg(code: &Option<String>) -> String {
        format!(
            "{} {}",
            EXCHANGE_RATE_ERROR,
            code.as_ref().unwrap_or(&"--".to_string())
        )
    }

    pub fn exchange_msg(source_id: i64) -> String {
        format!("{} source_id:{}", EXCHANGE_RATE_ERROR, source_id)
    }

    pub fn operator_error(id: i64) -> String {
        format!("{} {}", OPERATOR_ERROR, id)
    }

    pub fn bet_msg(game: &str) -> String {
        format!("{} {}", GAME_BET_ERROR, game)
    }
}

impl ServerError {}

/*fn err(line: u32, file: &'static str, message: &'static str) -> ServerError {
    ServerError {
        line,
        file,
        message: message.to_string(),
    }
}

fn err_msg(line: u32, file: &'static str, message: String) -> ServerError {
    ServerError {
        line,
        file,
        message,
    }
}*/

#[macro_export]
macro_rules! err_on {
    ($expression: expr) => {
        ServerError {
            line: line!(),
            file: file!(),
            message: $expression.to_string(),
        }
    };
}

impl Error for ServerError {}

impl From<ParseIntError> for ServerError {
    fn from(err: ParseIntError) -> Self {
        ServerError {
            line: 0,
            file: file!(),
            message: err.to_string(),
        }
    }
}

/*impl From<AccountError> for ServerError {
    fn from(err: AccountError) -> Self {
        ServerError {
            line: err.line,
            file: err.file,
            message: err.message,
        }
    }
}

impl From<RandError> for ServerError {
    fn from(value: RandError) -> Self {
        Self {
            line: value.line,
            file: value.file,
            message: value.message,
        }
    }
}*/

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /*let err_msg = match self.code {
            404 => "Sorry, Can not find the Page!",
            _ => "Sorry, something is wrong! Please Try Again!",
        };*/
        write!(
            f,
            "file:{} line:{} message:{}",
            self.file, self.line, self.message
        )
    }
}
