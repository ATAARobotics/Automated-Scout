use std::cmp::Ordering;
use std::collections::HashMap;

use serde::Serialize;

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
	pub win_count: u32,
	pub loss_count: u32,
	pub overall_speed: f32,
	pub overall_stability: f32,
	pub overall_defence: f32,
	pub matches: u32,
	teleop_scoring_matches: u32,
	auto_scoring_matches: u32,
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

pub fn analyze_data(database: &Database) -> Vec<TeamInfo> {
	let mut teams = HashMap::new();
	for match_info in database.get_all_matches().flatten() {
		let team = teams
			.entry(match_info.team_number)
			.or_insert_with(|| TeamInfo::new(match_info.team_number));
		team.average_auto_score += match_info.auto.low_goal_shots as f32 * 2.0
			+ match_info.auto.high_goal_shots as f32 * 4.0
			+ if match_info.auto.exited_tarmac {
				2.0
			} else {
				0.0
			};
		team.average_teleop_score += match_info.teleop.low_goal_shots as f32
			+ match_info.teleop.high_goal_shots as f32 * 2.0;
		team.average_climb_score += match match_info.climb.highest_scored {
			0 => 0.0,
			1 => 4.0,
			2 => 6.0,
			3 => 10.0,
			4 => 15.0,
			_ => unreachable!(),
		};
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
	}
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
		average.win_count += team_info.win_count;
		average.loss_count += team_info.loss_count;
		average.overall_speed += team_info.overall_speed;
		average.overall_stability += team_info.overall_stability;
		average.overall_defence += team_info.overall_defence;
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
		average.win_count /= total_teams;
		average.loss_count /= total_teams;
		average.overall_speed /= total_teams_f;
		average.overall_stability /= total_teams_f;
		average.overall_defence /= total_teams_f;
		average.matches /= total_teams;
	}
	let mut team_list: Vec<TeamInfo> = teams.into_values().collect();
	team_list.push(average);
	team_list.sort();
	team_list
}
