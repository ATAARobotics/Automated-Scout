use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChargeStation {
	Off,
	On,
	Charged,
}

impl Default for ChargeStation {
	fn default() -> Self {
		ChargeStation::Off
	}
}

impl Display for ChargeStation {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ChargeStation::Off => write!(f, "Off"),
			ChargeStation::On => write!(f, "On"),
			ChargeStation::Charged => write!(f, "Charged"),
		}
	}
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(from = "u32")]
#[serde(into = "u32")]
pub enum PickupType {
	None = 0,
	Cone = 1,
	Cube = 2,
	Both = 3,
}

impl From<u32> for PickupType {
	fn from(value: u32) -> Self {
		match value {
			0 => PickupType::None,
			1 => PickupType::Cone,
			2 => PickupType::Cube,
			3 => PickupType::Both,
			_ => panic!("Invalid Pickup Type: {}", value),
		}
	}
}

impl From<PickupType> for u32 {
	fn from(value: PickupType) -> Self {
		match value {
			PickupType::None => 0,
			PickupType::Cone => 1,
			PickupType::Cube => 2,
			PickupType::Both => 3,
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(from = "u32")]
#[serde(into = "u32")]
pub enum FloorPickupRange {
	None = 0,
	Elsewhere = 1,
	Hybrid = 2,
	Both = 3,
}

impl Default for FloorPickupRange {
	fn default() -> Self {
		FloorPickupRange::None
	}
}

impl From<u32> for FloorPickupRange {
	fn from(value: u32) -> Self {
		match value {
			0 => FloorPickupRange::None,
			1 => FloorPickupRange::Elsewhere,
			2 => FloorPickupRange::Hybrid,
			3 => FloorPickupRange::Both,
			_ => panic!("Invalid Floor Pickup Range value: {}", value),
		}
	}
}

impl From<FloorPickupRange> for u32 {
	fn from(value: FloorPickupRange) -> Self {
		match value {
			FloorPickupRange::None => 0,
			FloorPickupRange::Elsewhere => 1,
			FloorPickupRange::Hybrid => 2,
			FloorPickupRange::Both => 3,
		}
	}
}
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(from = "u32")]
#[serde(into = "u32")]
pub enum StackRange {
	None = 0,
	Hybrid = 1,
	Middle = 2,
	High = 3,
	All = 4,
}

impl Default for StackRange {
	fn default() -> Self {
		StackRange::None
	}
}

impl From<u32> for StackRange {
	fn from(value: u32) -> Self {
		match value {
			0 => StackRange::None,
			1 => StackRange::Hybrid,
			2 => StackRange::Middle,
			3 => StackRange::High,
			4 => StackRange::All,
			_ => panic!("Invalid Stack Range value: {}", value),
		}
	}
}

impl From<StackRange> for u32 {
	fn from(value: StackRange) -> Self {
		match value {
			StackRange::None => 0,
			StackRange::Hybrid => 1,
			StackRange::Middle => 2,
			StackRange::High => 3,
			StackRange::All => 4,
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(from = "u32")]
#[serde(into = "u32")]
pub enum DriveType {
	Swerve = 0,
	Tank = 1,
	Other = 2,
}

impl From<u32> for DriveType {
	fn from(value: u32) -> Self {
		match value {
			0 => DriveType::Swerve,
			1 => DriveType::Tank,
			2 => DriveType::Other,
			_ => panic!("Invalid drive type: {}", value),
		}
	}
}

impl From<DriveType> for u32 {
	fn from(value: DriveType) -> Self {
		match value {
			DriveType::Swerve => 0,
			DriveType::Tank => 1,
			DriveType::Other => 2,
		}
	}
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Auto {
	pub exited_tarmac: bool,
	pub charge_station: ChargeStation,
	pub cone_picked_up: u32,
	pub cube_picked_up: u32,
	pub hybrid_scored: u32,
	pub middle_cube_scored: u32,
	pub middle_cone_scored: u32,
	pub high_cube_scored: u32,
	pub high_cone_scored: u32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Teleop {
	pub cone_picked_up: u32,
	pub cube_picked_up: u32,
	pub hybrid_scored: u32,
	pub middle_cube_scored: u32,
	pub middle_cone_scored: u32,
	pub high_cube_scored: u32,
	pub high_cone_scored: u32,
	pub parked: bool,
	pub charge_station: ChargeStation,
}



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

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Pit {
	pub busy: Option<u32>,
	pub pit_people: Option<u32>,
	pub chaos: Option<u32>,
	pub friendly: Option<bool>,
	pub comments: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Robot {
	pub pickup_type: Option<PickupType>,
	pub floor_pickup_range: Option<FloorPickupRange>,
	pub human_pickup_range: Option<HumanPickupRange>,
	pub stack_type: Option<StackType>,
	pub stack_range: Option<StackRange>,
	pub drive_type: Option<DriveType>,
	pub balance_time: Option<u32>,
	pub everybot: Option<bool>,
	pub comments: String,
}

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

impl MatchInfo {
	pub const HEADER: &'static str = "match_number,match_category,team,auto_exited_tarmac,auto_starting_location,auto_cells_acquired,auto_low_goal_attempts,auto_low_goal_shots,auto_high_goal_attempts,auto_high_goal_shots,teleop_cells_acquired,teleop_low_goal_attempts,teleop_low_goal_shots,teleop_high_goal_attempts,teleop_high_goal_shots,highest_climb_attempted,highest_climb_scored,fell,speed,stability,defence,is_primary_defence,was_broken,was_disabled,notes\n";
	pub fn write_csv_line(&self) -> String {
		vec![
			self.match_number.to_string(),
			self.match_category.to_string(),
			self.team_number.to_string(),
			
			self.auto.exited_tarmac.to_string(),
			self.auto.charge_station.to_string(),
			self.auto.cone_picked_up.to_string(),
			self.auto.cube_picked_up.to_string(),
			self.auto.hybrid_scored.to_string(),
			self.auto.middle_cube_scored.to_string(),
			self.auto.middle_cone_scored.to_string(),
			self.auto.high_cube_scored.to_string(),
			self.auto.high_cone_scored.to_string(),
			
			self.teleop.cone_picked_up.to_string(),
			self.teleop.cube_picked_up.to_string(),
			self.teleop.hybrid_scored.to_string(),
			self.teleop.middle_cube_scored.to_string(),
			self.teleop.middle_cone_scored.to_string(),
			self.teleop.high_cube_scored.to_string(),
			self.teleop.high_cone_scored.to_string(),
			self.teleop.parked.to_string(),
			self.teleop.charge_station.to_string(),
			
			self.speed.to_string(),
			self.stability.to_string(),
			self.defence
				.map(|v| v.to_string())
				.unwrap_or_else(|| "N/A".to_string()),
			self.is_primary_defence.to_string(),
			self.was_broken.to_string(),
			self.was_disabled.to_string(),
			"\"".to_string() + &self.notes.replace('\n', "  ") + "\"",
		]
		.join(",") + "\n"
	}
}
