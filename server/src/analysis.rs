use std::cmp::Ordering;
use std::collections::HashMap;

// Make sure to edit this with anything in the data file!
use crate::data::{MatchType, AutoChargeStation, TeleopChargeStation};
use serde::{Deserialize, Serialize};

use crate::Database;

#[derive(Debug, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
// Setting Variables - These are not actually taking averages, the averages are overwritten into the variables in this file! Must match camelCase in lib file
pub struct TeamInfo {
	pub team_number: u32,
	pub team_name: Option<String>,
	pub team_rookie_year: Option<u32>,
	pub average_auto_score: f32,
	pub average_teleop_score: f32,
	//pub average_auto_cones_picked_up: f32,
	//pub average_auto_cubes_picked_up: f32,
	pub average_auto_hybrid_score: f32,
	pub average_auto_middle_score: f32,
	pub average_auto_high_score: f32,
	pub average_auto_cone_score: f32,
	pub average_auto_cube_score: f32,
	pub average_auto_hybrid_cube_score: f32,
	pub average_auto_hybrid_cone_score: f32,
	pub average_auto_middle_cube_score: f32,
	pub average_auto_middle_cone_score: f32,
	pub average_auto_high_cube_score: f32,
	pub average_auto_high_cone_score: f32,
	//pub average_teleop_cones_picked_up: f32,
	//pub average_teleop_cubes_picked_up: f32,
	pub average_teleop_hybrid_score: f32,
	pub average_teleop_middle_score: f32,
	pub average_teleop_high_score: f32,
	pub average_teleop_cone_score: f32,
	pub average_teleop_cube_score: f32,
	pub average_teleop_hybrid_cube_score: f32,
	pub average_teleop_hybrid_cone_score: f32,
	pub average_teleop_middle_cube_score: f32,
	pub average_teleop_middle_cone_score: f32,
	pub average_teleop_high_cube_score: f32,
	pub average_teleop_high_cone_score: f32,
	pub average_defence_score: f32,
	pub average_luck_score: f32,
	pub average_cone_score: f32,
	pub average_cube_score: f32,
	pub average_hybrid_score: f32,
	pub average_middle_score: f32,
	pub average_high_score: f32,
	pub charge_station_auto_off: f32,
	pub charge_station_auto_on: f32,
	pub charge_station_auto_charged: f32,
	pub charge_station_auto_other: f32,
	pub charge_station_teleop_off: f32,
	pub charge_station_teleop_parked: f32,
	pub charge_station_teleop_on: f32,
	pub charge_station_teleop_charged: f32,
	pub opr: f32,
	pub dpr: f32,
	pub win_count: u32,
	pub loss_count: u32,
	pub overall_speed: f32,
	pub overall_stability: f32,
	pub overall_defence: f32,
	pub ranking_points: f32,
	pub matches: u32,
	teleop_scoring_matches: u32,
	auto_scoring_matches: u32,
	defended_teams: u32,
	auto_hybrid_scoring_matches: u32,
	auto_medium_scoring_matches: u32,
	auto_high_scoring_matches: u32,
	teleop_hybrid_scoring_matches: u32,
	teleop_medium_scoring_matches: u32,
	teleop_high_scoring_matches: u32,
}

// Team info contains the team number and the specific data for the variable you are checking, does not contain match info!
impl TeamInfo {
	fn new(team_number: u32) -> Self {
		Self {
			team_number,

			..TeamInfo::default()
		}
	}
}

impl Eq for TeamInfo {}

impl PartialOrd for TeamInfo {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		(other.average_auto_score + other.average_teleop_score)
			.partial_cmp(
				&(self.average_auto_score + self.average_teleop_score),
			)
	}
}

impl Ord for TeamInfo {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}
// The blue alliance data
#[derive(Debug, Deserialize)]
struct RawOprData {
	dprs: HashMap<String, f32>,
	oprs: HashMap<String, f32>,
}

#[derive(Debug, Deserialize)]
struct RawStatusRecordData {
	losses: u32,
	wins: u32,
}

#[derive(Debug, Deserialize)]
struct RawStatusRankingData {
	matches_played: u32,
	record: RawStatusRecordData,
	sort_orders: Vec<f32>,
}

#[derive(Debug, Deserialize)]
struct RawStatusQualData {
	ranking: RawStatusRankingData,
}

#[derive(Debug, Deserialize)]
struct RawTeamStatusData {
	qual: Option<RawStatusQualData>,
}

#[derive(Debug, Default)]
struct TbaTeam {
	team_name: String,
	rookie_year: u32,
	opr: f32,
	dpr: f32,
	matches_played: u32,
	ranking_points: f32,
	wins: u32,
	losses: u32,
}

#[derive(Debug, Deserialize)]
struct RawTeamInfo {
	nickname: String,
	team_number: u32,
	rookie_year: u32,
}

#[derive(Debug, Deserialize)]
pub struct RawAllianceData {
	pub score: i32,
	pub team_keys: Vec<String>,
	pub surrogate_team_keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawAlliancesData {
	pub blue: RawAllianceData,
	pub red: RawAllianceData,
}

#[derive(Debug, Deserialize)]
pub struct RawMatchData {
	pub comp_level: String,
	pub match_number: u32,
	pub alliances: RawAlliancesData,
	pub time: u64,
	pub predicted_time: Option<u64>,
	pub actual_time: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct TbaMatch {
	blue_teams: Vec<u32>,
	red_teams: Vec<u32>,
}

fn get_tba_data() -> (HashMap<u32, TbaTeam>, HashMap<(MatchType, u32), TbaMatch>) {
	let mut tba_data = HashMap::new();

	if let Ok(resp) = ureq::get(&format!(
		"https://www.thebluealliance.com/api/v3/event/{}/oprs",
		option_env!("TBA_EVENT").unwrap_or("")
	))
	.set("X-TBA-Auth-Key", option_env!("TBA_AUTH_KEY").unwrap_or(""))
	.call()
	{
		if resp.status() == 200 {
			if let Ok(Ok(data)) = resp
				.into_string()
				.map(|data| serde_json::from_str::<RawOprData>(&data))
			{
				for (team, opr) in data.oprs.iter() {
					let team_number = (team[3..]).parse::<u32>().unwrap();
					tba_data
						.entry(team_number)
						.or_insert_with(TbaTeam::default)
						.opr = *opr;
				}
				for (team, dpr) in data.dprs.iter() {
					let team_number = (team[3..]).parse::<u32>().unwrap();
					tba_data
						.entry(team_number)
						.or_insert_with(TbaTeam::default)
						.dpr = *dpr;
				}
			}
		}
	}

	if let Ok(resp) = ureq::get(&format!(
		"https://www.thebluealliance.com/api/v3/event/{}/teams",
		option_env!("TBA_EVENT").unwrap_or("")
	))
	.set("X-TBA-Auth-Key", option_env!("TBA_AUTH_KEY").unwrap_or(""))
	.call()
	{
		if resp.status() == 200 {
			if let Ok(Ok(data)) = resp
				.into_string()
				.map(|data| serde_json::from_str::<Vec<RawTeamInfo>>(&data))
			{
				for team in data {
					let team_number = team.team_number;
					let mut tba_team = tba_data.entry(team_number).or_insert_with(TbaTeam::default);
					tba_team.rookie_year = team.rookie_year;
					tba_team.team_name = team.nickname;
				}
			}
		}
	}

	if let Ok(resp) = ureq::get(&format!(
		"https://www.thebluealliance.com/api/v3/event/{}/teams/statuses",
		option_env!("TBA_EVENT").unwrap_or("")
	))
	.set("X-TBA-Auth-Key", option_env!("TBA_AUTH_KEY").unwrap_or(""))
	.call()
	{
		if resp.status() == 200 {
			if let Ok(Ok(data)) = resp
				.into_string()
				.map(|data| serde_json::from_str::<HashMap<String, RawTeamStatusData>>(&data))
			{
				for (team, status) in data.iter() {
					let team_number = (team[3..]).parse::<u32>().unwrap();
					if let Some(RawStatusQualData { ranking }) = &status.qual {
						let mut tba_team =
							tba_data.entry(team_number).or_insert_with(TbaTeam::default);
						tba_team.matches_played = ranking.matches_played;
						tba_team.ranking_points = ranking.sort_orders[0]; // Should be "Average Ranking Points per Game" for this year.
						tba_team.wins = ranking.record.wins;
						tba_team.losses = ranking.record.losses;
					}
				}
			}
		}
	}

	let mut matches = HashMap::new();
	if let Ok(resp) = ureq::get(&format!(
		"https://www.thebluealliance.com/api/v3/event/{}/matches",
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
					let match_type = match tba_match.comp_level.as_str() {
						"qf" | "sf" | "f" => continue,
						"qm" => MatchType::Qualification,
						_ => MatchType::Practice,
					};
					matches.insert(
						(match_type, tba_match.match_number),
						TbaMatch {
							blue_teams: tba_match
								.alliances
								.blue
								.team_keys
								.iter()
								.chain(tba_match.alliances.blue.surrogate_team_keys.iter())
								.map(|s| (s[3..]).parse::<u32>().unwrap())
								.collect(),
							red_teams: tba_match
								.alliances
								.red
								.team_keys
								.iter()
								.chain(tba_match.alliances.red.surrogate_team_keys.iter())
								.map(|s| (s[3..]).parse::<u32>().unwrap())
								.collect(),
						},
					);
				}
			}
		}
	}

	(tba_data, matches)
}
// Start of data analysis, note that match info syntax takes in the fact whether its auto or teleop, and uses the variables in the lib file!
pub fn analyze_data(database: &Database) -> Vec<TeamInfo> {
	let mut teams = HashMap::new();
	let mut team_info_by_team = HashMap::new();
	for team_info in database.get_all_robots().flatten() {
		let infos = team_info_by_team
			.entry(team_info.team_number)
			.or_insert_with(Vec::new);
		infos.push(team_info);
	}

	// Match info
	let mut matches_by_game = HashMap::new();
	for match_info in database.get_all_matches().flatten() {
		let team = teams
			.entry(match_info.team_number)
			.or_insert_with(|| TeamInfo::new(match_info.team_number));
		// Charge station calculations, detects how many matches have a certain stat for charge station
		if match_info.auto.auto_charge_station == AutoChargeStation::Off {
			team.charge_station_auto_off += 1.0;
		}
		if match_info.auto.auto_charge_station == AutoChargeStation::On {
			team.charge_station_auto_on += 1.0;
		}
		if match_info.auto.auto_charge_station == AutoChargeStation::Charged {
			team.charge_station_auto_charged += 1.0;
		}
		if match_info.auto.auto_charge_station == AutoChargeStation::OtherRobot {
			team.charge_station_auto_other += 1.0;
		}
		if match_info.teleop.teleop_charge_station == TeleopChargeStation::Off {
			team.charge_station_teleop_off += 1.0;
		}
		if match_info.teleop.teleop_charge_station == TeleopChargeStation::Parked {
			team.charge_station_teleop_parked += 1.0;
		}
		if match_info.teleop.teleop_charge_station == TeleopChargeStation::On {
			team.charge_station_teleop_on += 1.0;
		}
		if match_info.teleop.teleop_charge_station == TeleopChargeStation::Charged {
			team.charge_station_teleop_charged += 1.0;
		}
		

		// Calculate auto score by taking all the information, and for scoring gamepieces multiplying the amount scored by the point value
		let auto_score = match_info.auto.hybrid_cube_scored as f32 * 3.0
			+ match_info.auto.hybrid_cone_scored as f32 * 3.0
			+ match_info.auto.middle_cube_scored as f32 * 4.0
			+ match_info.auto.middle_cone_scored as f32 * 4.0
			+ match_info.auto.high_cube_scored as f32 * 6.0
			+ match_info.auto.high_cone_scored as f32 * 6.0

		// Add points to auto score for exiting tarmac and charge station
			+ if match_info.auto.exited_tarmac {
				3.0
			} else {
				0.0
			}

			+ if match_info.auto.auto_charge_station == AutoChargeStation::On{
				8.0
			} else if match_info.auto.auto_charge_station == AutoChargeStation::Charged {
				12.0
			} else {
				0.0
			};	

		// Calculate teleop score by taking all the information, and for scoring gamepieces multiplying the amount scored by the point value
			let teleop_score = match_info.teleop.hybrid_cube_scored as f32 * 2.0
			+ match_info.teleop.hybrid_cone_scored as f32 * 2.0
			+ match_info.teleop.middle_cube_scored as f32 * 3.0
			+ match_info.teleop.middle_cone_scored as f32 * 3.0
			+ match_info.teleop.high_cube_scored as f32 * 5.0
			+ match_info.teleop.high_cone_scored as f32 * 5.0

		// Add amount of points charge station is worth
			+ if match_info.teleop.teleop_charge_station == TeleopChargeStation::On{
				6.0
			} else if match_info.teleop.teleop_charge_station == TeleopChargeStation::Charged {
				10.0
			} else if match_info.teleop.teleop_charge_station == TeleopChargeStation::Parked{
				2.0
			} else {
				0.0
			};	

		
	// Add the calculated scores to the average variable. 
	// Note: This is not the total average yet, just the sum of all points scored over all time for a team!
		team.average_auto_score += auto_score;
		team.average_teleop_score += teleop_score;
		
	// Calculate specific amounts of auto points scored for specific gamepieces and scoring areas
		let auto_hybrid =
			match_info.auto.hybrid_cone_scored as f32 * 3.0 + match_info.auto.hybrid_cube_scored as f32 * 3.0;
		let auto_middle =
			match_info.auto.middle_cone_scored as f32 * 4.0 + match_info.auto.middle_cube_scored as f32 * 4.0;
		let auto_high =
			match_info.auto.high_cone_scored as f32 * 6.0 + match_info.auto.high_cube_scored as f32 * 6.0;
		let auto_cone =
			match_info.auto.hybrid_cone_scored as f32 * 3.0 + match_info.auto.middle_cone_scored as f32 * 4.0 + match_info.auto.high_cone_scored as f32 * 6.0;
		let auto_cube =
			match_info.auto.hybrid_cube_scored as f32 * 3.0 + match_info.auto.middle_cube_scored as f32 * 4.0 + match_info.auto.high_cube_scored as f32 * 6.0;

	// Add calculated auto scores to average variable.
	// Note: This is not the total average yet, just the sum of all auto points scored over all time for a team!
		team.average_auto_hybrid_score += auto_hybrid;
		team.average_auto_middle_score += auto_middle;
		team.average_auto_high_score += auto_high;
		team.average_auto_cone_score += auto_cone;
		team.average_auto_cube_score += auto_cube;
	// Add match info to team info, these stats don't need more changing other then adding point value
		team.average_auto_hybrid_cone_score += match_info.auto.hybrid_cone_scored as f32 * 3.0;
		team.average_auto_hybrid_cube_score += match_info.auto.hybrid_cube_scored as f32 * 3.0;
		team.average_auto_middle_cone_score += match_info.auto.middle_cone_scored as f32 * 4.0;
		team.average_auto_middle_cube_score += match_info.auto.middle_cube_scored as f32 * 4.0;
		team.average_auto_high_cone_score += match_info.auto.high_cone_scored as f32 * 6.0;
		team.average_auto_high_cube_score += match_info.auto.high_cube_scored as f32 * 6.0;
	
	// Add match info to team info, these stats don't need changing
	/*	team.average_auto_cones_picked_up += match_info.auto.cone_picked_up as f32;
		team.average_auto_cubes_picked_up += match_info.auto.cube_picked_up as f32;
		team.average_teleop_cones_picked_up += match_info.teleop.cone_picked_up as f32;
		team.average_teleop_cubes_picked_up += match_info.teleop.cube_picked_up as f32;*/
	
	// Calculate specific amounts of teleop points scored for specific gamepieces and scoring areas
		let teleop_hybrid =
			match_info.teleop.hybrid_cone_scored as f32 * 2.0 + match_info.teleop.hybrid_cube_scored as f32 * 2.0;
		let teleop_middle =
			match_info.teleop.middle_cone_scored as f32 * 3.0 + match_info.teleop.middle_cube_scored as f32 * 3.0;
		let teleop_high =
			match_info.teleop.high_cone_scored as f32 * 5.0 + match_info.teleop.high_cube_scored as f32 * 5.0;
		let teleop_cone =
			match_info.teleop.hybrid_cone_scored as f32 * 2.0 + match_info.teleop.middle_cone_scored as f32 * 3.0 + match_info.teleop.high_cone_scored as f32 * 5.0;
		let teleop_cube =
			match_info.teleop.hybrid_cube_scored as f32 * 2.0 + match_info.teleop.middle_cube_scored as f32 * 3.0 + match_info.teleop.high_cube_scored as f32 * 5.0;

	// Add calculated teleop scores to average variable.
	// Note: This is not the total average yet, just the sum of all teleop points scored over all time for a team!
		team.average_teleop_hybrid_score += teleop_hybrid;
		team.average_teleop_middle_score += teleop_middle;
		team.average_teleop_high_score += teleop_high;
		team.average_teleop_cone_score += teleop_cone;
		team.average_teleop_cube_score += teleop_cube;
	// Add match info to team info, these stats don't need more changing other then adding point value
		team.average_teleop_hybrid_cone_score += match_info.teleop.hybrid_cone_scored as f32 * 2.0;
		team.average_teleop_hybrid_cube_score += match_info.teleop.hybrid_cube_scored as f32 * 2.0;
		team.average_teleop_middle_cone_score += match_info.teleop.middle_cone_scored as f32 * 3.0;
		team.average_teleop_middle_cube_score += match_info.teleop.middle_cube_scored as f32 * 3.0;
		team.average_teleop_high_cone_score += match_info.teleop.high_cone_scored as f32 * 5.0;
		team.average_teleop_high_cube_score += match_info.teleop.high_cube_scored as f32 * 5.0;
	// Add both auto and teleop scores to get total amount variable, again this is just total overall score not average
		team.average_cone_score += auto_cone + teleop_cone;
		team.average_cube_score += auto_cube + teleop_cube;
		team.average_hybrid_score += auto_hybrid + teleop_hybrid;
		team.average_middle_score += auto_middle + teleop_middle;
		team.average_high_score += auto_high + teleop_high;
		

	// Calculate amount of overall stats, the +1.0 is because f32 is a range of 0-4 and we want a range of 1-5
		team.overall_speed += match_info.speed as f32 + 1.0;
		team.overall_stability += match_info.stability as f32 + 1.0;
	// Check to make sure defence value is given, as we don't want data for when defence is not done in a match
		if let Some(v) = match_info.defence {
			team.overall_defence += v + 1.0;
		}
	// Add to match increment
		team.matches += 1;
		matches_by_game
			.entry((match_info.match_category, match_info.match_number))
			.or_insert(Vec::new())
			.push((match_info.team_number, teleop_score));
	}
	let (tba_teams, tba_matches) = get_tba_data();
	// The "fun" part
	for team_info in teams.values_mut() {
	// For each stat, divide the overall number from over all matches by the match count to get the proper average
		let match_count = (team_info.matches as f32).max(1.0);
		team_info.average_auto_score /= match_count;
		team_info.average_teleop_score /= match_count;
		//team_info.average_auto_cones_picked_up /= match_count;
		//team_info.average_auto_cubes_picked_up /= match_count;
		team_info.average_auto_hybrid_score /= match_count;
		team_info.average_auto_middle_score /= match_count;
		team_info.average_auto_high_score /= match_count;
		team_info.average_auto_cone_score /= match_count;
		team_info.average_auto_cube_score /= match_count;
		team_info.average_auto_hybrid_cone_score /= match_count;
		team_info.average_auto_hybrid_cube_score /= match_count;
		team_info.average_auto_middle_cone_score /= match_count;
		team_info.average_auto_middle_cube_score /= match_count;
		team_info.average_auto_high_cone_score /= match_count;
		team_info.average_auto_high_cube_score /= match_count;
		//team_info.average_teleop_cones_picked_up /= match_count;
		//team_info.average_teleop_cubes_picked_up /= match_count;
		team_info.average_teleop_hybrid_score /= match_count;
		team_info.average_teleop_middle_score /= match_count;
		team_info.average_teleop_high_score /= match_count;
		team_info.average_teleop_cone_score /= match_count;
		team_info.average_teleop_cube_score /= match_count;
		team_info.average_teleop_hybrid_cone_score /= match_count;
		team_info.average_teleop_hybrid_cube_score /= match_count;
		team_info.average_teleop_middle_cone_score /= match_count;
		team_info.average_teleop_middle_cube_score /= match_count;
		team_info.average_teleop_high_cone_score /= match_count;
		team_info.average_teleop_high_cube_score /= match_count;
		team_info.overall_speed /= match_count;
		team_info.overall_stability /= match_count;
		team_info.overall_defence /= match_count;
		team_info.average_cone_score /= match_count;
		team_info.average_cube_score /= match_count;
		team_info.average_hybrid_score /= match_count;
		team_info.average_middle_score /= match_count;
		team_info.average_high_score /= match_count;
		team_info.charge_station_auto_off /= match_count-team_info.charge_station_auto_other;
		team_info.charge_station_auto_on /= match_count-team_info.charge_station_auto_other;
		team_info.charge_station_auto_charged /= match_count-team_info.charge_station_auto_other;
		team_info.charge_station_teleop_off /= match_count;
		team_info.charge_station_teleop_parked /= match_count;
		team_info.charge_station_teleop_on /= match_count;
		team_info.charge_station_teleop_charged /= match_count;
		
		// TBA Data
		if let Some(tba_team) = tba_teams.get(&team_info.team_number) {
			team_info.opr = tba_team.opr;
			team_info.dpr = tba_team.dpr;
			team_info.win_count = tba_team.wins;
			team_info.loss_count = tba_team.losses;
			team_info.ranking_points = tba_team.ranking_points;
			team_info.matches = tba_team.matches_played;
			team_info.team_name = Some(tba_team.team_name.clone());
			team_info.team_rookie_year = Some(tba_team.rookie_year);
		}
	}
	// Defence score calculation by taking into account rated defence and TBA Data, has not been touched
	for ((match_type, match_id), matches) in matches_by_game.iter() {
		if let Some(alliances) = tba_matches.get(&(*match_type, *match_id)) {
			for (team_number, ..) in matches {
				let tmp = Vec::new();
				let (opponents, alliance) = if alliances.blue_teams.contains(team_number) {
					(&alliances.red_teams, &alliances.blue_teams)
				} else if alliances.red_teams.contains(team_number) {
					(&alliances.blue_teams, &alliances.red_teams)
				} else {
					(&tmp, &tmp)
				};
				let (
					mut average_defence_score,
					mut opponent_scores,
					mut ally_scores,
					mut defended_teams,
					mut allys,
				) = (0.0, 0.0, 0.0, 0, 0);
				for (other_team_number, teleop_score) in matches {
					let other_team = &teams[other_team_number];
					if opponents.contains(other_team_number) && other_team_number != team_number {
						opponent_scores += other_team.average_auto_score
							+ other_team.average_teleop_score;
						average_defence_score += other_team.average_teleop_score - teleop_score;
						defended_teams += 1;
					} else if alliance.contains(other_team_number)
						&& other_team_number != team_number
					{
						ally_scores += other_team.average_auto_score
							+ other_team.average_teleop_score;
						allys += 1;
					}
				}
				// Luck score calculation, if ally scores are better then luck goes up, if worse luck goes down
				let team = teams.get_mut(team_number).unwrap();
				if defended_teams > 0 {
					team.average_luck_score += ally_scores / (defended_teams as f32);
				}
				if allys > 0 {
					team.average_luck_score -= opponent_scores / (allys as f32);
				}
				team.average_defence_score += average_defence_score;
				team.defended_teams += defended_teams;
			}
		}
	}
	for team_info in teams.values_mut() {
	// Printing stats, this can be removed to clear terminal if wished
		println!(
			"{}, {}, {}",
			team_info.average_defence_score,
			team_info.defended_teams,
			team_info.average_defence_score / team_info.defended_teams as f32
		);
		if team_info.defended_teams > 0 {
			team_info.average_defence_score /= team_info.defended_teams as f32;
			team_info.average_luck_score /= team_info.defended_teams as f32;
		}
	}
	// Setting up average team, the average team has the average stats of all teams
	let mut average = TeamInfo {
		team_number: 0,
		..TeamInfo::default()
	};
	for team_info in teams.values() {
	// Add onto the average score the averages of all teams
		average.average_auto_score += team_info.average_auto_score;
		average.average_teleop_score += team_info.average_teleop_score;
		//average.average_auto_cones_picked_up += team_info.average_auto_cones_picked_up;
		//average.average_auto_cubes_picked_up += team_info.average_auto_cubes_picked_up;
		average.average_cone_score += team_info.average_cone_score;
		average.average_cube_score += team_info.average_cube_score;
		average.average_auto_cone_score += team_info.average_auto_cone_score;
		average.average_auto_cube_score += team_info.average_auto_cube_score;
		average.average_teleop_cone_score += team_info.average_teleop_cone_score;
		average.average_teleop_cube_score += team_info.average_teleop_cube_score;
		average.average_auto_hybrid_score += team_info.average_auto_hybrid_score;
		average.average_auto_middle_score += team_info.average_auto_middle_score;
		average.average_auto_high_score += team_info.average_auto_high_score;
		average.average_auto_hybrid_cone_score += team_info.average_auto_hybrid_cone_score;
		average.average_auto_hybrid_cube_score += team_info.average_auto_hybrid_cube_score;
		average.average_auto_middle_cone_score += team_info.average_auto_middle_cone_score;
		average.average_auto_middle_cube_score += team_info.average_auto_middle_cube_score;
		average.average_auto_high_cone_score += team_info.average_auto_high_cone_score;
		average.average_auto_high_cube_score += team_info.average_auto_high_cube_score;
		//average.average_teleop_cones_picked_up += team_info.average_teleop_cones_picked_up;
		//average.average_teleop_cubes_picked_up += team_info.average_teleop_cubes_picked_up;
		average.average_teleop_hybrid_score += team_info.average_teleop_hybrid_score;
		average.average_teleop_middle_score += team_info.average_teleop_middle_score;
		average.average_teleop_high_score += team_info.average_teleop_high_score;
		average.average_teleop_hybrid_cone_score += team_info.average_teleop_hybrid_cone_score;
		average.average_teleop_hybrid_cube_score += team_info.average_teleop_hybrid_cube_score;
		average.average_teleop_middle_cone_score += team_info.average_teleop_middle_cone_score;
		average.average_teleop_middle_cube_score += team_info.average_teleop_middle_cube_score;
		average.average_teleop_high_cone_score += team_info.average_teleop_high_cone_score;
		average.average_teleop_high_cube_score += team_info.average_teleop_high_cube_score;
		average.average_defence_score += team_info.average_defence_score;
		average.average_luck_score += team_info.average_luck_score;
		average.charge_station_auto_off += team_info.charge_station_auto_off;
		average.charge_station_auto_on += team_info.charge_station_auto_on;
		average.charge_station_auto_charged += team_info.charge_station_auto_charged;
		average.charge_station_teleop_off += team_info.charge_station_teleop_off;
		average.charge_station_teleop_parked += team_info.charge_station_teleop_parked;
		average.charge_station_teleop_on += team_info.charge_station_teleop_on;
		average.charge_station_teleop_charged += team_info.charge_station_teleop_charged;
		average.opr += team_info.opr;
		average.dpr += team_info.dpr;
		average.win_count += team_info.win_count;
		average.loss_count += team_info.loss_count;
		average.overall_speed += team_info.overall_speed;
		average.overall_stability += team_info.overall_stability;
		average.overall_defence += team_info.overall_defence;
		average.ranking_points += team_info.ranking_points;
		average.matches += team_info.matches;
	}
	{
	// For every team that has been scouted, divide the average team by that amount
		let total_teams = (teams.len() as u32).max(1);
		let total_teams_f = (teams.len() as f32).max(1.0);
		average.average_auto_score /= total_teams_f;
		average.average_teleop_score /= total_teams_f;

		//average.average_auto_cones_picked_up /= total_teams_f;
		//average.average_auto_cubes_picked_up /= total_teams_f;
		average.average_auto_cone_score /= total_teams_f;
		average.average_auto_cube_score /= total_teams_f;
		average.average_auto_high_cone_score /= total_teams_f;
		average.average_auto_high_cube_score /= total_teams_f;
		average.average_auto_high_score /= total_teams_f;
		average.average_auto_hybrid_score /= total_teams_f;
		average.average_auto_hybrid_cone_score /= total_teams_f;
		average.average_auto_hybrid_cube_score /= total_teams_f;
		average.average_auto_middle_cone_score /= total_teams_f;
		average.average_auto_middle_cube_score /= total_teams_f;
		average.average_auto_middle_score /= total_teams_f;
		average.average_cone_score /= total_teams_f;
		average.average_cube_score /= total_teams_f;
		average.average_defence_score /= total_teams_f;
		average.average_high_score /= total_teams_f;
		average.average_hybrid_score /= total_teams_f;
		average.average_luck_score /= total_teams_f;
		average.average_middle_score /= total_teams_f;
		//average.average_teleop_cones_picked_up /= total_teams_f;
		//average.average_teleop_cubes_picked_up /= total_teams_f;
		average.average_teleop_cone_score /= total_teams_f;
		average.average_teleop_cube_score /= total_teams_f;
		average.average_teleop_hybrid_cone_score /= total_teams_f;
		average.average_teleop_hybrid_cube_score /= total_teams_f;
		average.average_teleop_high_cone_score /= total_teams_f;
		average.average_teleop_high_cube_score /= total_teams_f;
		average.average_teleop_high_score /= total_teams_f;
		average.average_teleop_hybrid_score /= total_teams_f;
		average.average_teleop_middle_cone_score /= total_teams_f;
		average.average_teleop_middle_score /= total_teams_f;
		average.average_teleop_score /= total_teams_f;
		average.charge_station_auto_charged /= total_teams_f;
		average.charge_station_auto_off /= total_teams_f;
		average.charge_station_auto_on /= total_teams_f;
		average.charge_station_teleop_charged /= total_teams_f;
		average.charge_station_teleop_off /= total_teams_f;
		average.charge_station_teleop_parked /= total_teams_f;
		average.charge_station_teleop_on /= total_teams_f;
		average.opr /= total_teams_f;
		average.dpr /= total_teams_f;
		average.win_count /= total_teams;
		average.loss_count /= total_teams;
		average.overall_speed /= total_teams_f;
		average.overall_stability /= total_teams_f;
		average.overall_defence /= total_teams_f;
		average.ranking_points /= total_teams_f;
		average.matches /= total_teams;
	}
	// Put it all in team list to collect the data and push
	let mut team_list: Vec<TeamInfo> = teams.into_values().collect();
	team_list.push(average);
	team_list.sort();
	team_list
}
