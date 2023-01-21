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
pub enum StartingLocation {
	Left,
	Middle,
	Right,
}

impl Default for StartingLocation {
	fn default() -> Self {
		StartingLocation::Middle
	}
}

impl Display for StartingLocation {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			StartingLocation::Left => write!(f, "Left"),
			StartingLocation::Middle => write!(f, "Middle"),
			StartingLocation::Right => write!(f, "Right"),
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(from = "u32")]
#[serde(into = "u32")]
pub enum ShooterCapability {
	None = 0,
	Low = 1,
	High = 2,
	Both = 3,
}

impl From<u32> for ShooterCapability {
	fn from(value: u32) -> Self {
		match value {
			0 => ShooterCapability::None,
			1 => ShooterCapability::Low,
			2 => ShooterCapability::High,
			3 => ShooterCapability::Both,
			_ => panic!("Invalid shooter capability: {}", value),
		}
	}
}

impl From<ShooterCapability> for u32 {
	fn from(value: ShooterCapability) -> Self {
		match value {
			ShooterCapability::None => 0,
			ShooterCapability::Low => 1,
			ShooterCapability::High => 2,
			ShooterCapability::Both => 3,
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(from = "u32")]
#[serde(into = "u32")]
pub enum ShooterPositions {
	None = 0,
	Hub = 1,
	Far = 2,
	Both = 3,
}

impl Default for ShooterPositions {
	fn default() -> Self {
		ShooterPositions::None
	}
}

impl From<u32> for ShooterPositions {
	fn from(value: u32) -> Self {
		match value {
			0 => ShooterPositions::None,
			1 => ShooterPositions::Hub,
			2 => ShooterPositions::Far,
			3 => ShooterPositions::Both,
			_ => panic!("Invalid shooter positions value: {}", value),
		}
	}
}

impl From<ShooterPositions> for u32 {
	fn from(value: ShooterPositions) -> Self {
		match value {
			ShooterPositions::None => 0,
			ShooterPositions::Hub => 1,
			ShooterPositions::Far => 2,
			ShooterPositions::Both => 3,
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
	pub preloaded_cargo: bool,
	pub starting_location: StartingLocation,
	pub cubes_acquired: u32,
	pub low_goal_attempts: u32,
	pub low_goal_shots: u32,
	pub high_goal_attempts: u32,
	pub high_goal_shots: u32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Teleop {
	pub cells_acquired: u32,
	pub low_goal_attempts: u32,
	pub low_goal_shots: u32,
	pub high_goal_attempts: u32,
	pub high_goal_shots: u32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Climb {
	pub highest_attempted: u32,
	pub highest_scored: u32,
	pub fell: bool,
	pub started_before_endgame: bool,
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
	pub climb: Climb,
	pub speed: f32,
	pub stability: f32,
	pub defence: Option<f32>,
	pub is_primary_defence: bool,
	pub shooter_positions: ShooterPositions,
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
	pub auto_ball_count: Option<u32>,
	pub cube_capacity: Option<u32>,
	pub climb_time: Option<u32>,
	pub climb_height: Option<u32>,
	pub climb_everybot: Option<bool>,
	pub shooter_capability: Option<ShooterCapability>,
	pub shooter_range: Option<ShooterPositions>,
	pub drive_type: Option<DriveType>,
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
			self.auto.starting_location.to_string(),
			self.auto.cubes_acquired.to_string(),
			self.auto.low_goal_attempts.to_string(),
			self.auto.low_goal_shots.to_string(),
			self.auto.high_goal_attempts.to_string(),
			self.auto.high_goal_shots.to_string(),
			self.teleop.cells_acquired.to_string(),
			self.teleop.low_goal_attempts.to_string(),
			self.teleop.low_goal_shots.to_string(),
			self.teleop.high_goal_attempts.to_string(),
			self.teleop.high_goal_shots.to_string(),
			self.climb.highest_attempted.to_string(),
			self.climb.highest_scored.to_string(),
			self.climb.fell.to_string(),
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
