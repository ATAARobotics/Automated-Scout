use std::cmp::Ordering;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::Database;

#[derive(Debug, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamInfo {
	pub team_number: u32,
	pub average_auto_score: f32,
	pub average_teleop_score: f32,
	pub average_climb_score: f32,
	pub average_auto_ball_efficiency: f32,
	pub average_auto_high_goals: f32,
	pub average_auto_low_goals: f32,
	pub average_teleop_ball_efficiency: f32,
	pub average_teleop_high_goals: f32,
	pub average_teleop_low_goals: f32,
	pub average_defence_score: f32,
	pub climb_fail_rate: f32,
	pub climb_attempt_counts: [(u32, u32); 4],
	pub climb_before_endgame_rate: f32,
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
	opr: f32,
	dpr: f32,
	matches_played: u32,
	ranking_points: f32,
	wins: u32,
	losses: u32,
}

fn get_tba_data() -> HashMap<u32, TbaTeam> {
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

	tba_data
}

pub fn analyze_data(database: &Database) -> Vec<TeamInfo> {
	let mut teams = HashMap::new();
	let mut matches_by_game = HashMap::new();
	for match_info in database.get_all_matches().flatten() {
		let team = teams
			.entry(match_info.team_number)
			.or_insert_with(|| TeamInfo::new(match_info.team_number));
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
			match_info.auto.low_goal_shots as f32 + match_info.auto.high_goal_shots as f32;
		let auto_balls = (match_info.auto.cells_acquired as f32 + 1.0
			- match_info.auto.cells_dropped as f32)
			.max(auto_shots);
		if auto_balls > 0.0 {
			team.average_auto_ball_efficiency += auto_shots / auto_balls;
			team.auto_scoring_matches += 1;
		}
		team.average_auto_low_goals += match_info.auto.low_goal_shots as f32;
		team.average_auto_high_goals += match_info.auto.high_goal_shots as f32;
		let teleop_shots =
			match_info.teleop.low_goal_shots as f32 + match_info.teleop.high_goal_shots as f32;
		let teleop_balls = (match_info.teleop.cells_acquired as f32
			- match_info.teleop.cells_dropped as f32)
			.max(teleop_shots);
		if teleop_balls > 0.0 {
			team.average_teleop_ball_efficiency += teleop_shots / teleop_balls;
			team.teleop_scoring_matches += 1;
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
		if match_info.climb.started_before_endgame {
			team.climb_before_endgame_rate += 1.0;
		}
		team.overall_speed += match_info.speed.unwrap_or(0.5) as f32;
		team.overall_stability += match_info.stability.unwrap_or(0.5) as f32;
		team.overall_defence += match_info.stability.unwrap_or(0.5) as f32;
		team.matches += 1;
		matches_by_game
			.entry((match_info.match_category, match_info.match_number))
			.or_insert(Vec::new())
			.push((match_info.team_number, teleop_score));
	}
	let tba_data = get_tba_data();
	for team_info in teams.values_mut() {
		let match_count = team_info.matches as f32;
		team_info.average_auto_score /= match_count;
		team_info.average_teleop_score /= match_count;
		team_info.average_climb_score /= match_count;
		team_info.average_auto_ball_efficiency /= (team_info.auto_scoring_matches as f32).max(1.0);
		team_info.average_auto_high_goals /= match_count;
		team_info.average_auto_low_goals /= match_count;
		team_info.average_teleop_ball_efficiency /=
			(team_info.teleop_scoring_matches as f32).max(1.0);
		team_info.average_teleop_high_goals /= match_count;
		team_info.average_teleop_low_goals /= match_count;
		team_info.climb_before_endgame_rate /= match_count;
		team_info.overall_speed /= match_count;
		team_info.overall_stability /= match_count;
		team_info.overall_defence /= match_count;
		team_info.climb_fail_rate /= match_count;
		if let Some(tba_team) = tba_data.get(&team_info.team_number) {
			team_info.opr = tba_team.opr;
			team_info.dpr = tba_team.dpr;
			team_info.win_count = tba_team.wins;
			team_info.loss_count = tba_team.losses;
			team_info.ranking_points = tba_team.ranking_points;
			team_info.matches = tba_team.matches_played;
		}
	}
	for matches in matches_by_game.values() {
		for (team_number, ..) in matches {
			let (mut average_defence_score, mut defended_teams) = (0.0, 0);
			for (other_team_number, teleop_score) in matches {
				if other_team_number != team_number {
					let other_team = &teams[other_team_number];
					average_defence_score += other_team.average_teleop_score - teleop_score;
					defended_teams += 1;
				}
			}
			let team = teams.get_mut(team_number).unwrap();
			team.average_defence_score += average_defence_score;
			team.defended_teams += defended_teams;
		}
	}
	for team_info in teams.values_mut() {
		team_info.average_defence_score /= team_info.defended_teams as f32;
	}
	let mut average = TeamInfo {
		team_number: 0,
		..TeamInfo::default()
	};
	for team_info in teams.values() {
		average.average_auto_score += team_info.average_auto_score;
		average.average_teleop_score += team_info.average_teleop_score;
		average.average_climb_score += team_info.average_climb_score;
		average.average_auto_ball_efficiency += team_info.average_auto_ball_efficiency;
		average.average_auto_high_goals += team_info.average_auto_high_goals;
		average.average_auto_low_goals += team_info.average_auto_low_goals;
		average.average_teleop_ball_efficiency += team_info.average_teleop_ball_efficiency;
		average.average_teleop_high_goals += team_info.average_teleop_high_goals;
		average.average_teleop_low_goals += team_info.average_teleop_low_goals;
		average.average_defence_score += team_info.average_defence_score;
		average.climb_fail_rate += team_info.climb_fail_rate;
		for i in 0..4 {
			average.climb_attempt_counts[i].0 += team_info.climb_attempt_counts[i].0;
			average.climb_attempt_counts[i].1 += team_info.climb_attempt_counts[i].1;
		}
		average.climb_before_endgame_rate += team_info.climb_before_endgame_rate;
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
		let total_teams = teams.len() as u32;
		let total_teams_f = teams.len() as f32;
		average.average_auto_score /= total_teams_f;
		average.average_teleop_score /= total_teams_f;
		average.average_climb_score /= total_teams_f;
		average.average_auto_ball_efficiency /= total_teams_f;
		average.average_auto_high_goals /= total_teams_f;
		average.average_auto_low_goals /= total_teams_f;
		average.average_teleop_ball_efficiency /= total_teams_f;
		average.average_teleop_high_goals /= total_teams_f;
		average.average_teleop_low_goals /= total_teams_f;
		average.average_defence_score /= total_teams_f;
		average.climb_fail_rate /= total_teams_f;
		average.climb_before_endgame_rate /= total_teams_f;
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
	let mut team_list: Vec<TeamInfo> = teams.into_values().collect();
	team_list.push(average);
	team_list.sort();
	team_list
}
