use serde::{Deserialize, Serialize};

use crate::analysis::RawMatchData;

pub enum Match {
	Qualifier(u8),
	SemiFinals(u8, u8),
	QuarterFinals(u8, u8),
	Finals(u8),
}

#[derive(Debug, Deserialize, Serialize)]
struct AllianceData {
	teams: Vec<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AlliancesData {
	blue: AllianceData,
	red: AllianceData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MatchData {
	comp_level: String,
	match_number: u32,
	set_number: u32,
	alliances: AlliancesData,
	winning_alliance: String,
}

pub fn get_match_data(match_spec: Match) -> Option<MatchData> {
	if let Ok(resp) = ureq::get(&format!(
		"https://www.thebluealliance.com/api/v3/match/{}_{}",
		option_env!("TBA_EVENT").unwrap_or(""),
		match match_spec {
			Match::Qualifier(i) => format!("qm{}", i),
			Match::SemiFinals(s, i) => format!("sf{}m{}", s, i),
			Match::QuarterFinals(s, i) => format!("qf{}m{}", s, i),
			Match::Finals(i) => format!("f1m{}", i),
		}
	))
	.set("X-TBA-Auth-Key", option_env!("TBA_AUTH_KEY").unwrap_or(""))
	.call()
	{
		if resp.status() == 200 {
			if let Ok(Ok(tba_match)) = resp
				.into_string()
				.map(|data| serde_json::from_str::<RawMatchData>(&data))
			{
				return Some(MatchData {
					comp_level: tba_match.comp_level,
					match_number: tba_match.match_number,
					set_number: tba_match.set_number,
					alliances: AlliancesData {
						blue: AllianceData {
							teams: tba_match
								.alliances
								.blue
								.team_keys
								.iter()
								.chain(tba_match.alliances.blue.surrogate_team_keys.iter())
								.map(|s| (s[3..]).parse::<u32>().unwrap())
								.collect(),
						},
						red: AllianceData {
							teams: tba_match
								.alliances
								.red
								.team_keys
								.iter()
								.chain(tba_match.alliances.red.surrogate_team_keys.iter())
								.map(|s| (s[3..]).parse::<u32>().unwrap())
								.collect(),
						},
					},
					winning_alliance: tba_match.winning_alliance,
				});
			}
		}
	}

	None
}
