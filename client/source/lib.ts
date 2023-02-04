export type MatchType = "qualification" | "practice";
export type StackType = 0 | 1 | 2 | 3;
export type StackRange = 0 | 1 | 2 | 3 | 4;
export type BusinessLevel = 0 | 1 | 2;
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
	claimedBalanceTime: number | null;
	claimedEverybot: boolean;
	claimedDriveType: DriveType | null;
	claimedStackCone: boolean;
	claimedStackCube: boolean;
	claimedStackHybrid: boolean;
	claimedStackMiddle: boolean;
	claimedStackHigh: boolean;
	originalBalanceTime: number | null;
	originalEverybot: boolean;
	originalDriveType: DriveType | null;
	originalStackCone: boolean;
	originalStackCube: boolean;
	originalStackHybrid: boolean;
	originalStackMiddle: boolean;
	originalStackHigh: boolean;
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
		stackType: StackType | undefined;
		stackRange: StackRange | undefined;
		driveType: DriveType | undefined;
		balanceTime: number | undefined;
		everybot: boolean | undefined;
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
