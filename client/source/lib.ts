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
	averageAutoHybridScore: number;
	averageAutoMiddleScore: number;
	averageAutoHighScore: number;
	averageAutoConeScore: number;
	averageAutoCubeScore: number;
	averageAutoMiddleConeScore: number;
	averageAutoMiddleCubeScore: number;
	averageAutoHighCubeScore: number;
	averageAutoHighConeScore: number;
	averageTeleopHybridScore: number;
	averageTeleopMiddleScore: number;
	averageTeleopHighScore: number;
	averageTeleopConeScore: number;
	averageTeleopCubeScore: number;
	averageTeleopMiddleCubeScore: number;
	averageTeleopMiddleConeScore: number;
	averageTeleopHighCubeScore: number;
	averageTeleopHighConeScore: number;
	averageDefenceScore: number;
	averageLuckScore: number;
	averageConeScore: number;
	averageCubeScore: number;
	averageHybridScore: number;
	averageMiddleScore: number;
	averageHighScore: number;
	chargeStationAutoOff: number;
	chargeStationAutoOn: number;
	chargeStationAutoCharged: number;
	chargeStationTeleopOff: number;
	chargeStationTeleopOn: number;
	chargeStationTeleopCharged: number;
	parked: boolean;
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
		exitedTarmac: boolean;
		chargeStation: "off" | "on" | "charged";
		hybridScored: number;
		middleCubeScored: number;
		middleConeScored: number;
		highCubeScored: number;
		highConeScored: number;
	};
	teleop: {
		hybridScored: number;
		middleCubeScored: number;
		middleConeScored: number;
		highCubeScored: number;
		highConeScored: number;
		parked: boolean;
		chargeStation: "off" | "on" | "charged";
	};
	
	speed: number;
	stability: number;
	defence: number | undefined;
	isPrimaryDefence: boolean;
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
