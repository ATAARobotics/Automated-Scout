export interface TeamInfo {
	teamNumber: number;
	averageAutoScore: number;
	averageTeleopScore: number;
	averageClimbScore: number;
	averageAutoBallEfficiency: number;
	averageAutoHighGoals: number;
	averageAutoLowGoals: number;
	averageTeleopBallEfficiency: number;
	averageTeleopHighGoals: number;
	averageTeleopLowGoals: number;
	averageDefenceScore: number;
	climbFailRate: number;
	climbAttemptCounts: [number, number][];
	climbBeforeEndgameRate: number;
	winCount: number;
	lossCount: number;
	overallSpeed: number;
	overallStability: number;
	overallDefence: number;
	matches: number;
}
