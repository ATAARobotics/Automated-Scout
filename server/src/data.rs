use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

// Setting up MatchType
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MatchType {
	Qualification,
	Practice,
}

impl Default for MatchType {
	fn default() -> Self {
		MatchType::Practice
	}
}

impl Display for MatchType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			MatchType::Qualification => write!(f, "Qualification"),
			MatchType::Practice => write!(f, "Practice"),
		}
	}
}
// Setting up AutoChargeStation
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AutoChargeStation {
	Off,
	On,
	Charged,
	OtherRobot,
}

impl Default for AutoChargeStation {
	fn default() -> Self {
		AutoChargeStation::Off
	}
}

impl Display for AutoChargeStation {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			AutoChargeStation::Off => write!(f, "Off"),
			AutoChargeStation::On => write!(f, "On"),
			AutoChargeStation::Charged => write!(f, "Charged"),
			AutoChargeStation::OtherRobot => write!(f, "OtherRobot")
		}
	}
}
// Setting up TeleopChargeStation
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TeleopChargeStation {
	Off,
	Parked,
	On,
	Charged,
}

impl Default for TeleopChargeStation {
	fn default() -> Self {
		TeleopChargeStation::Off
	}
}

impl Display for TeleopChargeStation {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			TeleopChargeStation::Off => write!(f, "Off"),
			TeleopChargeStation::Parked => write!(f, "Parked"),
			TeleopChargeStation::On => write!(f, "On"),
			TeleopChargeStation::Charged => write!(f, "Charged"),
		}
	}
}
// Setting up PreferredPlay
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(from = "u32")]
#[serde(into = "u32")]
pub enum PreferredPlay {
	Defence = 0,
	PreferDefence = 1,
	PreferOffence = 2,
	Offence = 3,
}

impl Default for PreferredPlay {
	fn default() -> Self {
		PreferredPlay::PreferOffence
	}
}

impl From<u32> for PreferredPlay {
	fn from(value: u32) -> Self {
		match value {
			0 => PreferredPlay::Defence,
			1 => PreferredPlay::PreferDefence,
			2 => PreferredPlay::PreferOffence,
			3 => PreferredPlay::Offence,
			_ => panic!("Invalid Preferred Play value: {}", value),
		}
	}
}

impl From<PreferredPlay> for u32 {
	fn from(value: PreferredPlay) -> Self {
		match value {
			PreferredPlay::Defence => 0,
			PreferredPlay::PreferDefence => 1,
			PreferredPlay::PreferOffence => 2,
			PreferredPlay::Offence => 3,
		}
	}
}
// Setting up HumanPickupRange
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(from = "u32")]
#[serde(into = "u32")]
pub enum HumanPickupRange {
	None = 0,
	Chute = 1,
	SlideShelf = 2,
	Both = 3,
}

impl Default for HumanPickupRange {
	fn default() -> Self {
		HumanPickupRange::None
	}
}

impl From<u32> for HumanPickupRange {
	fn from(value: u32) -> Self {
		match value {
			0 => HumanPickupRange::None,
			1 => HumanPickupRange::Chute,
			2 => HumanPickupRange::SlideShelf,
			3 => HumanPickupRange::Both,
			_ => panic!("Invalid Human Player Pickup Range value: {}", value),
		}
	}
}

impl From<HumanPickupRange> for u32 {
	fn from(value: HumanPickupRange) -> Self {
		match value {
			HumanPickupRange::None => 0,
			HumanPickupRange::Chute => 1,
			HumanPickupRange::SlideShelf => 2,
			HumanPickupRange::Both => 3,
		}
	}
}
// Setting up StackType
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(from = "u32")]
#[serde(into = "u32")]
pub enum StackType {
	None = 0,
	Cone = 1,
	Cube = 2,
	Both = 3,
}

impl From<u32> for StackType {
	fn from(value: u32) -> Self {
		match value {
			0 => StackType::None,
			1 => StackType::Cone,
			2 => StackType::Cube,
			3 => StackType::Both,
			_ => panic!("Invalid Stack Type: {}", value),
		}
	}
}

impl From<StackType> for u32 {
	fn from(value: StackType) -> Self {
		match value {
			StackType::None => 0,
			StackType::Cone => 1,
			StackType::Cube => 2,
			StackType::Both => 3,
		}
	}
}
// Setting up PreferredStack
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(from = "u32")]
#[serde(into = "u32")]
pub enum PreferredStack {
	None = 0,
	Hybrid = 1,
	Middle = 2,
	High = 3,
}

impl Default for PreferredStack {
	fn default() -> Self {
		PreferredStack::None
	}
}

impl From<u32> for PreferredStack {
	fn from(value: u32) -> Self {
		match value {
			0 => PreferredStack::None,
			1 => PreferredStack::Hybrid,
			2 => PreferredStack::Middle,
			3 => PreferredStack::High,
			_ => panic!("Invalid Preferred Stack value: {}", value),
		}
	}
}

impl From<PreferredStack> for u32 {
	fn from(value: PreferredStack) -> Self {
		match value {
			PreferredStack::None => 0,
			PreferredStack::Hybrid => 1,
			PreferredStack::Middle => 2,
			PreferredStack::High => 3,
		}
	}
}
// Setting up ConfidenceLevel
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(from = "u32")]
#[serde(into = "u32")]
pub enum ConfidenceLevel {
	HonestlyUnconfident = 0,
	SemiUnconfident = 1,
	Middle = 2,
	Confident = 3,
	TooConfident = 4,
}

impl Default for ConfidenceLevel {
	fn default() -> Self {
		ConfidenceLevel::Middle
	}
}

impl From<u32> for ConfidenceLevel {
	fn from(value: u32) -> Self {
		match value {
			0 => ConfidenceLevel::HonestlyUnconfident,
			1 => ConfidenceLevel::SemiUnconfident,
			2 => ConfidenceLevel::Middle,
			3 => ConfidenceLevel::Confident,
			4 => ConfidenceLevel::TooConfident,
			_ => panic!("Invalid Cnfidence value: {}", value),
		}
	}
}

impl From<ConfidenceLevel> for u32 {
	fn from(value: ConfidenceLevel) -> Self {
		match value {
			ConfidenceLevel::HonestlyUnconfident => 0,
			ConfidenceLevel::SemiUnconfident => 1,
			ConfidenceLevel::Middle => 2,
			ConfidenceLevel::Confident => 3,
			ConfidenceLevel::TooConfident => 4,
		}
	}
}
// Setting up ChargeBattery
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(from = "u32")]
#[serde(into = "u32")]
pub enum ChargeBattery {
	No = 0,
	Yes = 1,
}

impl Default for ChargeBattery {
	fn default() -> Self {
		ChargeBattery::No
	}
}

impl From<u32> for ChargeBattery {
	fn from(value: u32) -> Self {
		match value {
			0 => ChargeBattery::No,
			1 => ChargeBattery::Yes,
			_ => panic!("Invalid Charge Battery value: {}", value),
		}
	}
}

impl From<ChargeBattery> for u32 {
	fn from(value: ChargeBattery) -> Self {
		match value {
			ChargeBattery::No => 0,
			ChargeBattery::Yes => 1,
		}
	}
}
// Setting up VisionType
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(from = "u32")]
#[serde(into = "u32")]
pub enum VisionType {
	None = 0,
	Tape = 1,
	AprilTags = 2,
	Both = 3,
}

impl Default for VisionType {
	fn default() -> Self {
		VisionType::None
	}
}

impl From<u32> for VisionType {
	fn from(value: u32) -> Self {
		match value {
			0 => VisionType::None,
			1 => VisionType::Tape,
			2 => VisionType::AprilTags,
			3 => VisionType::Both,
			_ => panic!("Invalid Vision value: {}", value),
		}
	}
}

impl From<VisionType> for u32 {
	fn from(value: VisionType) -> Self {
		match value {
			VisionType::None => 0,
			VisionType::Tape => 1,
			VisionType::AprilTags => 2,
			VisionType::Both => 3,
		}
	}
}
// Setting up BumperType
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(from = "u32")]
#[serde(into = "u32")]
pub enum BumperType {
	None = 0,
	Swap = 1,
	Reversable = 2,
}

impl Default for BumperType {
	fn default() -> Self {
		BumperType::None
	}
}

impl From<u32> for BumperType {
	fn from(value: u32) -> Self {
		match value {
			0 => BumperType::None,
			1 => BumperType::Swap,
			2 => BumperType::Reversable,
			_ => panic!("Invalid Bumper value: {}", value),
		}
	}
}

impl From<BumperType> for u32 {
	fn from(value: BumperType) -> Self {
		match value {
			BumperType::None => 0,
			BumperType::Swap => 1,
			BumperType::Reversable => 2,
		}
	}
}

// Setting up Auto Structure, match info can pull from here
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Auto {
	pub exited_tarmac: bool,
	pub auto_charge_station: AutoChargeStation,
	//pub cone_picked_up: u32,
	//pub cube_picked_up: u32,
	pub hybrid_cube_scored: u32,
	pub hybrid_cone_scored: u32,
	pub middle_cube_scored: u32,
	pub middle_cone_scored: u32,
	pub high_cube_scored: u32,
	pub high_cone_scored: u32,
}
// Setting up Teleop Structure, match info can pull from here
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Teleop {
	//pub cone_picked_up: u32,
	//pub cube_picked_up: u32,
	pub hybrid_cube_scored: u32,
	pub hybrid_cone_scored: u32,
	pub middle_cube_scored: u32,
	pub middle_cone_scored: u32,
	pub high_cube_scored: u32,
	pub high_cone_scored: u32,
	pub parked: bool,
	pub teleop_charge_station: TeleopChargeStation,
}


// Setting up Match Info, this consists of values mentioned earlier plus a few others
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct MatchInfo {
	#[serde(rename = "match")]
	pub match_number: u32,
	pub match_category: MatchType,
	#[serde(rename = "team")]
	pub team_number: u32,
	pub auto: Auto,
	pub teleop: Teleop,
	pub speed: f32,
	pub stability: f32,
	pub defence: Option<f32>,
	pub is_primary_defence: bool,
	pub was_broken: bool,
	pub was_disabled: bool,
	pub notes: String,
	pub last_modified_time: u64,
}

// Setting up Pit (Pit Scouting) structure
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Pit {
	pub pit_people: Option<u32>,
	pub chaos: Option<u32>,
	pub confidence_level: Option<ConfidenceLevel>,
	pub scouting_method: String,
}

// Setting up Robot (Pit Scouting) structure, contains a few types from earlier inside of it
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Robot {
	pub bumper_type: Option<BumperType>,
	pub vision_type: Option<VisionType>,
	pub human_pickup_range: Option<HumanPickupRange>,
	pub stack_type: Option<StackType>,
	pub preferred_play: Option<PreferredPlay>,
	pub preferred_stack: Option<PreferredStack>,
	pub charge_battery: Option<ChargeBattery>,
	pub battery_amount: Option<u32>,
	pub drive_motor_amount: Option<u32>,
	pub other_motor_amount: Option<u32>,
	pub balance_time: Option<u32>,
	pub auto_settings: String,
	pub drive_type: String,
	pub comments: String,
}

// Setting up RobotInfo (Pit scouting), contains Pit and Robot structure
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RobotInfo {
	#[serde(rename = "scoutingTime")]
	pub visit_number: u32,
	#[serde(rename = "team")]
	pub team_number: u32,
	pub pit: Pit,
	pub robot: Robot,
	pub images: Vec<String>,
	pub last_modified_time: u64,
}

// Setting up general Info
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Info {
	MatchInfo(MatchInfo),
	RobotInfo(RobotInfo),
}

impl Default for Info {
	fn default() -> Self {
		Info::MatchInfo(MatchInfo::default())
	}
}

// Convert match info to string
impl MatchInfo {
	// Arden match data export
	pub const HEADER: &'static str = "match_number,match_category,team,auto_exited_tarmac,auto_charge_station,auto_hybrid_cube_scored,auto_hybrid_cone_scored,auto_middle_cube_scored,auto_middle_cone_scored,auto_high_cube_scored,auto_high_cone_scored,teleop_hybrid_cube_scored,teleop_hybrid_cone_scored,teleop_middle_cube_scored,teleop_middle_cone_scored,teleop_high_cube_scored,teleop_high_cone_scored,speed,stability,defence,is_primary_defence,was_broken,was_disabled,notes\n";
	pub fn write_csv_line(&self) -> String {
		vec![
			self.match_number.to_string(),
			self.match_category.to_string(),
			self.team_number.to_string(),
			
			self.auto.exited_tarmac.to_string(),
			self.auto.auto_charge_station.to_string(),
			//self.auto.cone_picked_up.to_string(),
			//self.auto.cube_picked_up.to_string(),
			self.auto.hybrid_cube_scored.to_string(),
			self.auto.hybrid_cone_scored.to_string(),
			self.auto.middle_cube_scored.to_string(),
			self.auto.middle_cone_scored.to_string(),
			self.auto.high_cube_scored.to_string(),
			self.auto.high_cone_scored.to_string(),
			
			//self.teleop.cone_picked_up.to_string(),
			//self.teleop.cube_picked_up.to_string(),
			self.teleop.hybrid_cube_scored.to_string(),
			self.teleop.hybrid_cone_scored.to_string(),
			self.teleop.middle_cube_scored.to_string(),
			self.teleop.middle_cone_scored.to_string(),
			self.teleop.high_cube_scored.to_string(),
			self.teleop.high_cone_scored.to_string(),
			self.teleop.parked.to_string(),
			self.teleop.teleop_charge_station.to_string(),
			
			self.speed.to_string(),
			self.stability.to_string(),
			self.defence
				.map(|v| v.to_string())
				.unwrap_or_else(|| "N/A".to_string()),
			self.is_primary_defence.to_string(),
			self.was_disabled.to_string(),
			"\"".to_string() + &self.notes.replace('\n', "  ") + "\"",
		]
		.join(",") + "\n"
	}
}
