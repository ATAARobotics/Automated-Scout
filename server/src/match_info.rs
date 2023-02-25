use std::collections::HashMap;

use serde::Serialize;

use crate::analysis::RawMatchData;
use crate::data::{MatchType, RobotInfo};
use crate::{Database, MatchInfo};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullMatchInfo {
	time: u64,
	blue_teams: Vec<(u32, Option<MatchInfo>)>,
	red_teams: Vec<(u32, Option<MatchInfo>)>,
	blue_score: i32,
	red_score: i32,
}

fn team_key_to_number(team_key: &str) -> u32 {
	team_key.replace("frc", "").parse::<u32>().unwrap()
}

pub fn get_match_info(database: &Database, match_number: u32) -> Option<FullMatchInfo> {
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
		"https://www.thebluealliance.com/api/v3/match/{}_qm{}",
		option_env!("TBA_EVENT").unwrap_or(""),
		match_number,
	))
	.set("X-TBA-Auth-Key", option_env!("TBA_AUTH_KEY").unwrap_or(""))
	.call()
	{
		if resp.status() == 200 {
			if let Ok(Ok(tba_match)) = resp
				.into_string()
				.map(|data| serde_json::from_str::<RawMatchData>(&data))
			{
				Some(FullMatchInfo {
					time: tba_match
						.actual_time
						.or(tba_match.predicted_time)
						.unwrap_or(tba_match.time),
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
				})
			} else {
				None
			}
		} else {
			None
		}
	} else {
		None
	}
}
