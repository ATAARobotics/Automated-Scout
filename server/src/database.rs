use crate::data::MatchInfo;
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

// Check whether the given path is a valid match (whether it was scouted after the start of the competition).
fn is_match_valid(match_info: &MatchInfo) -> bool {
	match_info.match_number != 0 && match_info.match_number < 100 && match_info.team_number != 0 && match_info.last_modified_time > 1647129117000
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
	pub fn get_all_matches(&self) -> MatchIter {
		MatchIter::from_sled(self.backend.scan_prefix(b"match_"))
	}
	pub fn merge_matches(&self, matches: &Vec<MatchInfo>) -> Result<(), DatabaseError> {
		for match_info in matches {
			self.write_match(match_info)?;
		}
		Ok(())
	}
	pub fn get_match_list(&self) -> Vec<MatchInfo> {
		self.get_all_matches().map(|data| data.unwrap()).collect()
	}
}
