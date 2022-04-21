export type DriveType = 0 | 1 | 2;

export interface TeamInfo {
	teamNumber: number;
	teamName: string | null;
	teamRookieYear: number | null;
	averageAutoScore: number;
	averageTeleopScore: number;
	averageClimbScore: number;
	averageAutoBallEfficiency: number;
	averageAutoHighGoalAccuracy: number;
	averageAutoLowGoalAccuracy: number;
	averageAutoHighGoals: number;
	averageAutoLowGoals: number;
	averageTeleopBallEfficiency: number;
	averageTeleopHighGoalAccuracy: number;
	averageTeleopLowGoalAccuracy: number;
	averageTeleopHighGoals: number;
	averageTeleopLowGoals: number;
	averageDefenceScore: number;
	averageLuckScore: number;
	climbFailRate: number;
	climbPartialSuccessRate: number;
	climbCompleteSuccessRate: number;
	climbAttemptCounts: [
		[number, number],
		[number, number],
		[number, number],
		[number, number]
	];
	climbBeforeEndgameRate: number;
	shootHubRate: number;
	shootFarRate: number;
	startLeftRate: number;
	startMiddleRate: number;
	startRightRate: number;
	opr: number;
	dpr: number;
	winCount: number;
	lossCount: number;
	overallSpeed: number;
	overallStability: number;
	overallDefence: number;
	rankingPoints: number;
	averagePeopleInPit: number;
	averagePitBusiness: number;
	averagePitChaos: number;
	friendly: boolean;
	claimedAutoBallCount: number | null;
	claimedBallCapacity: number | null;
	claimedClimbTime: number | null;
	claimedClimbEverybot: boolean;
	claimedDriveType: DriveType | null;
	claimedShooterLow: boolean;
	claimedShooterHigh: boolean;
	claimedShooterHub: boolean;
	claimedShooterFar: boolean;
	originalAutoBallCount: number | null;
	originalBallCapacity: number | null;
	originalClimbTime: number | null;
	originalClimbEverybot: boolean;
	originalDriveType: DriveType | null;
	originalShooterLow: boolean;
	originalShooterHigh: boolean;
	originalShooterHub: boolean;
	originalShooterFar: boolean;
	matches: number;
}
