use std::cmp::Ordering;
use std::collections::HashMap;

use crate::data::{DriveType, MatchType, ShooterCapability, ShooterPositions, StartingLocation};
use serde::{Deserialize, Serialize};

use crate::Database;

#[derive(Debug, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamInfo {
	pub team_number: u32,
	pub team_name: Option<String>,
	pub team_rookie_year: Option<u32>,
	pub average_auto_score: f32,
	pub average_teleop_score: f32,
	pub average_climb_score: f32,
	pub average_auto_ball_efficiency: f32,
	pub average_auto_high_goal_accuracy: f32,
	pub average_auto_low_goal_accuracy: f32,
	pub average_auto_high_goals: f32,
	pub average_auto_low_goals: f32,
	pub average_teleop_ball_efficiency: f32,
	pub average_teleop_high_goal_accuracy: f32,
	pub average_teleop_low_goal_accuracy: f32,
	pub average_teleop_high_goals: f32,
	pub average_teleop_low_goals: f32,
	pub average_defence_score: f32,
	pub average_luck_score: f32,
	pub climb_fail_rate: f32,
	pub climb_partial_success_rate: f32,
	pub climb_complete_success_rate: f32,
	pub climb_attempt_counts: [(u32, u32); 4],
	pub climb_before_endgame_rate: f32,
	pub shoot_hub_rate: f32,
	pub shoot_far_rate: f32,
	pub start_left_rate: f32,
	pub start_middle_rate: f32,
	pub start_right_rate: f32,
	pub opr: f32,
	pub dpr: f32,
	pub win_count: u32,
	pub loss_count: u32,
	pub overall_speed: f32,
	pub overall_stability: f32,
	pub overall_defence: f32,
	pub ranking_points: f32,
	pub average_people_in_pit: f32,
	pub average_pit_business: f32,
	pub average_pit_chaos: f32,
	pub friendly: bool,
	pub claimed_auto_ball_count: Option<u32>,
	pub claimed_ball_capacity: Option<u32>,
	pub claimed_climb_time: Option<u32>,
	pub claimed_climb_everybot: bool,
	pub claimed_drive_type: Option<DriveType>,
	pub claimed_shooter_low: bool,
	pub claimed_shooter_high: bool,
	pub claimed_shooter_hub: bool,
	pub claimed_shooter_far: bool,
	pub original_auto_ball_count: Option<u32>,
	pub original_ball_capacity: Option<u32>,
	pub original_climb_time: Option<u32>,
	pub original_climb_everybot: bool,
	pub original_drive_type: Option<DriveType>,
	pub original_shooter_low: bool,
	pub original_shooter_high: bool,
	pub original_shooter_hub: bool,
	pub original_shooter_far: bool,
	pub matches_played: u32,
	pub matches_scouted: u32,
	teleop_scoring_matches: u32,
	auto_scoring_matches: u32,
	climb_attempts: u32,
	defended_teams: u32,
	auto_low_goal_scoring_matches: u32,
	auto_high_goal_scoring_matches: u32,
	teleop_low_goal_scoring_matches: u32,
	teleop_high_goal_scoring_matches: u32,
}

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
		(other.average_auto_score + other.average_teleop_score + other.average_climb_score)
			.partial_cmp(
				&(self.average_auto_score + self.average_teleop_score + self.average_climb_score),
			)
	}
}

impl Ord for TeamInfo {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

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
	pub set_number: u32,
	pub alliances: RawAlliancesData,
	pub time: u64,
	pub predicted_time: Option<u64>,
	pub actual_time: Option<u64>,
	pub winning_alliance: String,
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

pub fn analyze_data(database: &Database) -> Vec<TeamInfo> {
	let mut teams = HashMap::new();
	let mut team_info_by_team = HashMap::new();
	for team_info in database.get_all_robots().flatten() {
		let infos = team_info_by_team
			.entry(team_info.team_number)
			.or_insert_with(Vec::new);
		infos.push(team_info);
	}
	for (team_number, infos) in team_info_by_team {
		let first = infos
			.iter()
			.reduce(|first, i| {
				if i.visit_number < first.visit_number {
					i
				} else {
					first
				}
			})
			.unwrap();
		let last = infos
			.iter()
			.reduce(|last, i| {
				if i.visit_number > last.visit_number {
					i
				} else {
					last
				}
			})
			.unwrap();
		let mut team = teams
			.entry(team_number)
			.or_insert_with(|| TeamInfo::new(team_number));
		let mut total = 0;
		team.friendly = true;
		for info in &infos {
			total += 1;
			team.average_people_in_pit += info.pit.pit_people.unwrap_or(0) as f32;
			team.average_pit_business += info.pit.busy.unwrap_or(0) as f32;
			team.average_pit_chaos += info.pit.chaos.unwrap_or(0) as f32;
			team.friendly = team.friendly && info.pit.friendly.unwrap_or(true);
		}
		team.average_people_in_pit /= total as f32;
		team.average_pit_business /= total as f32;
		team.average_pit_chaos /= total as f32;
		team.claimed_auto_ball_count = last.robot.auto_ball_count;
		team.claimed_ball_capacity = last.robot.ball_capacity;
		team.claimed_climb_time = last.robot.climb_time;
		team.claimed_climb_everybot = last.robot.climb_everybot.unwrap_or(false);
		team.claimed_shooter_low = last.robot.shooter_capability == Some(ShooterCapability::Low)
			|| last.robot.shooter_capability == Some(ShooterCapability::Both);
		team.claimed_shooter_high = last.robot.shooter_capability == Some(ShooterCapability::High)
			|| last.robot.shooter_capability == Some(ShooterCapability::Both);
		team.claimed_shooter_hub = last.robot.shooter_range == Some(ShooterPositions::Hub)
			|| last.robot.shooter_range == Some(ShooterPositions::Both);
		team.claimed_shooter_far = last.robot.shooter_range == Some(ShooterPositions::Far)
			|| last.robot.shooter_range == Some(ShooterPositions::Both);
		team.claimed_drive_type = last.robot.drive_type;
		team.original_auto_ball_count = first.robot.auto_ball_count;
		team.original_ball_capacity = first.robot.ball_capacity;
		team.original_climb_time = first.robot.climb_time;
		team.original_climb_everybot = first.robot.climb_everybot.unwrap_or(false);
		team.original_shooter_low = first.robot.shooter_capability == Some(ShooterCapability::Low)
			|| first.robot.shooter_capability == Some(ShooterCapability::Both);
		team.original_shooter_high = first.robot.shooter_capability
			== Some(ShooterCapability::High)
			|| first.robot.shooter_capability == Some(ShooterCapability::Both);
		team.original_shooter_hub = first.robot.shooter_range == Some(ShooterPositions::Hub)
			|| first.robot.shooter_range == Some(ShooterPositions::Both);
		team.original_shooter_far = first.robot.shooter_range == Some(ShooterPositions::Far)
			|| first.robot.shooter_range == Some(ShooterPositions::Both);
		team.original_drive_type = first.robot.drive_type;
	}
	let mut matches_by_game = HashMap::new();
	for match_info in database.get_all_matches().flatten() {
		let team = teams
			.entry(match_info.team_number)
			.or_insert_with(|| TeamInfo::new(match_info.team_number));
		if match_info.auto.starting_location == StartingLocation::Left {
			team.start_left_rate += 1.0;
		}
		if match_info.auto.starting_location == StartingLocation::Middle {
			team.start_middle_rate += 1.0;
		}
		if match_info.auto.starting_location == StartingLocation::Right {
			team.start_right_rate += 1.0;
		}
		let auto_score = match_info.auto.low_goal_shots as f32 * 2.0
			+ match_info.auto.high_goal_shots as f32 * 4.0
			+ if match_info.auto.exited_tarmac {
				2.0
			} else {
				0.0
			};
		let teleop_score = match_info.teleop.low_goal_shots as f32
			+ match_info.teleop.high_goal_shots as f32 * 2.0;
		let climb_score = match match_info.climb.highest_scored {
			0 => 0.0,
			1 => 4.0,
			2 => 6.0,
			3 => 10.0,
			4 => 15.0,
			_ => unreachable!(),
		};
		team.average_auto_score += auto_score;
		team.average_teleop_score += teleop_score;
		team.average_climb_score += climb_score;
		let auto_shots =
			match_info.auto.low_goal_attempts as f32 + match_info.auto.high_goal_attempts as f32;
		let auto_balls = (if match_info.auto.preloaded_cargo {
			1.0
		} else {
			0.0
		} + match_info.auto.cells_acquired as f32
			+ 1.0)
			.max(auto_shots);
		if auto_balls > 0.0 {
			team.average_auto_ball_efficiency += auto_shots / auto_balls;
			team.auto_scoring_matches += 1;
		}
		if match_info.auto.low_goal_attempts > 0 {
			team.average_auto_low_goal_accuracy +=
				match_info.auto.low_goal_shots as f32 / match_info.auto.low_goal_attempts as f32;
			team.auto_low_goal_scoring_matches += 1;
		}
		if match_info.auto.high_goal_attempts > 0 {
			team.average_auto_high_goal_accuracy +=
				match_info.auto.high_goal_shots as f32 / match_info.auto.high_goal_attempts as f32;
			team.auto_high_goal_scoring_matches += 1;
		}
		team.average_auto_low_goals += match_info.auto.low_goal_shots as f32;
		team.average_auto_high_goals += match_info.auto.high_goal_shots as f32;
		let teleop_shots = match_info.teleop.low_goal_attempts as f32
			+ match_info.teleop.high_goal_attempts as f32;
		let teleop_balls = (match_info.teleop.cells_acquired as f32).max(teleop_shots);
		if teleop_balls > 0.0 {
			team.average_teleop_ball_efficiency += teleop_shots / teleop_balls;
			team.teleop_scoring_matches += 1;
		}
		if match_info.teleop.low_goal_attempts > 0 {
			team.average_teleop_low_goal_accuracy += match_info.teleop.low_goal_shots as f32
				/ match_info.teleop.low_goal_attempts as f32;
			team.teleop_low_goal_scoring_matches += 1;
		}
		if match_info.teleop.high_goal_attempts > 0 {
			team.average_teleop_high_goal_accuracy += match_info.teleop.high_goal_shots as f32
				/ match_info.teleop.high_goal_attempts as f32;
			team.teleop_high_goal_scoring_matches += 1;
		}
		team.average_teleop_low_goals += match_info.teleop.low_goal_shots as f32;
		team.average_teleop_high_goals += match_info.teleop.high_goal_shots as f32;
		if match_info.climb.fell {
			team.climb_fail_rate += 1.0;
		}
		for i in 0..match_info.climb.highest_attempted {
			team.climb_attempt_counts[i as usize].0 += 1;
		}
		for i in 0..match_info.climb.highest_scored {
			team.climb_attempt_counts[i as usize].1 += 1;
		}
		if match_info.climb.highest_attempted != 0 {
			if match_info.climb.highest_scored == match_info.climb.highest_attempted {
				team.climb_complete_success_rate += 1.0;
			}
			if match_info.climb.highest_scored != 0 {
				team.climb_partial_success_rate += 1.0;
			}
			team.climb_attempts += 1;
		}
		if match_info.climb.started_before_endgame {
			team.climb_before_endgame_rate += 1.0;
		}
		if match_info.shooter_positions == ShooterPositions::Hub
			|| match_info.shooter_positions == ShooterPositions::Both
		{
			team.shoot_hub_rate += 1.0;
		}
		if match_info.shooter_positions == ShooterPositions::Far
			|| match_info.shooter_positions == ShooterPositions::Both
		{
			team.shoot_far_rate += 1.0;
		}
		team.overall_speed += match_info.speed as f32;
		team.overall_stability += match_info.stability as f32;
		if let Some(v) = match_info.defence {
			team.overall_defence += v;
		}
		team.matches_scouted += 1;
		matches_by_game
			.entry((match_info.match_category, match_info.match_number))
			.or_insert(Vec::new())
			.push((match_info.team_number, teleop_score));
	}
	let (tba_teams, tba_matches) = get_tba_data();
	for team_info in teams.values_mut() {
		let match_count = (team_info.matches_scouted as f32).max(1.0);
		team_info.average_auto_score /= match_count;
		team_info.average_teleop_score /= match_count;
		team_info.average_climb_score /= match_count;
		team_info.average_auto_ball_efficiency /= (team_info.auto_scoring_matches as f32).max(1.0);
		team_info.average_auto_low_goal_accuracy /=
			(team_info.auto_low_goal_scoring_matches as f32).max(1.0);
		team_info.average_auto_high_goal_accuracy /=
			(team_info.auto_high_goal_scoring_matches as f32).max(1.0);
		team_info.average_auto_high_goals /= match_count;
		team_info.average_auto_low_goals /= match_count;
		team_info.average_teleop_ball_efficiency /=
			(team_info.teleop_scoring_matches as f32).max(1.0);
		team_info.average_teleop_low_goal_accuracy /=
			(team_info.teleop_low_goal_scoring_matches as f32).max(1.0);
		team_info.average_teleop_high_goal_accuracy /=
			(team_info.teleop_high_goal_scoring_matches as f32).max(1.0);
		team_info.average_teleop_high_goals /= match_count;
		team_info.average_teleop_low_goals /= match_count;
		team_info.climb_before_endgame_rate /= match_count;
		team_info.overall_speed /= match_count;
		team_info.overall_stability /= match_count;
		team_info.overall_defence /= match_count;
		team_info.climb_fail_rate /= match_count;
		team_info.climb_complete_success_rate /= (team_info.climb_attempts as f32).max(1.0);
		team_info.climb_partial_success_rate /= (team_info.climb_attempts as f32).max(1.0);
		team_info.shoot_hub_rate /= match_count;
		team_info.shoot_far_rate /= match_count;
		team_info.start_left_rate /= match_count;
		team_info.start_middle_rate /= match_count;
		team_info.start_right_rate /= match_count;
		if let Some(tba_team) = tba_teams.get(&team_info.team_number) {
			team_info.opr = tba_team.opr;
			team_info.dpr = tba_team.dpr;
			team_info.win_count = tba_team.wins;
			team_info.loss_count = tba_team.losses;
			team_info.ranking_points = tba_team.ranking_points;
			team_info.matches_played = tba_team.matches_played;
			team_info.team_name = Some(tba_team.team_name.clone());
			team_info.team_rookie_year = Some(tba_team.rookie_year);
		}
	}
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
							+ other_team.average_teleop_score
							+ other_team.average_climb_score;
						average_defence_score += other_team.average_teleop_score - teleop_score;
						defended_teams += 1;
					} else if alliance.contains(other_team_number)
						&& other_team_number != team_number
					{
						ally_scores += other_team.average_auto_score
							+ other_team.average_teleop_score
							+ other_team.average_climb_score;
						allys += 1;
					}
				}
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
	let mut average = TeamInfo {
		team_number: 0,
		..TeamInfo::default()
	};
	let mut team_list: Vec<TeamInfo> = teams
		.into_values()
		.filter(|t| t.matches_played > 0)
		.collect();
	for team_info in &team_list {
		average.average_auto_score += team_info.average_auto_score;
		average.average_teleop_score += team_info.average_teleop_score;
		average.average_climb_score += team_info.average_climb_score;
		average.average_auto_ball_efficiency += team_info.average_auto_ball_efficiency;
		average.average_auto_high_goal_accuracy += team_info.average_auto_high_goal_accuracy;
		average.average_auto_low_goal_accuracy += team_info.average_auto_low_goal_accuracy;
		average.average_auto_high_goals += team_info.average_auto_high_goals;
		average.average_auto_low_goals += team_info.average_auto_low_goals;
		average.average_teleop_ball_efficiency += team_info.average_teleop_ball_efficiency;
		average.average_teleop_high_goal_accuracy += team_info.average_teleop_high_goal_accuracy;
		average.average_teleop_low_goal_accuracy += team_info.average_teleop_low_goal_accuracy;
		average.average_teleop_high_goals += team_info.average_teleop_high_goals;
		average.average_teleop_low_goals += team_info.average_teleop_low_goals;
		average.average_defence_score += team_info.average_defence_score;
		average.average_luck_score += team_info.average_luck_score;
		average.climb_fail_rate += team_info.climb_fail_rate;
		average.climb_partial_success_rate += team_info.climb_partial_success_rate;
		average.climb_complete_success_rate += team_info.climb_complete_success_rate;
		average.climb_before_endgame_rate += team_info.climb_before_endgame_rate;
		average.shoot_hub_rate += team_info.shoot_hub_rate;
		average.shoot_far_rate += team_info.shoot_far_rate;
		average.start_left_rate += team_info.start_left_rate;
		average.start_right_rate += team_info.start_right_rate;
		average.start_middle_rate += team_info.start_middle_rate;
		for i in 0..4 {
			average.climb_attempt_counts[i].0 += team_info.climb_attempt_counts[i].0;
			average.climb_attempt_counts[i].1 += team_info.climb_attempt_counts[i].1;
		}
		average.opr += team_info.opr;
		average.dpr += team_info.dpr;
		average.win_count += team_info.win_count;
		average.loss_count += team_info.loss_count;
		average.overall_speed += team_info.overall_speed;
		average.overall_stability += team_info.overall_stability;
		average.overall_defence += team_info.overall_defence;
		average.ranking_points += team_info.ranking_points;
		average.matches_scouted += team_info.matches_scouted;
		average.matches_played += team_info.matches_played;
	}
	{
		let total_teams = (team_list.len() as u32).max(1);
		let total_teams_f = (team_list.len() as f32).max(1.0);
		average.average_auto_score /= total_teams_f;
		average.average_teleop_score /= total_teams_f;
		average.average_climb_score /= total_teams_f;
		average.average_auto_ball_efficiency /= total_teams_f;
		average.average_auto_high_goal_accuracy /= total_teams_f;
		average.average_auto_low_goal_accuracy /= total_teams_f;
		average.average_auto_high_goals /= total_teams_f;
		average.average_auto_low_goals /= total_teams_f;
		average.average_teleop_ball_efficiency /= total_teams_f;
		average.average_teleop_high_goal_accuracy /= total_teams_f;
		average.average_teleop_low_goal_accuracy /= total_teams_f;
		average.average_teleop_high_goals /= total_teams_f;
		average.average_teleop_low_goals /= total_teams_f;
		average.average_defence_score /= total_teams_f;
		average.average_luck_score /= total_teams_f;
		average.climb_fail_rate /= total_teams_f;
		average.climb_partial_success_rate /= total_teams_f;
		average.climb_complete_success_rate /= total_teams_f;
		average.climb_before_endgame_rate /= total_teams_f;
		average.shoot_hub_rate /= total_teams_f;
		average.shoot_far_rate /= total_teams_f;
		average.start_left_rate /= total_teams_f;
		average.start_right_rate /= total_teams_f;
		average.start_middle_rate /= total_teams_f;
		average.opr /= total_teams_f;
		average.dpr /= total_teams_f;
		average.win_count /= total_teams;
		average.loss_count /= total_teams;
		average.overall_speed /= total_teams_f;
		average.overall_stability /= total_teams_f;
		average.overall_defence /= total_teams_f;
		average.ranking_points /= total_teams_f;
		average.matches_scouted /= total_teams;
		average.matches_played /= total_teams;
	}
	team_list.push(average);
	team_list.sort();
	team_list
}
