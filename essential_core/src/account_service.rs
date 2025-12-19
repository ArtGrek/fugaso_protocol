use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum ProxyAlias {
    #[serde(rename = "demo")]
    Demo,
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "external")]
    External,
    #[serde(rename = "sweep_external")]
    SweepExternal,
}

impl ToString for ProxyAlias {
    fn to_string(&self) -> String {
        match self {
            ProxyAlias::Demo => "demo".to_string(),
            ProxyAlias::Static => "static".to_string(),
            ProxyAlias::External => "external".to_string(),
            ProxyAlias::SweepExternal => "sweep_external".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct AccountError {
    pub rc: i32,
    pub line: u32,
    pub file: &'static str,
    pub message: String,
}

impl std::error::Error for AccountError {}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "file:{} line:{} message:{} rc:{}",
            self.file, self.line, self.message, self.rc
        )
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum ErrorType {
    OUT_OF_MONEY_CODE,
    WAGER_NOT_FOUND,
    OPERATION_NOT_ALLOWED,
    NOT_LOGGED_ON,
    TECHNICAL_ERROR,
    ACCOUNT_BLOCKED,
    BET_LIMIT_EXCEEDED,
    OUT_OF_ATTEMPTS,
    GENERAL_CODE,
    UNKNOWN_CURRENCY,
    PARAMETER_REQUIRED,
    UNKNOWN,
    AUTHENTICATION_FAILED,
    CUSTOM_ERROR,
    ADMIN_CLOSE,
}