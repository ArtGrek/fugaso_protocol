use super::super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::super::enums::{ActionsEnum, MultiValueEnum, ModesEnum, StatusCodesEnum, CommandsEnum, CurrentActionsEnum, BonusTypeEnum, BonusModesEnum, StatusTypesEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bs {
	pub position: i64 /* 0, 1, 2, 3, 4 */,
	pub reel: i64 /* 0, 1, 2, 3, 4 */,
	#[serde(rename = "type")]
	pub bs_type: BonusTypeEnum /* collector, infinity, infinity_wrath, infinity_wrath_sticky, mystery, mystery_jp, regular */,
	pub value: i64 /* 0, 200, 400, 600, 800, 1000, 1200, 1400, 1600, 1800, 2000, 2200, 2400, 2600, 2800, 3000, 3200, 3400, 3600, 3800, 4000, 4200, 4400, 4600, 4800, 5000, 5200, 5400, 5600, 5800, 6000, 6200, 6400, 6600, 6800, 7000, 7200, 7400, 7600, 7800, 8000, 8200, 8400, 8600, 8800, 9000, 9200, 9400, 9600, 10000, 10200, 10400, 10600, 10800, 11000, 11200, 11400, 11600, 11800, 12000, 12200, 12400, 12600, 12800, 13000, 13200, 13400, 13600, 13800, 14000, 14200, 14400, 14600, 14800, 15000, 15400, 15800, 16000, 16400, 16600, 16800, 17200, 17400, 17600, 18600, 19000, 19400, 19600, 19800, 20200, 20400, 21400, 21600, 22000, 22200, 24000, 27000 */,
}

impl From<server::Bs> for Bs {
	fn from(obj: server::Bs) -> Self {
		Bs {
			position: obj.position,
			reel: obj.reel,
			bs_type: obj.bs_type.into(),
			value: obj.value,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Drawer {
	pub thor_wrath_spins_left: i64 /* 0, 1, 2, 3, 4, 5 */,
}

impl From<server::Drawer> for Drawer {
	fn from(obj: server::Drawer) -> Self {
		Drawer {
			thor_wrath_spins_left: obj.thor_wrath_spins_left,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Spins {
	pub bac: Vec<i64> /* [0,0], [0,10], [0,11], [0,12], [0,14], [0,15], [0,16], [0,17], [0,18], [0,19], [0,20], [0,21], [0,22], [0,23], [0,24], [0,25], [0,26], [0,27], [0,28], [0,29], [0,30], [0,31], [0,32], [0,33], [0,34], [0,35], [0,36], [0,37], [0,38], [0,39], [0,40], [0,41], [0,42], [0,43], [0,44], [0,46], [0,47], [0,48], [0,49], [0,5], [0,6], [0,7], [0,8], [0,9], [1,100], [1,101], [1,102], [1,103], [1,104], [1,105], [1,106], [1,107], [1,108], [1,109], [1,113], [1,114], [1,115], [1,116], [1,117], [1,119], [1,120], [1,121], [1,122], [1,124], [1,126], [1,127], [1,128], [1,129], [1,130], [1,131], [1,132], [1,134], [1,135], [1,136], [1,137], [1,138], [1,139], [1,50], [1,51], [1,53], [1,54], [1,55], [1,56], [1,57], [1,58], [1,59], [1,60], [1,61], [1,62], [1,64], [1,65], [1,66], [1,67], [1,68], [1,69], [1,71], [1,72], [1,73], [1,74], [1,75], [1,76], [1,77], [1,78], [1,79], [1,80], [1,81], [1,82], [1,84], [1,85], [1,86], [1,87], [1,88], [1,89], [1,91], [1,92], [1,93], [1,95], [1,96], [1,97], [1,98], [1,99], [2,140], [2,141], [2,142], [2,143], [2,144], [2,145], [2,146], [2,148], [2,149], [2,150], [2,151], [2,152], [2,153], [2,154], [2,155], [2,157], [2,158], [2,159], [2,162], [2,163], [2,166], [2,169], [2,170], [2,171], [2,173], [2,174], [2,175], [2,176], [2,177], [2,178], [2,181], [2,182], [2,185], [2,186], [2,187], [2,189], [2,191], [2,192], [2,193], [2,196], [2,197], [2,200], [2,201], [2,202], [2,203], [2,204], [2,207], [2,208], [2,209], [2,210], [2,211], [2,212], [2,213], [2,215], [2,217], [2,218], [2,219], [2,221], [2,222], [2,223], [2,224], [2,225], [2,226], [2,227], [2,228], [2,229], [3,231], [3,233], [3,234], [3,235], [3,237], [3,239], [3,242], [3,244], [3,245], [3,246], [3,247], [3,248], [3,249], [3,250], [3,251], [3,252], [3,253], [3,254], [3,256], [3,258], [3,260], [3,262], [3,264], [3,265], [3,267], [3,268], [3,269], [3,271], [3,273], [3,274], [3,275], [3,276], [3,277], [3,278], [3,280], [3,281], [3,282], [3,283], [3,284], [3,285], [3,287], [3,289], [3,290], [3,291], [3,294], [3,295], [3,296], [3,297], [3,301], [3,304], [3,308], [3,309], [3,311], [3,312], [3,314], [3,315], [3,316], [3,317], [3,318], [3,319], [3,320], [3,321], [3,324], [3,325], [3,328], [3,329], [3,330], [3,331], [3,333], [3,334], [3,335], [3,336], [3,338], [3,339], [3,341], [3,342], [3,343], [3,344], [3,346], [3,348], [3,349], [4,350] */,
	pub bet_per_line: i64 /* 20 */,
	pub bg_type: i64 /* 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 */,
	pub board: Vec<Vec<i64>> /* [[1,1,1,1,1],[1,1,5,0,1],[0,0,1,1,1],[1,1,12,0,0],[0,0,1,1,1]] */,
	pub bs: Vec<Bs>,
	pub bs_count: i64 /* 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bs_sticky_bet: Option<i64> /* 200 */,
	pub bs_v: Vec<Vec<MultiValueEnum>> /* [[400,600,400,200,600],[200,200,1000,0,600],[0,0,400,400,200],[200,200,7000,0,0],[0,0,400,200,400]] */,
	pub bs_values: Vec<Vec<i64>> /* [[2,3,2,1,3],[1,1,5,0,3],[0,0,2,2,1],[1,1,35,0,0],[0,0,2,1,2]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub drawer: Option<Drawer>,
	pub is_bought: bool /* false, true */,
	pub is_extra: bool /* false */,
	pub last_bet: i64 /* 20 */,
	pub lines: i64 /* 1 */,
	pub round_bet: i64 /* 200 */,
	pub round_win: i64 /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<BonusModesEnum> /* 1, 2, 3 */,
	pub total_win: i64 /* 18000 */,
}

impl From<server::Spins> for Spins {
	fn from(obj: server::Spins) -> Self {
		Spins {
			bac: obj.bac,
			bet_per_line: obj.bet_per_line,
			bg_type: obj.bg_type.unwrap_or_default(),
			board: obj.board,
			//bs: obj.bs.into_iter().map(Into::into).collect(),
			bs: obj.bs.unwrap_or_default().into_iter().map(Into::into).collect(),
			bs_count: obj.bs_count,
			bs_sticky_bet: obj.bs_sticky_bet,
			bs_v: obj.bs_v.into_iter().map(|inner_vec| {inner_vec.into_iter().map(Into::into).collect()}).collect(),
			bs_values: obj.bs_values,
			drawer: obj.drawer.map(Into::into),
			is_bought: obj.is_bought.unwrap_or_default(),
			is_extra: obj.is_extra.unwrap_or_default(),
			last_bet: obj.last_bet.unwrap_or_default(),
			lines: obj.lines,
			round_bet: obj.round_bet,
			round_win: obj.round_win,
			selected_mode: obj.selected_mode.map(Into::into),
			total_win: obj.total_win.unwrap_or_default(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Achievements {
	pub level: i64 /* 0, 1, 2, 3, 4 */,
	pub level_percent: f64 /* 0.008, 0.011, 0.022, 0.025, 0.033, 0.042, 0.044, 0.056, 0.058, 0.067, 0.075, 0.078, 0.089, 0.1, 0.111, 0.117, 0.12, 0.122, 0.125, 0.133, 0.14, 0.142, 0.144, 0.15, 0.156, 0.158, 0.16, 0.167, 0.175, 0.178, 0.18, 0.183, 0.189, 0.192, 0.2, 0.211, 0.217, 0.22, 0.233, 0.24, 0.244, 0.25, 0.256, 0.267, 0.278, 0.28, 0.283, 0.289, 0.292, 0.3, 0.308, 0.311, 0.317, 0.32, 0.322, 0.325, 0.333, 0.34, 0.342, 0.344, 0.356, 0.358, 0.36, 0.367, 0.375, 0.378, 0.38, 0.383, 0.389, 0.392, 0.4, 0.411, 0.417, 0.42, 0.422, 0.425, 0.433, 0.44, 0.442, 0.45, 0.456, 0.458, 0.46, 0.467, 0.475, 0.478, 0.48, 0.492, 0.5, 0.508, 0.511, 0.52, 0.522, 0.533, 0.54, 0.542, 0.544, 0.55, 0.556, 0.558, 0.56, 0.567, 0.578, 0.58, 0.589, 0.592, 0.6, 0.611, 0.617, 0.62, 0.622, 0.633, 0.64, 0.644, 0.65, 0.656, 0.658, 0.66, 0.667, 0.675, 0.678, 0.68, 0.683, 0.689, 0.7, 0.708, 0.711, 0.717, 0.72, 0.722, 0.725, 0.733, 0.74, 0.742, 0.744, 0.75, 0.756, 0.758, 0.76, 0.767, 0.778, 0.78, 0.783, 0.789, 0.792, 0.8, 0.811, 0.817, 0.82, 0.822, 0.825, 0.833, 0.84, 0.842, 0.844, 0.856, 0.858, 0.86, 0.867, 0.875, 0.878, 0.88, 0.883, 0.889, 0.9, 0.908, 0.911, 0.92, 0.922, 0.925, 0.933, 0.94, 0.942, 0.944, 0.95, 0.956, 0.96, 0.967, 0.978, 0.98, 0.983, 0.989, 0.992, 0, 1 */,
	pub number: i64 /* 0, 5, 6, 7, 8, 9, 10, 11, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 46, 47, 48, 49, 50, 51, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 64, 65, 66, 67, 68, 69, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 84, 85, 86, 87, 88, 89, 91, 92, 93, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 113, 114, 115, 116, 117, 119, 120, 121, 122, 124, 126, 127, 128, 129, 130, 131, 132, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 148, 149, 150, 151, 152, 153, 154, 155, 157, 158, 159, 162, 163, 166, 169, 170, 171, 173, 174, 175, 176, 177, 178, 181, 182, 185, 186, 187, 189, 191, 192, 193, 196, 197, 200, 201, 202, 203, 204, 207, 208, 209, 210, 211, 212, 213, 215, 217, 218, 219, 221, 222, 223, 224, 225, 226, 227, 228, 229, 231, 233, 234, 235, 237, 239, 242, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 256, 258, 260, 262, 264, 265, 267, 268, 269, 271, 273, 274, 275, 276, 277, 278, 280, 281, 282, 283, 284, 285, 287, 289, 290, 291, 294, 295, 296, 297, 301, 304, 308, 309, 311, 312, 314, 315, 316, 317, 318, 319, 320, 321, 324, 325, 328, 329, 330, 331, 333, 334, 335, 336, 338, 339, 341, 342, 343, 344, 346, 348, 349, 350 */,
	pub total_percent: f64 /* 0.014, 0.017, 0.02, 0.023, 0.026, 0.029, 0.031, 0.034, 0.04, 0.043, 0.046, 0.049, 0.051, 0.054, 0.057, 0.06, 0.063, 0.066, 0.069, 0.071, 0.074, 0.077, 0.08, 0.083, 0.086, 0.089, 0.091, 0.094, 0.097, 0.1, 0.103, 0.106, 0.109, 0.111, 0.114, 0.117, 0.12, 0.123, 0.126, 0.131, 0.134, 0.137, 0.14, 0.143, 0.146, 0.151, 0.154, 0.157, 0.16, 0.163, 0.166, 0.169, 0.171, 0.174, 0.177, 0.183, 0.186, 0.189, 0.191, 0.194, 0.197, 0.203, 0.206, 0.209, 0.211, 0.214, 0.217, 0.22, 0.223, 0.226, 0.229, 0.231, 0.234, 0.24, 0.243, 0.246, 0.249, 0.251, 0.254, 0.26, 0.263, 0.266, 0.271, 0.274, 0.277, 0.28, 0.283, 0.286, 0.289, 0.291, 0.294, 0.297, 0.3, 0.303, 0.306, 0.309, 0.311, 0.323, 0.326, 0.329, 0.331, 0.334, 0.34, 0.343, 0.346, 0.349, 0.354, 0.36, 0.363, 0.366, 0.369, 0.371, 0.374, 0.377, 0.383, 0.386, 0.389, 0.391, 0.394, 0.397, 0.4, 0.403, 0.406, 0.409, 0.411, 0.414, 0.417, 0.423, 0.426, 0.429, 0.431, 0.434, 0.437, 0.44, 0.443, 0.449, 0.451, 0.454, 0.463, 0.466, 0.474, 0.483, 0.486, 0.489, 0.494, 0.497, 0.5, 0.503, 0.506, 0.509, 0.517, 0.52, 0.529, 0.531, 0.534, 0.54, 0.546, 0.549, 0.551, 0.56, 0.563, 0.571, 0.574, 0.577, 0.58, 0.583, 0.591, 0.594, 0.597, 0.6, 0.603, 0.606, 0.609, 0.614, 0.62, 0.623, 0.626, 0.631, 0.634, 0.637, 0.64, 0.643, 0.646, 0.649, 0.651, 0.654, 0.66, 0.666, 0.669, 0.671, 0.677, 0.683, 0.691, 0.697, 0.7, 0.703, 0.706, 0.709, 0.711, 0.714, 0.717, 0.72, 0.723, 0.726, 0.731, 0.737, 0.743, 0.749, 0.754, 0.757, 0.763, 0.766, 0.769, 0.774, 0.78, 0.783, 0.786, 0.789, 0.791, 0.794, 0.8, 0.803, 0.806, 0.809, 0.811, 0.814, 0.82, 0.826, 0.829, 0.831, 0.84, 0.843, 0.846, 0.849, 0.86, 0.869, 0.88, 0.883, 0.889, 0.891, 0.897, 0.9, 0.903, 0.906, 0.909, 0.911, 0.914, 0.917, 0.926, 0.929, 0.937, 0.94, 0.943, 0.946, 0.951, 0.954, 0.957, 0.96, 0.966, 0.969, 0.974, 0.977, 0.98, 0.983, 0.989, 0.994, 0.997, 0, 1 */,
}

impl From<server::Achievements> for Achievements {
	fn from(obj: server::Achievements) -> Self {
		Achievements {
			level: obj.level,
			level_percent: obj.level_percent,
			number: obj.number,
			total_percent: obj.total_percent,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bb {
	pub thor_state: i64 /* 0 */,
	pub thor_wrath_spins_left: i64 /* 0 */,
	pub thor_wrath_total: i64 /* 0 */,
}

impl From<server::Bb> for Bb {
	fn from(obj: server::Bb) -> Self {
		Bb {
			thor_state: obj.thor_state.unwrap_or_default(),
			thor_wrath_spins_left: obj.thor_wrath_spins_left,
			thor_wrath_total: obj.thor_wrath_total.unwrap_or_default(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LastArgs {
}

impl From<server::LastArgs> for LastArgs {
	fn from(_obj: server::LastArgs) -> Self {
		LastArgs {
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Context {
	pub achievements: Achievements,
	pub actions: Vec<ActionsEnum> /* buy_spin, spin */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bb: Option<Bb>,
	pub bet20: Bb,
	pub current: CurrentActionsEnum /* spins */,
	pub last_action: ActionsEnum /* bonus_spins_stop */,
	pub last_args: LastArgs,
	pub last_win: i64 /* 18000 */,
	pub round_finished: bool /* true */,
	pub spins: Spins,
	pub version: i64 /* 1 */,
}

impl From<server::Context> for Context {
	fn from(obj: server::Context) -> Self {
		Context {
			achievements: obj.achievements.into(),
			actions: obj.actions.into_iter().map(Into::into).collect(),
			bb: obj.bb.map(Into::into),
			bet20: obj.bet20.into(),
			current: obj.current,
			last_action: obj.last_action,
			last_args: obj.last_args.into(),
			last_win: obj.last_win.unwrap_or_default(),
			round_finished: obj.round_finished,
			spins: obj.spins.into(),
			version: obj.version,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Data {
	pub quick_spin: bool /* false */,
}

impl From<server::Data> for Data {
	fn from(obj: server::Data) -> Self {
		Data {
			quick_spin: obj.quick_spin,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct OriginData {
	pub autogame: bool /* false */,
	pub data: Data,
	pub feature: bool /* true */,
	pub mobile: bool /* false */,
	pub portrait: bool /* false */,
	pub quick_spin: bool /* false */,
	pub quickspin: i64 /* 2 */,
	pub sound: bool /* false */,
}

impl From<server::OriginData> for OriginData {
	fn from(obj: server::OriginData) -> Self {
		OriginData {
			autogame: obj.autogame.unwrap_or_default(),
			data: obj.data.into(),
			feature: obj.feature.unwrap_or_default(),
			mobile: obj.mobile.unwrap_or_default(),
			portrait: obj.portrait.unwrap_or_default(),
			quick_spin: obj.quick_spin,
			quickspin: obj.quickspin.unwrap_or_default(),
			sound: obj.sound.unwrap_or_default(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Status {
	pub code: StatusCodesEnum /* FUNDS_EXCEED, OK */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reason: Option<String> /* Insufficient balance */,
	#[serde(rename = "type", skip_serializing_if = "Option::is_none")]
	pub status_type: Option<StatusTypesEnum> /* exceed */,
}

impl From<server::Status> for Status {
	fn from(obj: server::Status) -> Self {
		Status {
			code: obj.code.into(),
			reason: obj.reason,
			status_type: obj.status_type,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
	pub balance: i64 /* 505000 */,
	pub balance_version: i64 /* 1752068033386 */,
	pub currency: String /* FUN */,
	pub huid: String /* "686e6f522c7c80483b132b54" */,
	pub nick: String /* Player 3a9b80b6-e351-4713-9627-3ad37a961139, Player 481f841a-566d-4f5d-99b7-c085c96e378e, Player 5a8c32af-ffda-4247-8eb5-05f73d3148e9, Player c80b1aac-2423-4f93-9058-3392e18805de */,
	pub show_balance: bool /* true */,
}

impl From<server::User> for User {
	fn from(obj: server::User) -> Self {
		User {
			balance: obj.balance,
			balance_version: obj.balance_version,
			currency: obj.currency,
			huid: obj.huid,
			nick: obj.nick.into(),
			show_balance: obj.show_balance,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BonusSpinsStop {
	pub command: CommandsEnum /* play */,
	pub context: Context,
	pub modes: Vec<ModesEnum> /* auto, play */,
	pub origin_data: OriginData,
	pub request_id: String /* "de70abbd38cf4a678bd313fe15383683" */,
	pub session_id: String /* "17520679221969UH15ouuh3xFUSvXY.EmVaz7x07pImki9byd2v" */,
	pub status: Status,
	pub user: User,
}

impl From<server::Server> for BonusSpinsStop {
	fn from(obj: server::Server) -> Self {
		BonusSpinsStop {
			command: obj.command,
			context: obj.context.unwrap_or_default().into(),
			modes: obj.modes.into_iter().map(Into::into).collect(),
			origin_data: obj.origin_data.unwrap_or_default().into(),
			request_id: obj.request_id,
			session_id: obj.session_id,
			status: obj.status.into(),
			user: obj.user.into(),
		}
	}
}

