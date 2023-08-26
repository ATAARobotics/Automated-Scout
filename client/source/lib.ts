// Exporting Pit Scouting values types, to clarify which values are recieved by Scouty McScout in the database file
export type MatchType = "qualification" | "practice";
export type HumanPickupRange = 0 | 1 | 2 | 3;
export type StackType = 0 | 1 | 2 | 3;
export type ConfidenceLevel = 0 | 1 | 2 | 3 | 4;
export type BumperType = 0 | 1 | 2;
export type VisionType = 0 | 1 | 2 | 3;
export type PreferredStack = 0 | 1 | 2 | 3;
export type PreferredPlay = 0 | 1 | 2 | 3;
export type ChargeBattery = 0 | 1;

// Exporting TeamInfo, these are the same as the variables in analysis
export interface TeamInfo {
	teamNumber: number;
	teamName: string | null;
	teamRookieYear: number | null;
	averageAutoScore: number;
	averageTeleopScore: number;
	averageAutoCubesPickedUp: number;
	averageAutoConesPickedUp: number;
	averageAutoHybridScore: number;
	averageAutoMiddleScore: number;
	averageAutoHighScore: number;
	averageAutoConeScore: number;
	averageAutoCubeScore: number;
	averageAutoMiddleConeScore: number;
	averageAutoMiddleCubeScore: number;
	averageAutoHighCubeScore: number;
	averageAutoHighConeScore: number;
	averageTeleopCubesPickedUp: number;
	averageTeleopConesPickedUp: number;
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
	chargeStationTeleopParked: number;
	chargeStationTeleopOn: number;
	chargeStationTeleopCharged: number;
	opr: number;
	dpr: number;
	winCount: number;
	lossCount: number;
	overallSpeed: number;
	overallStability: number;
	overallDefence: number;
	rankingPoints: number;
	matches: number;
}

// Exporting match info, make sure it matches up with the database file in scouty mcscout
export interface MatchInfo {
	type: "match_info";
	match: number;
	matchCategory: MatchType;
	team: number;
	auto: {
		exitedTarmac: boolean;
		autoChargeStation: "off" | "on" | "charged";
		conePickedUp: number;
		cubePickedUp: number;
		hybridScored: number;
		middleCubeScored: number;
		middleConeScored: number;
		highCubeScored: number;
		highConeScored: number;
	};
	teleop: {
		conePickedUp: number;
		cubePickedUp: number;
		hybridScored: number;
		middleCubeScored: number;
		middleConeScored: number;
		highCubeScored: number;
		highConeScored: number;
		teleopChargeStation: "off" | "parked" | "on" | "charged";
	};
	
	speed: number;
	stability: number;
	defence: number | undefined;
	isPrimaryDefence: boolean;
	wasDisabled: boolean;
	notes: string;
	lastModifiedTime: number;
}

// Exporting pit scouting info, make sure it matches up with the database file in scouty mcscout
export interface RobotInfo {
	type: "robot_info";
	scoutingTime: number;
	team: number;
	pit: {
		confidenceLevel: ConfidenceLevel | undefined;
		pitPeople: number | undefined;
		chaos: number | undefined;
		scoutingMethod: string;
	};
	robot: {
		bumperType: BumperType | undefined;
		visionType: VisionType | undefined;
		humanPickupRange: HumanPickupRange | undefined;
		stackType: StackType | undefined;
		preferredStack: PreferredStack | undefined;
		preferredPlay: PreferredPlay | undefined
		driveType: string;
		driveMotorAmount: number | undefined;
		otherMotorAmount: number | undefined;
		balanceTime: number | undefined;
		chargeBattery: ChargeBattery | undefined;
		batteryAmount: number | undefined;
		autoSettings: string;
		comments: string;
	};
	images: string[];
	lastModifiedTime: number;
}

// Exporting team info, taken from TBA
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
