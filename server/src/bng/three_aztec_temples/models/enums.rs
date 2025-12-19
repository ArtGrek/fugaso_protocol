use serde::{Serialize, Deserialize, Deserializer};
use serde_json::Value;
use strum_macros::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum ModesEnum {
	#[default]
	#[serde(rename = "auto")]
	Auto,
	#[serde(rename = "freebet")]
	Freebet,
	#[serde(rename = "play")]
	Play,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default, PartialEq)]
pub enum CommandsEnum {
	#[default]
	#[serde(rename = "login")]
	Login,
	#[serde(rename = "start")]
	Start,
	#[serde(rename = "play")]
	Play,
	#[serde(rename = "sync")]
	Sync,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum CurrentActionsEnum {
	#[default]
	#[serde(rename = "bonus")]
	Bonus,
	#[serde(rename = "spins")]
	Spins,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default, PartialEq)]
pub enum ActionsEnum {
	#[default]
	#[serde(rename = "spin")]
	Spin,
	#[serde(rename = "buy_spin")]
	BuySpin,
	#[serde(rename = "bonus_init")]
	BonusInit,
	#[serde(rename = "respin")]
	Respin,
	#[serde(rename = "bonus_spins_stop")]
	BonusSpinsStop,
	#[serde(rename = "init")]
	Init,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default, PartialEq)]
pub enum BonusModesEnum {
	#[default]
	#[serde(rename = "0")]
	Enum0,
	#[serde(rename = "1")]
	Enum1,
	#[serde(rename = "2")]
	Enum2,
}

impl BonusModesEnum {
    pub fn as_usize(&self) -> usize {
        match self {
            BonusModesEnum::Enum0 => 0,
            BonusModesEnum::Enum1 => 1,
            BonusModesEnum::Enum2 => 2,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum StatusCodesEnum {
	#[default]
	#[serde(rename = "OK")]
	Ok,
	#[serde(rename = "FUNDS_EXCEED")]
	FundsExceed,
	#[serde(rename = "GAME_REOPENED")]
	GameReopened,
	#[serde(rename = "PLAYER_DISCONNECTED")]
	PlayerDisconnected,
	#[serde(rename = "INTERNAL_ERROR")]
	InternalServerError,
	#[serde(rename = "BAD_REQUEST")]
	BadRequest,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum StatusTypesEnum {
	#[default]
	#[serde(rename = "crit")]
	Crit,
	#[serde(rename = "exceed")]
	Exceed,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum CurrenciesEnum {
	#[default]
	#[serde(rename = "FUN")]
	Fun,
	#[serde(rename = "USD")]
	Usd,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum SymbolsTypesEnum {
	#[default]
	#[serde(rename = "line")]
	Line,
	#[serde(rename = "scat")]
	Scat,
	#[serde(rename = "wild")]
	Wild,
}

use super::super::settings::{MINI_STR, MINOR_STR, MAJOR_STR, MINI_VALUE, MINOR_VALUE, MAJOR_VALUE};

#[derive(Debug, Serialize, Clone, Display, PartialEq)]
#[serde(untagged)]
pub enum MultiValueEnum {
	Float(f64),
	Int(i64),
	String(String),
}

impl Default for MultiValueEnum {
	fn default() -> Self {
		MultiValueEnum::Int(0)
	}
}

impl MultiValueEnum {

    pub fn as_f64(&self) -> f64 {
        match self {
            MultiValueEnum::Float(f) => *f,
            MultiValueEnum::Int(i) => *i as f64,
            MultiValueEnum::String(s) => match s.as_str() {
                MINI_STR => MINI_VALUE as f64,
                MINOR_STR => MINOR_VALUE as f64,
                MAJOR_STR => MAJOR_VALUE as f64,
                _ => 0.0,
            },
        }
    }

    pub fn to_num(&self) -> MultiValueEnum {
        match self {
            MultiValueEnum::Float(f) => MultiValueEnum::Float(*f),
            MultiValueEnum::Int(i) => MultiValueEnum::Int(*i),
            MultiValueEnum::String(s) => match s.as_str() {
            	MINI_STR => MultiValueEnum::Int(MINI_VALUE),
                MINOR_STR => MultiValueEnum::Int(MINOR_VALUE),
                MAJOR_STR => MultiValueEnum::Int(MAJOR_VALUE),
                _ => self.clone(),
            },
        }
    }

    pub fn to_multi_value_by_coast(&self, a_coast: u64) -> MultiValueEnum {
        match self {
            MultiValueEnum::Float(f) => {MultiValueEnum::Float(*f * a_coast as f64)},
            MultiValueEnum::Int(i) => {MultiValueEnum::Int(*i * a_coast as i64)},
            MultiValueEnum::String(s) => MultiValueEnum::String(s.clone()),
            /*MultiValueEnum::String(s) => {
                let bsv = s.clone();
                match s.as_str() {
                    MINI_STR => {*self = MultiValueEnum::Int(MINI_VALUE)},
                    MINOR_STR => {*self = MultiValueEnum::Int(MINOR_VALUE)},
                    MAJOR_STR => {*self = MultiValueEnum::Int(MAJOR_VALUE)},
                    _ => {},
                };
                MultiValueEnum::String(bsv)
            },*/
        }
    }

    pub fn to_num_value_by_coast(&self, a_coast: u64) -> MultiValueEnum {
        match self {
            MultiValueEnum::Float(f) => MultiValueEnum::Float(*f * a_coast as f64),
            MultiValueEnum::Int(i) => MultiValueEnum::Int(*i * a_coast as i64),
            MultiValueEnum::String(s) => match s.as_str() {
            	MINI_STR => MultiValueEnum::Int(MINI_VALUE * a_coast as i64),
                MINOR_STR => MultiValueEnum::Int(MINOR_VALUE * a_coast as i64),
                MAJOR_STR => MultiValueEnum::Int(MAJOR_VALUE * a_coast as i64),
                _ => self.clone(),
            },
        }
    }

}

impl std::ops::Add for MultiValueEnum {
    type Output = MultiValueEnum;

    fn add(self, rhs: Self) -> MultiValueEnum {
        match (self, rhs) {
            (MultiValueEnum::Int(a), MultiValueEnum::Int(b)) => MultiValueEnum::Int(a + b),
            (MultiValueEnum::Float(a), MultiValueEnum::Float(b)) => MultiValueEnum::Float(a + b),
            (MultiValueEnum::Int(a), MultiValueEnum::Float(b)) => MultiValueEnum::Float(a as f64 + b),
            (MultiValueEnum::Float(a), MultiValueEnum::Int(b)) => MultiValueEnum::Float(a + b as f64),
            (MultiValueEnum::String(s), MultiValueEnum::Int(b)) => {
                let value = match s.as_str() {
                    MINI_STR => MINI_VALUE,
                    MINOR_STR => MINOR_VALUE,
                    MAJOR_STR => MAJOR_VALUE,
                    _ => 0,
                };
                MultiValueEnum::Int(value + b)
            }
            (MultiValueEnum::String(s), MultiValueEnum::Float(b)) => {
                let value = match s.as_str() {
                    MINI_STR => MINI_VALUE as f64,
                    MINOR_STR => MINOR_VALUE as f64,
                    MAJOR_STR => MAJOR_VALUE as f64,
                    _ => 0.0,
                };
                MultiValueEnum::Float(value + b)
            }
            (MultiValueEnum::Int(a), MultiValueEnum::String(s)) => {
                let value = match s.as_str() {
                    MINI_STR => MINI_VALUE,
                    MINOR_STR => MINOR_VALUE,
                    MAJOR_STR => MAJOR_VALUE,
                    _ => 0,
                };
                MultiValueEnum::Int(a + value)
            }
            (MultiValueEnum::Float(a), MultiValueEnum::String(s)) => {
                let value = match s.as_str() {
                    MINI_STR => MINI_VALUE as f64,
                    MINOR_STR => MINOR_VALUE as f64,
                    MAJOR_STR => MAJOR_VALUE as f64,
                    _ => 0.0,
                };
                MultiValueEnum::Float(a + value)
            }
            _ => MultiValueEnum::Int(0),
        }
    }
}

impl std::ops::AddAssign for MultiValueEnum {
    fn add_assign(&mut self, other: Self) {
        match (&mut *self, other) {
            (MultiValueEnum::Int(a), MultiValueEnum::Int(b)) => {*a += b},
            (MultiValueEnum::Float(a), MultiValueEnum::Float(b)) => {*a += b},
            (MultiValueEnum::Int(a), MultiValueEnum::Float(b)) => {*self = MultiValueEnum::Float(*a as f64 + b);}
            (MultiValueEnum::Float(a), MultiValueEnum::Int(b)) => {*self = MultiValueEnum::Float(*a + b as f64);}
            (MultiValueEnum::String(s), MultiValueEnum::Int(b)) => {
                let value = match s.as_str() {
                    MINI_STR => MINI_VALUE,
                    MINOR_STR => MINOR_VALUE,
                    MAJOR_STR => MAJOR_VALUE,
                    _ => 0,
                };
                *self = MultiValueEnum::Int(value + b);
            }
            (MultiValueEnum::String(s), MultiValueEnum::Float(b)) => {
                let value = match s.as_str() {
                    MINI_STR => MINI_VALUE as f64,
                    MINOR_STR => MINOR_VALUE as f64,
                    MAJOR_STR => MAJOR_VALUE as f64,
                    _ => 0.0,
                };
                *self = MultiValueEnum::Float(value + b);
            }
            _ => {*self = MultiValueEnum::Int(0);}
        }
    }
}

impl std::ops::Sub for MultiValueEnum {
    type Output = MultiValueEnum;

    fn sub(self, rhs: Self) -> MultiValueEnum {
        match (self, rhs) {
            (MultiValueEnum::Int(a), MultiValueEnum::Int(b)) => MultiValueEnum::Int(a - b),
            (MultiValueEnum::Float(a), MultiValueEnum::Float(b)) => MultiValueEnum::Float(a - b),
            (MultiValueEnum::Int(a), MultiValueEnum::Float(b)) => MultiValueEnum::Float(a as f64 - b),
            (MultiValueEnum::Float(a), MultiValueEnum::Int(b)) => MultiValueEnum::Float(a - b as f64),
            (MultiValueEnum::String(s), MultiValueEnum::Int(b)) => {
                let value = match s.as_str() {
                    MINI_STR => MINI_VALUE,
                    MINOR_STR => MINOR_VALUE,
                    MAJOR_STR => MAJOR_VALUE,
                    _ => 0,
                };
                MultiValueEnum::Int(value - b)
            }
            (MultiValueEnum::String(s), MultiValueEnum::Float(b)) => {
                let value = match s.as_str() {
                    MINI_STR => MINI_VALUE as f64,
                    MINOR_STR => MINOR_VALUE as f64,
                    MAJOR_STR => MAJOR_VALUE as f64,
                    _ => 0.0,
                };
                MultiValueEnum::Float(value - b)
            }
            (MultiValueEnum::Int(a), MultiValueEnum::String(s)) => {
                let value = match s.as_str() {
                    MINI_STR => MINI_VALUE,
                    MINOR_STR => MINOR_VALUE,
                    MAJOR_STR => MAJOR_VALUE,
                    _ => 0,
                };
                MultiValueEnum::Int(a - value)
            }
            (MultiValueEnum::Float(a), MultiValueEnum::String(s)) => {
                let value = match s.as_str() {
                    MINI_STR => MINI_VALUE as f64,
                    MINOR_STR => MINOR_VALUE as f64,
                    MAJOR_STR => MAJOR_VALUE as f64,
                    _ => 0.0,
                };
                MultiValueEnum::Float(a - value)
            }
            _ => MultiValueEnum::Int(0),
        }
    }
}

impl std::ops::SubAssign for MultiValueEnum {
    fn sub_assign(&mut self, other: Self) {
        match (&mut *self, other) {
            (MultiValueEnum::Int(a), MultiValueEnum::Int(b)) => {*a -= b},
            (MultiValueEnum::Float(a), MultiValueEnum::Float(b)) => {*a -= b},
            (MultiValueEnum::Int(a), MultiValueEnum::Float(b)) => {*self = MultiValueEnum::Float(*a as f64 - b);},
            (MultiValueEnum::Float(a), MultiValueEnum::Int(b)) => {*self = MultiValueEnum::Float(*a - b as f64);},
            (MultiValueEnum::String(s), MultiValueEnum::Int(b)) => {
                let value = match s.as_str() {
                    MINI_STR => MINI_VALUE,
                    MINOR_STR => MINOR_VALUE,
                    MAJOR_STR => MAJOR_VALUE,
                    _ => 0,
                };
                *self = MultiValueEnum::Int(value - b);
            }
            (MultiValueEnum::String(s), MultiValueEnum::Float(b)) => {
                let value = match s.as_str() {
                    MINI_STR => MINI_VALUE as f64,
                    MINOR_STR => MINOR_VALUE as f64,
                    MAJOR_STR => MAJOR_VALUE as f64,
                    _ => 0.0,
                };
                *self = MultiValueEnum::Float(value - b);
            }
            _ => {*self = MultiValueEnum::Int(0);}
        }
    }
}

impl std::ops::Mul for MultiValueEnum {
    type Output = MultiValueEnum;

    fn mul(self, rhs: Self) -> MultiValueEnum {
        match (self, rhs) {
            (MultiValueEnum::Int(a), MultiValueEnum::Int(b)) => MultiValueEnum::Int(a * b),
            (MultiValueEnum::Float(a), MultiValueEnum::Float(b)) => MultiValueEnum::Float(a * b),
            (MultiValueEnum::Int(a), MultiValueEnum::Float(b)) => MultiValueEnum::Float(a as f64 * b),
            (MultiValueEnum::Float(a), MultiValueEnum::Int(b)) => MultiValueEnum::Float(a * b as f64),
            (MultiValueEnum::String(s), MultiValueEnum::Int(b)) => {
                let value = match s.as_str() {
                    MINI_STR => MINI_VALUE,
                    MINOR_STR => MINOR_VALUE,
                    MAJOR_STR => MAJOR_VALUE,
                    _ => 0,
                };
                MultiValueEnum::Int(value * b)
            }
            (MultiValueEnum::String(s), MultiValueEnum::Float(b)) => {
                let value = match s.as_str() {
                    MINI_STR => MINI_VALUE as f64,
                    MINOR_STR => MINOR_VALUE as f64,
                    MAJOR_STR => MAJOR_VALUE as f64,
                    _ => 0.0,
                };
                MultiValueEnum::Float(value * b)
            }
            (MultiValueEnum::Int(a), MultiValueEnum::String(s)) => {
                let value = match s.as_str() {
                    MINI_STR => MINI_VALUE,
                    MINOR_STR => MINOR_VALUE,
                    MAJOR_STR => MAJOR_VALUE,
                    _ => 0,
                };
                MultiValueEnum::Int(a * value)
            }
            (MultiValueEnum::Float(a), MultiValueEnum::String(s)) => {
                let value = match s.as_str() {
                    MINI_STR => MINI_VALUE as f64,
                    MINOR_STR => MINOR_VALUE as f64,
                    MAJOR_STR => MAJOR_VALUE as f64,
                    _ => 0.0,
                };
                MultiValueEnum::Float(a * value)
            }
            _ => MultiValueEnum::Int(0),
        }
    }
}

impl std::ops::MulAssign for MultiValueEnum {
    fn mul_assign(&mut self, other: Self) {
        match (&mut *self, other) {
            (MultiValueEnum::Int(a), MultiValueEnum::Int(b)) => {*a *= b},
            (MultiValueEnum::Float(a), MultiValueEnum::Float(b)) => {*a *= b},
            (MultiValueEnum::Int(a), MultiValueEnum::Float(b)) => {*self = MultiValueEnum::Float(*a as f64 * b);}
            (MultiValueEnum::Float(a), MultiValueEnum::Int(b)) => {*self = MultiValueEnum::Float(*a * b as f64);}
            (MultiValueEnum::String(s), MultiValueEnum::Int(b)) => {
                let value = match s.as_str() {
                    MINI_STR => MINI_VALUE,
                    MINOR_STR => MINOR_VALUE,
                    MAJOR_STR => MAJOR_VALUE,
                    _ => 0,
                };
                *self = MultiValueEnum::Int(value * b);
            }
            (MultiValueEnum::String(s), MultiValueEnum::Float(b)) => {
                let value = match s.as_str() {
                    MINI_STR => MINI_VALUE as f64,
                    MINOR_STR => MINOR_VALUE as f64,
                    MAJOR_STR => MAJOR_VALUE as f64,
                    _ => 0.0,
                };
                *self = MultiValueEnum::Float(value * b);
            }
            _ => {*self = MultiValueEnum::Int(0);}
        }
    }
}

impl std::ops::Div<i64> for MultiValueEnum {
    type Output = MultiValueEnum;

    fn div(self, rhs: i64) -> MultiValueEnum {
        match self {
            MultiValueEnum::Int(a) => {
                if a % rhs == 0 {
                    MultiValueEnum::Int(a / rhs)
                } else {
                    MultiValueEnum::Float(a as f64 / rhs as f64)
                }
            }
            MultiValueEnum::Float(a) => {
                let res = a / rhs as f64;
                if res.fract() == 0.0 {
                    MultiValueEnum::Int(res as i64)
                } else {
                    MultiValueEnum::Float(res)
                }
            }
            MultiValueEnum::String(s) => {
                let value = match s.as_str() {
                    MINI_STR => MINI_VALUE,
                    MINOR_STR => MINOR_VALUE,
                    MAJOR_STR => MAJOR_VALUE,
                    _ => 0,
                };
                MultiValueEnum::Int(value) // без деления
            }
        }
    }
}

impl<'de> Deserialize<'de> for MultiValueEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let val = Value::deserialize(deserializer)?;
        match val {
            Value::Number(num) => {
                if num.is_i64() {
                    Ok(MultiValueEnum::Int(num.as_i64().unwrap()))
                } else if num.is_u64() {
                    Ok(MultiValueEnum::Int(num.as_u64().unwrap() as i64))
                } else if num.is_f64() {
                    // Всегда оставлять как Float, если пришло как float!
                    Ok(MultiValueEnum::Float(num.as_f64().unwrap()))
                } else {
                    Err(serde::de::Error::custom("Unknown number type"))
                }
            }
            Value::String(s) => Ok(MultiValueEnum::String(s)),
            _ => Err(serde::de::Error::custom("Unexpected value")),
        }
    }
}

