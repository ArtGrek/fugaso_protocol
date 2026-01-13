
use serde::{Deserialize, Serialize, };
use std::collections::HashMap;
use essential_core::error::ServerError;
use essential_core::err_on;
use crate::protocol::DatabaseStore;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiftItem {
    pub p: (usize, usize),
    pub m: i32,
    pub v: i32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MegaThunderLinkInfo {
    #[serde(default)]
    pub total: i64,
    pub respins: i32,
    #[serde(default)]
    pub accum: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay: Option<Vec<Vec<char>>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mults: Vec<Vec<i32>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lifts: Vec<Vec<i32>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lifts_new: Vec<LiftItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grand: Vec<i32>,
}

impl DatabaseStore for MegaThunderLinkInfo {
    fn from_db(value: &str) -> Result<Self, ServerError> {serde_json::from_str(&value).map_err(|e| err_on!(e))}
    fn to_db(&self) -> Result<String, ServerError> {serde_json::to_string(self).map_err(|e| err_on!(e))}
    fn respins(&self) -> i32 {self.respins}
}

use strum_macros::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum ModesEnum {
	#[default]
	#[serde(rename = "auto")]
	Auto,
	#[serde(rename = "play")]
	Play,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default, PartialEq)]
pub enum CommandEnum {
	#[default]
	#[serde(rename = "login")]
	Login,
	#[serde(rename = "play")]
	Play,
	#[serde(rename = "start")]
	Start,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum ContextActionEnum {
	#[default]
	#[serde(rename = "bonus")]
	Bonus,
	#[serde(rename = "spins")]
	Spins,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default, PartialEq)]
pub enum ActionNameEnum {
	#[default]
	#[serde(rename = "init")]
	Init,
	#[serde(rename = "bonus_init")]
	BonusInit,
	#[serde(rename = "bonus_spins_stop")]
	BonusSpinsStop,
	#[serde(rename = "buy_spin")]
	BuySpin,
	#[serde(rename = "respin")]
	Respin,
	#[serde(rename = "spin")]
	Spin,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum SelectedModeEnum {
	#[default]
	#[serde(rename = "1")]
	Enum1,
	#[serde(rename = "2")]
	Enum2,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display)]
#[serde(untagged)]
pub enum BonusBsVEnum {
	Float(f64),
	Int(i64),
	String(String),
}

impl Default for BonusBsVEnum {
	fn default() -> Self {
		BonusBsVEnum::Int(0)
	}
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum SpinsReelsetNameEnum {
	#[default]
	#[serde(rename = "low")]
	Low,
	#[serde(rename = "mid")]
	Mid,
	#[serde(rename = "special_bonus")]
	SpecialBonus,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum SymbolsTypeEnum {
	#[default]
	#[serde(rename = "line")]
	Line,
	#[serde(rename = "scat")]
	Scat,
	#[serde(rename = "wild")]
	Wild,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum StatusCodeEnum {
	#[default]
	#[serde(rename = "OK")]
	Ok,
	#[serde(rename = "OTHER ERROR")]
	OtherError,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Changes {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub multiplier: Option<i64> /* 2, 3, 5 */,
	pub reel: i64 /* 0, 1, 2, 3, 4 */,
	pub row: i64 /* 0, 1, 2 */,
	pub symbol: i64 /* 10, 11, 12 */,
	pub value: i64 /* 1, 2, 3, 4, 5, 6, 7, 10, 15, 30, 100 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ColumnResults {
	pub reel: i64 /* 0, 1, 2, 3, 4 */,
	pub win: i64 /* 350 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bonus {
	pub _total_win: i64 /* 0, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 242, 243, 244, 245, 247, 248, 249, 250, 251, 252, 253, 254, 255, 256, 257, 258, 260, 261, 262, 263, 264, 265, 266, 267, 268, 269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 282, 283, 284, 285, 286, 288, 289, 290, 291, 292, 293, 295, 296, 297, 298, 299, 300, 301, 304, 305, 306, 307, 308, 309, 311, 312, 313, 315, 316, 317, 319, 320, 321, 322, 324, 325, 327, 328, 329, 330, 336, 337, 338, 339, 342, 343, 344, 345, 346, 347, 348, 350, 351, 352, 355, 357, 358, 359, 360, 361, 362, 363, 364, 365, 366, 367, 370, 371, 372, 373, 374, 376, 378, 380, 382, 383, 385, 386, 388, 389, 390, 392, 393, 394, 396, 398, 403, 405, 409, 410, 412, 413, 414, 416, 418, 419, 421, 424, 427, 431, 432, 434, 436, 437, 438, 439, 440, 442, 443, 445, 446, 447, 449, 455, 456, 457, 458, 461, 464, 469, 472, 474, 476, 479, 481, 484, 485, 486, 487, 492, 493, 495, 499, 500, 502, 503, 510, 519, 520, 524, 540, 544, 546, 547, 552, 561, 564, 567, 571, 572, 575, 578, 581, 592, 599, 602, 605, 608, 614, 619, 627, 629, 630, 633, 635, 645, 646, 650, 651, 656, 662, 666, 674, 675, 676, 682, 684, 687, 691, 697, 707, 714, 720, 723, 727, 735, 737, 742, 749, 751, 754, 759, 776, 790, 797, 803, 805, 815, 819, 853, 856, 858, 864, 893, 896, 898, 921, 931, 937, 956, 957, 975, 980, 1002, 1018, 1019, 1020, 1021, 1023, 1024, 1025, 1026, 1027, 1028, 1029, 1030, 1031, 1032, 1033, 1034, 1036, 1037, 1038, 1039, 1040, 1041, 1042, 1043, 1044, 1045, 1047, 1048, 1050, 1051, 1052, 1053, 1054, 1055, 1056, 1057, 1058, 1062, 1063, 1065, 1067, 1068, 1069, 1070, 1071, 1072, 1073, 1075, 1079, 1080, 1081, 1082, 1083, 1085, 1089, 1095, 1096, 1098, 1099, 1100, 1108, 1109, 1110, 1111, 1115, 1117, 1118, 1119, 1124, 1125, 1126, 1130, 1133, 1138, 1141, 1143, 1144, 1145, 1147, 1154, 1168, 1172, 1174, 1175, 1185, 1202, 1218, 1221, 1224, 1242, 1252, 1253, 1255, 1275, 1289, 1293, 1296, 1389, 1401, 1406, 1412, 1418, 1435, 1437, 1442, 4251 */,
	pub bac: Vec<i64> /* [0,0], [0,1], [0,2], [0,3], [0,4], [0,5], [0,6], [0,7], [0,8], [1,0], [1,10], [1,11], [1,1], [1,2], [1,3], [1,4], [1,5], [1,6], [1,7], [1,8], [1,9], [2,0], [2,10], [2,11], [2,1], [2,2], [2,3], [2,4], [2,5], [2,6], [2,7], [2,8], [2,9], [3,0], [3,10], [3,11], [3,12], [3,13], [3,1], [3,2], [3,3], [3,4], [3,5], [3,6], [3,7], [3,8], [3,9], [4,0], [4,10], [4,11], [4,12], [4,13], [4,14], [4,15], [4,1], [4,2], [4,3], [4,4], [4,5], [4,6], [4,7], [4,8], [4,9], [5,0], [5,10], [5,11], [5,12], [5,13], [5,14], [5,15], [5,16], [5,17], [5,1], [5,2], [5,3], [5,4], [5,5], [5,6], [5,7], [5,8], [5,9], [6,0], [6,10], [6,11], [6,12], [6,13], [6,14], [6,15], [6,16], [6,17], [6,18], [6,19], [6,1], [6,2], [6,3], [6,4], [6,5], [6,6], [6,7], [6,8], [6,9], [7,0], [7,10], [7,11], [7,12], [7,13], [7,14], [7,15], [7,16], [7,17], [7,18], [7,19], [7,1], [7,20], [7,21], [7,2], [7,3], [7,4], [7,5], [7,6], [7,7], [7,8], [7,9], [8,0], [8,10], [8,11], [8,12], [8,13], [8,14], [8,15], [8,16], [8,17], [8,18], [8,19], [8,1], [8,20], [8,21], [8,22], [8,23], [8,2], [8,3], [8,4], [8,5], [8,6], [8,7], [8,8], [8,9], [9,0] */,
	pub bac_win: bool /* false */,
	pub back_to: String /* spins */,
	pub bet_per_line: i64 /* 5 */,
	pub bg_type: i64 /* 0, 1, 2 */,
	pub board: Vec<Vec<i64>> /* [[10,10,0],[0,0,0],[0,10,10],[0,0,0],[10,10,0]] */,
	pub bs_count: i64 /* 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 37 */,
	pub bs_multi: Vec<Vec<i64>> /* [[1,1,1],[1,1,1],[1,1,1],[1,1,1],[1,1,1]] */,
	pub bs_v: Vec<Vec<BonusBsVEnum>> /* [[250,200,0],[0,0,0],[0,50,100],[0,0,0],[50,150,0]] */,
	pub bs_values: Vec<Vec<i64>> /* [[5,4,0],[0,0,0],[0,1,2],[0,0,0],[1,3,0]] */,
	pub changes: Vec<Changes>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub column_results: Option<Vec<ColumnResults>>,
	pub grand: Vec<i64> /* [0,0,0,0,0], [0,0,0,0,1], [0,0,0,0,2], [0,0,0,0,3], [0,0,0,1,0], [0,0,0,1,1], [0,0,0,1,2], [0,0,0,1,3], [0,0,0,2,0], [0,0,0,2,1], [0,0,0,2,2], [0,0,0,3,0], [0,0,0,3,1], [0,0,0,3,2], [0,0,1,0,0], [0,0,1,0,1], [0,0,1,0,2], [0,0,1,0,3], [0,0,1,1,0], [0,0,1,1,1], [0,0,1,1,2], [0,0,1,1,3], [0,0,1,2,0], [0,0,1,2,1], [0,0,1,2,2], [0,0,1,3,0], [0,0,1,3,1], [0,0,1,3,2], [0,0,1,3,3], [0,0,1,4,1], [0,0,2,0,0], [0,0,2,0,1], [0,0,2,0,2], [0,0,2,0,3], [0,0,2,1,0], [0,0,2,1,1], [0,0,2,1,2], [0,0,2,1,3], [0,0,2,2,0], [0,0,2,2,1], [0,0,2,2,2], [0,0,2,3,1], [0,0,2,3,2], [0,0,2,3,3], [0,0,3,0,0], [0,0,3,0,1], [0,0,3,0,2], [0,0,3,1,0], [0,0,3,1,1], [0,0,3,1,2], [0,0,3,2,0], [0,0,3,2,1], [0,0,3,3,3], [0,1,0,0,0], [0,1,0,0,1], [0,1,0,0,2], [0,1,0,0,3], [0,1,0,0,4], [0,1,0,1,0], [0,1,0,1,1], [0,1,0,1,2], [0,1,0,1,3], [0,1,0,2,0], [0,1,0,2,1], [0,1,0,2,2], [0,1,0,2,3], [0,1,0,3,0], [0,1,0,3,1], [0,1,0,3,2], [0,1,0,4,1], [0,1,0,4,2], [0,1,1,0,0], [0,1,1,0,1], [0,1,1,0,2], [0,1,1,0,3], [0,1,1,1,0], [0,1,1,1,1], [0,1,1,1,2], [0,1,1,2,0], [0,1,1,2,1], [0,1,1,2,2], [0,1,1,3,0], [0,1,1,3,1], [0,1,1,3,2], [0,1,1,4,1], [0,1,1,4,2], [0,1,2,0,0], [0,1,2,0,1], [0,1,2,0,2], [0,1,2,0,3], [0,1,2,1,0], [0,1,2,1,1], [0,1,2,1,2], [0,1,2,1,3], [0,1,2,2,0], [0,1,2,2,1], [0,1,2,2,2], [0,1,2,3,0], [0,1,2,3,1], [0,1,2,3,2], [0,1,2,3,3], [0,1,3,0,0], [0,1,3,0,1], [0,1,3,0,2], [0,1,3,1,0], [0,1,3,1,1], [0,1,3,1,2], [0,1,3,2,0], [0,1,3,3,0], [0,1,4,1,0], [0,1,4,2,0], [0,1,4,2,1], [0,2,0,0,0], [0,2,0,0,1], [0,2,0,0,2], [0,2,0,0,3], [0,2,0,0,4], [0,2,0,1,0], [0,2,0,1,1], [0,2,0,1,2], [0,2,0,2,0], [0,2,0,2,1], [0,2,0,2,2], [0,2,0,2,3], [0,2,0,3,0], [0,2,0,3,1], [0,2,0,3,2], [0,2,0,3,3], [0,2,1,0,0], [0,2,1,0,1], [0,2,1,0,2], [0,2,1,0,3], [0,2,1,0,4], [0,2,1,0,5], [0,2,1,1,0], [0,2,1,1,1], [0,2,1,1,2], [0,2,1,1,3], [0,2,1,2,0], [0,2,1,2,1], [0,2,1,2,2], [0,2,1,2,3], [0,2,1,3,0], [0,2,1,3,1], [0,2,1,3,2], [0,2,1,3,3], [0,2,2,0,0], [0,2,2,0,1], [0,2,2,0,2], [0,2,2,0,3], [0,2,2,1,0], [0,2,2,1,1], [0,2,2,1,2], [0,2,2,1,3], [0,2,2,2,0], [0,2,2,2,1], [0,2,2,2,2], [0,2,2,3,0], [0,2,2,3,1], [0,2,3,0,1], [0,2,3,0,2], [0,2,3,1,0], [0,2,3,1,1], [0,2,5,2,1], [0,3,0,0,0], [0,3,0,0,1], [0,3,0,1,0], [0,3,0,1,1], [0,3,0,1,2], [0,3,0,2,0], [0,3,0,2,1], [0,3,1,0,0], [0,3,1,0,1], [0,3,1,0,2], [0,3,1,0,5], [0,3,1,1,0], [0,3,1,1,1], [0,3,1,2,0], [0,3,2,0,0], [0,3,2,0,1], [0,3,2,1,0], [0,3,2,1,1], [0,3,3,1,0], [0,3,3,1,1], [0,4,0,0,0], [0,4,0,1,0], [0,4,0,1,1], [0,4,2,0,1], [0,4,2,1,1], [1,0,0,0,0], [1,0,0,0,1], [1,0,0,0,2], [1,0,0,0,3], [1,0,0,1,0], [1,0,0,1,1], [1,0,0,1,2], [1,0,0,1,3], [1,0,0,2,0], [1,0,0,2,1], [1,0,0,2,2], [1,0,0,3,0], [1,0,0,3,1], [1,0,0,3,2], [1,0,1,0,0], [1,0,1,0,1], [1,0,1,0,2], [1,0,1,0,3], [1,0,1,1,0], [1,0,1,1,1], [1,0,1,1,2], [1,0,1,1,3], [1,0,1,2,0], [1,0,1,2,1], [1,0,1,2,2], [1,0,1,3,0], [1,0,1,3,1], [1,0,1,4,0], [1,0,2,0,0], [1,0,2,0,1], [1,0,2,0,2], [1,0,2,0,3], [1,0,2,1,0], [1,0,2,1,1], [1,0,2,1,2], [1,0,2,2,0], [1,0,2,2,1], [1,0,2,2,2], [1,0,2,3,1], [1,0,3,0,0], [1,0,3,0,1], [1,0,3,0,2], [1,0,3,1,0], [1,0,3,1,1], [1,0,3,1,2], [1,0,3,2,0], [1,0,3,2,1], [1,1,0,0,0], [1,1,0,0,1], [1,1,0,0,2], [1,1,0,0,3], [1,1,0,1,0], [1,1,0,1,1], [1,1,0,1,2], [1,1,0,1,3], [1,1,0,2,0], [1,1,0,2,1], [1,1,0,2,2], [1,1,0,2,3], [1,1,0,3,0], [1,1,0,3,1], [1,1,0,3,2], [1,1,1,0,0], [1,1,1,0,1], [1,1,1,0,2], [1,1,1,0,3], [1,1,1,1,0], [1,1,1,1,1], [1,1,1,1,2], [1,1,1,1,3], [1,1,1,2,0], [1,1,1,2,1], [1,1,1,2,2], [1,1,1,3,0], [1,1,1,4,0], [1,1,2,0,0], [1,1,2,0,1], [1,1,2,0,2], [1,1,2,0,3], [1,1,2,1,0], [1,1,2,1,1], [1,1,2,1,2], [1,1,2,2,0], [1,1,2,2,1], [1,1,2,3,0], [1,1,2,3,1], [1,1,2,3,2], [1,1,3,0,0], [1,1,3,0,1], [1,1,3,0,2], [1,1,3,0,3], [1,1,3,1,0], [1,1,3,1,2], [1,1,3,2,0], [1,1,3,2,1], [1,1,3,3,0], [1,2,0,0,0], [1,2,0,0,1], [1,2,0,0,2], [1,2,0,1,0], [1,2,0,1,1], [1,2,0,1,2], [1,2,0,2,0], [1,2,0,2,2], [1,2,0,3,0], [1,2,0,3,1], [1,2,0,3,3], [1,2,1,0,0], [1,2,1,0,1], [1,2,1,0,2], [1,2,1,1,0], [1,2,1,1,1], [1,2,1,1,2], [1,2,1,2,0], [1,2,1,2,1], [1,2,1,3,0], [1,2,1,3,3], [1,2,2,0,0], [1,2,2,0,1], [1,2,2,0,2], [1,2,2,0,3], [1,2,2,1,0], [1,2,2,1,1], [1,2,2,2,0], [1,2,2,2,1], [1,2,2,2,2], [1,2,3,0,0], [1,2,3,0,2], [1,2,3,0,3], [1,3,0,0,0], [1,3,0,0,1], [1,3,0,1,0], [1,3,0,1,1], [1,3,0,1,2], [1,3,0,2,1], [1,3,0,2,2], [1,3,1,0,0], [1,3,1,0,1], [1,3,1,1,0], [1,3,1,1,1], [1,3,2,0,0], [1,3,2,0,1], [1,3,2,1,1], [1,3,2,2,1], [1,3,3,1,0], [1,4,0,1,1], [2,0,0,0,0], [2,0,0,0,1], [2,0,0,0,2], [2,0,0,0,3], [2,0,0,1,0], [2,0,0,1,1], [2,0,0,1,2], [2,0,0,1,3], [2,0,0,2,0], [2,0,0,2,1], [2,0,0,2,2], [2,0,1,0,0], [2,0,1,0,1], [2,0,1,0,2], [2,0,1,0,3], [2,0,1,1,0], [2,0,1,1,1], [2,0,1,1,2], [2,0,1,1,3], [2,0,1,2,0], [2,0,1,2,1], [2,0,2,0,0], [2,0,2,0,1], [2,0,2,0,2], [2,0,2,1,0], [2,0,2,1,1], [2,0,2,1,2], [2,0,2,1,3], [2,0,2,2,0], [2,0,2,2,1], [2,0,2,2,2], [2,0,2,2,3], [2,0,3,0,0], [2,0,3,0,1], [2,0,3,1,0], [2,0,3,1,1], [2,1,0,0,0], [2,1,0,0,1], [2,1,0,0,2], [2,1,0,1,0], [2,1,0,1,1], [2,1,0,1,2], [2,1,0,2,0], [2,1,0,2,1], [2,1,0,2,2], [2,1,0,2,3], [2,1,0,3,0], [2,1,0,3,1], [2,1,1,0,0], [2,1,1,0,1], [2,1,1,0,2], [2,1,1,1,0], [2,1,1,1,1], [2,1,1,2,0], [2,1,1,2,1], [2,1,1,2,3], [2,1,2,0,0], [2,1,2,0,1], [2,1,2,0,2], [2,1,2,1,0], [2,1,2,1,1], [2,1,2,1,2], [2,1,2,2,0], [2,1,3,0,0], [2,1,3,0,2], [2,2,0,0,0], [2,2,0,0,1], [2,2,0,1,0], [2,2,0,1,1], [2,2,0,2,0], [2,2,0,3,0], [2,2,0,3,1], [2,2,1,0,0], [2,2,1,0,1], [2,2,1,0,2], [2,2,1,1,0], [2,2,2,0,0], [2,2,2,0,2], [2,2,2,1,0], [2,2,2,1,1], [2,2,3,0,2], [2,2,3,1,2], [2,3,0,0,0], [2,3,0,0,1], [2,3,0,0,2], [2,3,0,1,0], [2,3,0,1,1], [2,3,1,1,0], [3,0,0,0,0], [3,0,0,0,1], [3,0,0,0,2], [3,0,0,1,0], [3,0,0,1,1], [3,0,0,1,2], [3,0,0,2,0], [3,0,1,0,0], [3,0,1,0,1], [3,0,1,0,3], [3,0,1,1,0], [3,0,1,1,1], [3,0,1,1,3], [3,0,1,2,0], [3,0,1,2,1], [3,0,2,0,0], [3,0,2,0,2], [3,0,2,0,3], [3,0,2,1,0], [3,0,2,1,1], [3,0,2,2,1], [3,1,0,0,0], [3,1,0,0,1], [3,1,0,0,2], [3,1,0,1,0], [3,1,0,1,1], [3,1,0,2,0], [3,1,0,2,1], [3,1,1,0,0], [3,1,1,0,1], [3,1,1,1,0], [3,1,1,1,1], [3,1,1,2,1], [3,1,2,0,0], [3,1,2,0,1], [3,1,2,1,0], [3,1,2,1,1], [3,2,0,0,1], [3,2,0,1,0], [3,2,0,1,1], [3,2,1,0,0], [3,2,1,0,1], [4,0,1,2,0], [4,0,2,1,0], [4,0,2,2,0], [4,1,0,0,2] */,
	pub is_grand_win: bool /* false, true */,
	pub is_max_win: bool /* false, true */,
	pub lines: i64 /* 5 */,
	pub round_bet: i64 /* 50 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub round_win: Option<i64> /* 3150 */,
	pub rounds_count: i64 /* 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47 */,
	pub rounds_granted: i64 /* 3 */,
	pub rounds_left: i64 /* 0, 1, 2, 3 */,
	pub rounds_lefts: i64 /* 0, 1, 2, 3 */,
	pub selected_mode: BonusBsVEnum /* 1, 2 */,
	pub total_win: i64 /* 0 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LastArgs {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_factor: Option<i64> /* 10 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_per_line: Option<i64> /* 5 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lines: Option<i64> /* 5 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<SelectedModeEnum> /* 1, 2 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Winlines {
	pub amount: i64 /* 50, 125, 150, 200, 375, 500, 1250, 2500 */,
	pub line: i64 /* 1, 2, 3, 4, 5 */,
	pub occurrences: i64 /* 3, 4, 5 */,
	pub positions: Vec<Vec<i64>> /* [[0,0],[1,0],[2,0],[3,0]] */,
	pub symbol: i64 /* 1, 2, 3, 4, 5, 6, 7, 8 */,
	#[serde(rename = "type")]
	pub winlines_type: String /* lb */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Spins {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub _total_win: Option<i64> /* 0 */,
	pub bac: Vec<i64> /* [0,0], [0,1], [0,2], [0,3], [0,4], [0,5], [0,6], [0,7], [0,8], [1,0], [1,10], [1,11], [1,1], [1,2], [1,3], [1,4], [1,5], [1,6], [1,7], [1,8], [1,9], [2,0], [2,10], [2,11], [2,1], [2,2], [2,3], [2,4], [2,5], [2,6], [2,7], [2,8], [2,9], [3,0], [3,10], [3,11], [3,12], [3,13], [3,1], [3,2], [3,3], [3,4], [3,5], [3,6], [3,7], [3,8], [3,9], [4,0], [4,10], [4,11], [4,12], [4,13], [4,14], [4,15], [4,1], [4,2], [4,3], [4,4], [4,5], [4,6], [4,7], [4,8], [4,9], [5,0], [5,10], [5,11], [5,12], [5,13], [5,14], [5,15], [5,16], [5,17], [5,1], [5,2], [5,3], [5,4], [5,5], [5,6], [5,7], [5,8], [5,9], [6,0], [6,10], [6,11], [6,12], [6,13], [6,14], [6,15], [6,16], [6,17], [6,18], [6,19], [6,1], [6,2], [6,3], [6,4], [6,5], [6,6], [6,7], [6,8], [6,9], [7,0], [7,10], [7,11], [7,12], [7,13], [7,14], [7,15], [7,16], [7,17], [7,18], [7,19], [7,1], [7,20], [7,21], [7,2], [7,3], [7,4], [7,5], [7,6], [7,7], [7,8], [7,9], [8,0], [8,10], [8,11], [8,12], [8,13], [8,14], [8,15], [8,16], [8,17], [8,18], [8,19], [8,1], [8,20], [8,21], [8,22], [8,23], [8,2], [8,3], [8,4], [8,5], [8,6], [8,7], [8,8], [8,9], [9,0] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bac_pos: Option<Vec<Vec<i64>>> /* [[0,0],[0,1],[4,0],[4,1]] */,
	pub bac_win: bool /* false, true */,
	pub bet_per_line: i64 /* 5 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bg_type: Option<i64> /* 0, 1, 2 */,
	pub board: Vec<Vec<i64>> /* [[2,2,5],[9,9,9],[10,11,10],[9,9,9],[7,3,3]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub boost_pay: Option<i64> /* 0, 100, 150, 200, 250, 300, 400, 450, 500, 600, 700, 750, 800, 900, 1000, 1050, 1100, 1200, 1250, 1300, 1350, 1400, 1500, 1600, 1650, 1700, 1750, 1800, 1900, 1950, 2000, 2100, 2200, 2250, 2300, 2400, 2500, 2550, 2600, 2700, 2850, 3000, 3100, 3200, 3300, 3400, 3450, 3500, 3800, 4800, 15150 */,
	pub bs_v: Vec<Vec<BonusBsVEnum>> /* [[0,0,0],[0,0,0],[250,"major",350],[0,0,0],[0,0,0]] */,
	pub bs_values: Vec<Vec<i64>> /* [[0,0,0],[0,0,0],[5,100,7],[0,0,0],[0,0,0]] */,
	pub lines: i64 /* 5 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub original_board: Option<Vec<Vec<i64>>> /* [[2,4,2],[7,5,5],[7,10,10],[3,6,5],[5,8,4]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reelset_name: Option<SpinsReelsetNameEnum> /* low, mid, special_bonus */,
	pub round_bet: i64 /* 50 */,
	pub round_win: i64 /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<i64> /* 1, 2 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub total_win: Option<i64> /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub winlines: Option<Vec<Winlines>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Context {
	pub actions: Vec<ActionNameEnum> /* bonus_init, bonus_spins_stop, buy_spin, respin, spin */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus: Option<Bonus>,
	pub bonus_trigger: bool /* false, true */,
	pub current: ContextActionEnum /* bonus, spins */,
	pub last_action: ActionNameEnum /* bonus_init, bonus_spins_stop, buy_spin, init, respin, spin */,
	pub last_args: LastArgs,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_win: Option<i64> /* 150 */,
	pub round_finished: bool /* false, true */,
	pub spins: Spins,
	pub version: i64 /* 1 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Data {
	pub quick_spin: bool /* false */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct OriginData {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub autogame: Option<bool> /* false */,
	pub data: Data,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub feature: Option<bool> /* false, true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mobile: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub portrait: Option<bool> /* false */,
	pub quick_spin: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub quickspin: Option<i64> /* 1, 2, 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sound: Option<bool> /* false */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BigWinBuyBonus {
	#[serde(rename = "1")]
	pub big_win_buy_bonus_1: Vec<i64> /* [70,100,140] */,
	#[serde(rename = "2")]
	pub big_win_buy_bonus_2: Vec<i64> /* [300,400,600] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CurrencyFormat {
	pub currency_style: String /* code */,
	#[serde(rename = "decimalSeparator")]
	pub decimal_separator: String /* . */,
	pub denominator: i64 /* 100 */,
	pub style: String /* money */,
	#[serde(rename = "thousandsSeparator")]
	pub thousands_separator: String /*  */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct FreespinsBuyingPrice {
	#[serde(rename = "1")]
	pub freespins_buying_price_1: i64 /* 70 */,
	#[serde(rename = "2")]
	pub freespins_buying_price_2: i64 /* 300 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Jackpots {
	pub grand: i64 /* 1000 */,
	pub major: i64 /* 100 */,
	pub mini: i64 /* 15 */,
	pub minor: i64 /* 30 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PaytableEntry {
	pub multiplier: i64 /* 10, 30, 100 */,
	pub occurrences: i64 /* 3, 4, 5 */,
	#[serde(rename = "type")]
	pub paytable_elem_type: String /* lb */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Reelsamples {
	pub spins: Vec<Vec<i64>> /* [[1,2,3,4,5,6,7,8,10,11],[1,2,3,4,5,6,7,8,9,10,11],[1,2,3,4,5,6,7,8,9,10,11,12],[1,2,3,4,5,6,7,8,9,10,11],[1,2,3,4,5,6,7,8,9,10,11]] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RtpBuyBonus {
	#[serde(rename = "1")]
	pub rtp_buy_bonus_1: String /* 95.50% */,
	#[serde(rename = "2")]
	pub rtp_buy_bonus_2: String /* 95.57% */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Symbols {
	pub id: i64 /* 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 */,
	pub name: String /* el_bonus, el_bonus_booster, el_bonus_jackpot, el_empty, el_low_1, el_low_2, el_low_3, el_low_4, el_mid_1, el_mid_2, el_mid_3, el_mid_4, el_wild */,
	#[serde(rename = "type")]
	pub symbols_type: SymbolsTypeEnum /* line, scat, wild */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Settings {
	pub bet_factor: Vec<i64> /* [10] */,
	pub bets: Vec<i64> /* [1,2,3,4,5,8,10,15,20,30,40,50,75,100,200,300,500] */,
	pub big_win: Vec<i64> /* [10,25,50] */,
	pub big_win_bonus: Vec<i64> /* [40,80] */,
	pub big_win_buy_bonus: BigWinBuyBonus,
	pub bonus_symbols: Vec<i64> /* [1,2,3,4,5,6,7,10] */,
	pub booster_values: Vec<i64> /* [2,3,5] */,
	pub cols: i64 /* 5 */,
	pub currency_format: CurrencyFormat,
	pub freespins_buying_price: FreespinsBuyingPrice,
	pub jackpots: Jackpots,
	pub lines: Vec<i64> /* [5] */,
	pub paylines: Vec<Vec<i64>> /* [[1,1,1,1,1],[0,0,0,0,0],[2,2,2,2,2],[0,1,2,1,0],[2,1,0,1,2]] */,
    pub paytable: HashMap<String, Vec<PaytableEntry>>,
	pub reelsamples: Reelsamples,
	pub respins_granted: i64 /* 3 */,
	pub rows: i64 /* 3 */,
	pub rtp: String /* 95.54% */,
	pub rtp_buy_bonus: RtpBuyBonus,
	pub symbols: Vec<Symbols>,
	pub symbols_line: Vec<i64> /* [1,2,3,4,5,6,7,8] */,
	pub symbols_scat: Vec<i64> /* [0,10,11,12] */,
	pub symbols_wild: Vec<i64> /* [9] */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Status {
	pub code: StatusCodeEnum /* OK, OTHER ERROR */,
	#[serde(rename = "type", skip_serializing_if = "Option::is_none")]
	pub status_type: Option<String> /* crit */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
	pub balance: i64 /* 500000 */,
	pub balance_version: i64 /* 1767598769271 */,
	pub currency: String /* FUN */,
	pub huid: String /* "695b6ab1dcd98a2353837c41" */,
	pub nick: String /* "Player 6b83e2dd-8457-4313-bffa-be9fd598d5e6" */,
	pub show_balance: bool /* true */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct GrandLightningOut {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub command: Option<CommandEnum> /* login, play, start */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub context: Option<Context>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modes: Option<Vec<ModesEnum>> /* auto, play */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_data: Option<OriginData>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub request_id: Option<String> /* "c47eebe9eab74a15af19386ae37d3c0d" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session_id: Option<String> /* "1767598769219GLJeLpH9IIAtJFfHk.Y5hvPpQHB2SIrIeTtFnd" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub settings: Option<Settings>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<Status>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<User>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Params {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_factor: Option<i64> /* 10 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_per_line: Option<i64> /* 5 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lines: Option<i64> /* 5 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<SelectedModeEnum> /* 1, 2 */,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Action {
	pub name: ActionNameEnum /* bonus_init, bonus_spins_stop, buy_spin, respin, spin */,
	pub params: Params,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct GrandLightningIn {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub action: Option<Action>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub autogame: Option<bool> /* false */,
	pub command: CommandEnum /* login, play, start */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub huid: Option<String> /* "695b6ab1dcd98a2353837c41" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<String> /* en */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mobile: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mode: Option<String> /* play */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub portrait: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 218 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub quick_spin: Option<i64> /* 1, 2, 3 */,
	pub request_id: String /* "c47eebe9eab74a15af19386ae37d3c0d" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session_id: Option<String> /* "1767598769219GLJeLpH9IIAtJFfHk.Y5hvPpQHB2SIrIeTtFnd" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sound: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub token: Option<String> /* exdemo */,
}

pub const BOARD_HEIGHT: usize = 3;
pub const BOARD_WIDTH: usize = 5;
pub const LINES_COUNT: usize = 25;
pub const MINI_VALUE: i64 = 10;
pub const MINI_STR: &str = "mini";
pub const MINOR_VALUE: i64 = 20;
pub const MINOR_STR: &str = "minor";
pub const MAJOR_VALUE: i64 = 100;
pub const MAJOR_STR: &str = "major";
pub static JACKPOTS_VALUE: &[i64] = &[10, 20, 100];
pub static JACKPOTS_STR: &[&str] = &["mini", "minor", "major"];
pub const COIN: i64 = 10;
pub const COIN_STR: &str = "10";
pub const JACKPOT: i64 = 11;
pub const JACKPOT_STR: &str = "11";
pub const MULTI: i64 = 12;
pub const MULTI_STR: &str = "12";
pub const WILD: i64 = 9;
pub static CHEAP_SYMBOLS: &[i64] = &[1,2,3,4];
pub static EXPENSIVE_SYMBOLS: &[i64] = &[5,6,7,8];
pub static SYMBOLS: &[i64] = &[1,2,3,4,5,6,7,8];
pub static SPECIALS: &[i64] = &[11, 12, 13];
pub static SPINS_SYMBOLS: &[i64] = &[10, 11, 12, 13];
pub static BONUS_SYMBOLS: &[i64] = &[10, 11, 12, 13, 14];

pub const SYMBOL_COIN: char = 'J';