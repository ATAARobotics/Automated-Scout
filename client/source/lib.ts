export type DriveType = 0 | 1 | 2;

export interface TeamInfo {
	teamNumber: number;
	averageAutoScore: number;
	averageTeleopScore: number;
	averageClimbScore: number;
	averageAutoBallEfficiency: number;
	average_auto_high_goal_accuracy: number,
	average_auto_low_goal_accuracy: number,
	averageAutoHighGoals: number;
	averageAutoLowGoals: number;
	averageTeleopBallEfficiency: number;
	average_teleop_high_goal_accuracy: number,
	average_teleop_low_goal_accuracy: number,
	averageTeleopHighGoals: number;
	averageTeleopLowGoals: number;
	averageDefenceScore: number;
	average_luck_score: number,
	climbFailRate: number;
	climb_partial_success_rate: number,
	climb_complete_success_rate: number,
	climbAttemptCounts: [
		[number, number],
		[number, number],
		[number, number],
		[number, number]
	];
	climbBeforeEndgameRate: number;
	shoot_hub_rate: number,
	shoot_far_rate: number,
	start_left_rate: number,
	start_middle_rate: number,
	start_right_rate: number,
	opr: number;
	dpr: number;
	winCount: number;
	lossCount: number;
	overallSpeed: number;
	overallStability: number;
	overallDefence: number;
	rankingPoints: number;
	average_people_in_pit: number,
	average_pit_business: number,
	average_pit_chaos: number,
	friendly: boolean,
	claimed_auto_ball_count: number | null,
	claimed_ball_capacity: number | null,
	claimed_climb_time: number | null,
	claimed_climb_everybot: boolean,
	claimed_drive_type: DriveType | null,
	claimed_shooter_low: boolean,
	claimed_shooter_high: boolean,
	claimed_shooter_hub: boolean,
	claimed_shooter_far: boolean,
	original_auto_ball_count: number | null,
	original_ball_capacity: number | null,
	original_climb_time: number | null,
	original_climb_everybot: boolean,
	original_drive_type: DriveType | null,
	original_shooter_low: boolean,
	original_shooter_high: boolean,
	original_shooter_hub: boolean,
	original_shooter_far: boolean,
	matches: number;
}
