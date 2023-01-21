export type MatchType = "qualification" | "practice";
export type ClimbLevel = 0 | 1 | 2 | 3 | 4;
export type ShooterPositions = 0 | 1 | 2 | 3;
export type BusinessLevel = 0 | 1 | 2;
export type CubeCapacity = 0 | 1 | 2;
export type ShooterCapability = 0 | 1 | 2 | 3;
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

export interface MatchInfo {
	type: "match_info";
	match: number;
	matchCategory: MatchType;
	team: number;
	auto: {
		preloadedCargo: boolean;
		exitedTarmac: boolean;
		startingLocation: "left" | "middle" | "right";
		cellsAcquired: number;
		lowGoalAttempts: number;
		lowGoalShots: number;
		highGoalAttempts: number;
		highGoalShots: number;
	};
	teleop: {
		cellsAcquired: number;
		lowGoalAttempts: number;
		lowGoalShots: number;
		highGoalAttempts: number;
		highGoalShots: number;
	};
	climb: {
		startedBeforeEndgame: boolean;
		highestAttempted: ClimbLevel;
		highestScored: ClimbLevel;
		fell: boolean;
	};
	speed: number;
	stability: number;
	defence: number | undefined;
	isPrimaryDefence: boolean;
	shooterPositions: ShooterPositions;
	wasBroken: boolean;
	wasDisabled: boolean;
	notes: string;
	lastModifiedTime: number;
}

export interface RobotInfo {
	type: "robot_info";
	scoutingTime: number;
	team: number;
	pit: {
		busy: BusinessLevel | undefined;
		pitPeople: number | undefined;
		chaos: number | undefined;
		friendly: boolean | undefined;
		comments: string;
	};
	robot: {
		autoBallCount: number | undefined;
		cubeCapacity: CubeCapacity | undefined;
		climbTime: number | undefined;
		climbHeight: ClimbLevel | undefined;
		climbEverybot: boolean | undefined;
		shooterCapability: ShooterCapability | undefined;
		shooterRange: ShooterPositions | undefined;
		driveType: DriveType | undefined;
		comments: string;
	};
	images: string[];
	lastModifiedTime: number;
}

export interface FullTeamInfo {
	teamNumber: number;
	matches: {
		time: number;
		alliance: "blue" | "red";
		blueTeams: [number, MatchInfo | null][];
		redTeams: [number, MatchInfo | null][];
	}[];
	images: string[];
	pitVisits: RobotInfo[];
}
