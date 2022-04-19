use crate::data::{MatchInfo, RobotInfo};
use crate::Info;
use std::array::TryFromSliceError;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
	#[error("Internal database error: {0}")]
	Sled(#[from] sled::Error),
	#[error("Failed to decode data in database: {0}")]
	Serde(#[from] bincode::Error),
	#[error("Failed to decode data in database: {0}")]
	TryFromSlice(#[from] TryFromSliceError),
}

#[derive(Debug, Clone)]
pub struct Database {
	backend: sled::Db,
}

pub struct MatchIter {
	inner: sled::Iter,
}

impl MatchIter {
	fn from_sled(iter: sled::Iter) -> Self {
		MatchIter { inner: iter }
	}
}

pub struct RobotIter {
	inner: sled::Iter,
}

impl RobotIter {
	fn from_sled(iter: sled::Iter) -> Self {
		RobotIter { inner: iter }
	}
}

const COMP_START: u64 = 1650316000000;

// Check whether the given match is a valid match (whether it was scouted after the start of the competition).
fn is_match_valid(match_info: &MatchInfo) -> bool {
	match_info.match_number != 0
		&& match_info.match_number < 100
		&& match_info.team_number != 0
		&& match_info.last_modified_time > COMP_START
}
// Check whether the given robot is a valid robot (whether it was scouted after the start of the competition).
fn is_robot_valid(robot_info: &RobotInfo) -> bool {
	robot_info.visit_number < 10
		&& robot_info.team_number != 0
		&& robot_info.last_modified_time > COMP_START
}

impl Iterator for MatchIter {
	type Item = Result<MatchInfo, DatabaseError>;

	fn next(&mut self) -> Option<Self::Item> {
		for next in self.inner.by_ref() {
			let (_key, value) = next.unwrap();
			let value: MatchInfo = bincode::deserialize(&value).unwrap();
			if is_match_valid(&value) {
				return Some(Ok(value));
			}
		}
		None
	}
}

impl Iterator for RobotIter {
	type Item = Result<RobotInfo, DatabaseError>;

	fn next(&mut self) -> Option<Self::Item> {
		for next in self.inner.by_ref() {
			let (_key, value) = next.unwrap();
			let value: RobotInfo = bincode::deserialize(&value).unwrap();
			if is_robot_valid(&value) {
				return Some(Ok(value));
			}
		}
		None
	}
}

impl Database {
	pub fn open(file: &Path) -> Self {
		Database {
			backend: sled::open(file).unwrap(),
		}
	}
	pub fn get_match_id(match_info: &MatchInfo) -> Vec<u8> {
		Vec::from(format!(
			"match_{}_{:?}_{}",
			match_info.match_number, match_info.match_category, match_info.team_number,
		))
	}
	pub fn get_robot_id(robot_info: &RobotInfo) -> Vec<u8> {
		Vec::from(format!(
			"robot_{}_{}",
			robot_info.team_number, robot_info.visit_number,
		))
	}
	pub fn write_match(&self, match_info: &MatchInfo) -> Result<(), DatabaseError> {
		if !is_match_valid(match_info) {
			return Ok(());
		}
		let id = Self::get_match_id(match_info);
		if let Some(data) = self.backend.get(&id)? {
			if let Ok(old_match_info) = bincode::deserialize::<MatchInfo>(&data) {
				if old_match_info.last_modified_time >= match_info.last_modified_time {
					// Don't replace newer things.
					return Ok(());
				}
			}
		}
		let data = bincode::serialize(match_info)?;
		self.backend.insert(id, data)?;
		Ok(())
	}
	pub fn write_robot(&self, robot_info: &RobotInfo) -> Result<(), DatabaseError> {
		if !is_robot_valid(robot_info) {
			return Ok(());
		}
		let id = Self::get_robot_id(robot_info);
		if let Some(data) = self.backend.get(&id)? {
			if let Ok(old_robot_info) = bincode::deserialize::<RobotInfo>(&data) {
				if old_robot_info.last_modified_time >= robot_info.last_modified_time {
					// Don't replace newer things.
					return Ok(());
				}
			}
		}
		let data = bincode::serialize(robot_info)?;
		self.backend.insert(id, data)?;
		Ok(())
	}
	pub fn write_info(&self, info: &Info) -> Result<(), DatabaseError> {
		match info {
			Info::MatchInfo(match_info) => self.write_match(match_info),
			Info::RobotInfo(robot_info) => self.write_robot(robot_info),
		}
	}
	pub fn get_all_matches(&self) -> MatchIter {
		MatchIter::from_sled(self.backend.scan_prefix(b"match_"))
	}
	pub fn get_all_robots(&self) -> RobotIter {
		RobotIter::from_sled(self.backend.scan_prefix(b"robot_"))
	}
	pub fn merge_info(&self, infos: &Vec<Info>) -> Result<(), DatabaseError> {
		for info in infos {
			self.write_info(info)?;
		}
		Ok(())
	}
	pub fn get_info_list(&self) -> Vec<Info> {
		self.get_all_matches()
			.map(|data| Info::MatchInfo(data.unwrap()))
			.chain(
				self.get_all_robots()
					.map(|data| Info::RobotInfo(data.unwrap())),
			)
			.collect()
	}
}
