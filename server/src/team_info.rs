use std::cmp::Ordering;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::analysis::RawMatchData;
use crate::data::{MatchType, RobotInfo};
use crate::{Database, MatchInfo};

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AllianceColour {
	Blue,
	Red,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullMatchInfo {
	time: u64,
	alliance: AllianceColour,
	blue_teams: Vec<(u32, Option<MatchInfo>)>,
	red_teams: Vec<(u32, Option<MatchInfo>)>,
	blue_score: i32,
	red_score: i32,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullTeamInfo {
	pub team_number: u32,
	pub matches: Vec<FullMatchInfo>,
	pub images: Vec<String>,
	pub pit_visits: Vec<RobotInfo>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct RawMediaInfo {
	direct_url: String,
	preferred: bool,
}

impl PartialOrd for RawMediaInfo {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for RawMediaInfo {
	fn cmp(&self, other: &Self) -> Ordering {
		other.preferred.cmp(&self.preferred)
	}
}

fn team_key_to_number(team_key: &str) -> u32 {
	team_key.replace("frc", "").parse::<u32>().unwrap()
}

pub fn get_team_info(database: &Database, team_number: u32) -> FullTeamInfo {
	let mut team_info = FullTeamInfo::default();

	let mut local_team_pits = database
		.get_all_robots()
		.filter_map(|r| r.ok())
		.filter(|r| r.team_number == team_number)
		.collect::<Vec<_>>();
	local_team_pits.sort_by(|a, b| a.visit_number.cmp(&b.visit_number));
	for pit_data in local_team_pits {
		for image in pit_data.images.iter() {
			team_info.images.push(image.clone());
		}
		team_info.pit_visits.push(pit_data);
	}

	if let Ok(resp) = ureq::get(&format!(
		"https://www.thebluealliance.com/api/v3/team/frc{}/media/2022",
		team_number,
	))
	.set("X-TBA-Auth-Key", option_env!("TBA_AUTH_KEY").unwrap_or(""))
	.call()
	{
		if resp.status() == 200 {
			if let Ok(Ok(mut data)) = resp
				.into_string()
				.map(|data| serde_json::from_str::<Vec<RawMediaInfo>>(&data))
			{
				data.sort();
				for media_info in data {
					team_info.images.push(media_info.direct_url);
				}
			}
		}
	}

	let mut match_infos = database
		.get_all_matches()
		.filter_map(|m| m.ok())
		.filter_map(|m| {
			if m.match_category == MatchType::Qualification {
				Some(((m.match_number, m.team_number), m))
			} else {
				None
			}
		})
		.collect::<HashMap<_, _>>();

	if let Ok(resp) = ureq::get(&format!(
		"https://www.thebluealliance.com/api/v3/team/frc{}/event/{}/matches",
		team_number,
		option_env!("TBA_EVENT").unwrap_or("")
	))
	.set("X-TBA-Auth-Key", option_env!("TBA_AUTH_KEY").unwrap_or(""))
	.call()
	{
		if resp.status() == 200 {
			if let Ok(Ok(data)) = resp
				.into_string()
				.map(|data| serde_json::from_str::<Vec<RawMatchData>>(&data))
			{
				for tba_match in data {
					team_info.matches.push(FullMatchInfo {
						time: tba_match
							.actual_time
							.or(tba_match.predicted_time)
							.unwrap_or(tba_match.time),
						alliance: if tba_match
							.alliances
							.blue
							.team_keys
							.contains(&format!("frc{}", team_number))
						{
							AllianceColour::Blue
						} else {
							AllianceColour::Red
						},
						blue_teams: tba_match
							.alliances
							.blue
							.team_keys
							.iter()
							.chain(tba_match.alliances.blue.surrogate_team_keys.iter())
							.map(|team| {
								let team_number = team_key_to_number(team);
								(
									team_number,
									match_infos.remove(&(tba_match.match_number, team_number)),
								)
							})
							.collect(),
						red_teams: tba_match
							.alliances
							.red
							.team_keys
							.iter()
							.chain(tba_match.alliances.red.surrogate_team_keys.iter())
							.map(|team| {
								let team_number = team_key_to_number(team);
								(
									team_number,
									match_infos.remove(&(tba_match.match_number, team_number)),
								)
							})
							.collect(),
						blue_score: tba_match.alliances.blue.score,
						red_score: tba_match.alliances.red.score,
					});
				}
			}
		}
	}

	team_info
}
