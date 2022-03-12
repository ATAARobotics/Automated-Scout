import * as React from "react";

import { TeamInfo } from "../lib";

const COMPARISON_TEAM_NUMBERS = [0, 4421];

interface TeamListProps {
	data: TeamInfo[];
}

/**
 * Round a number to two decimal places
 *
 * @param num The number.
 * @returns The rounded number.
 */
function roundNum(num: number): string {
	return num.toFixed(2);
}

/**
 * Format a number between 0 and 1 as a percentage with one decimal place.
 *
 * @param num The number to format.
 * @returns The formatted percentage.
 */
function formatPercent(num: number): string {
	return (Math.round(num * 1000) / 10).toString() + "%";
}

/**
 * Find the greatest common divisor of two numbers.
 *
 * @param a The first number.
 * @param b The second number.
 * @returns The greatest common divisor.
 */
function gcd(a: number, b: number): number {
	if (b === 0) {
		return a;
	}
	return gcd(b, a % b);
}

/**
 * Format two numbers as a ratio.
 * This simplifies the ratio by dividing by the greatest common factor.
 *
 * @param a The first (right) number.
 * @param b The second (left) number.
 * @returns The formatted ratio.
 */
function formatRatio(a: number, b: number): string {
	a = Math.floor(a);
	b = Math.floor(b);
	let gcf = gcd(a, b);
	if (a === 0 || b === 0) {
		gcf = 1;
	}
	return `${a / gcf}:${b / gcf}`;
}

/**
 * Get a color for a team's score relative to the average score.
 *
 * @param score The score of this team.
 * @param averageScore The average score of all teams.
 * @param spread The spread between the "best" and average scores.
 * @returns The colour to represent the relative score of this team.
 */
function getColour(
	score: number,
	averageScore: number,
	spread: number,
): string {
	const relativeScore = Math.min(
		Math.max((score - averageScore) / spread, -1),
		1,
	);
	if (relativeScore > 0) {
		return `rgb(${100 - relativeScore * 50}%, 100%, ${
			100 - relativeScore * 50
		}%)`;
	} else {
		return `rgb(100%, ${100 + relativeScore * 50}%, ${
			100 + relativeScore * 50
		}%)`;
	}
}

/**
 * Render a single team's information.
 *
 * @param info The team's information.
 * @param idx The index of this row.
 * @param average The mean information for all teams, if present used to colourize this team.
 * @returns The rendered team.
 */
function renderSingleTeam(
	info: TeamInfo,
	idx: number,
	average?: TeamInfo | undefined,
): JSX.Element {
	let autoStyle: React.CSSProperties | undefined;
	let teleopStyle: React.CSSProperties | undefined;
	let climbStyle: React.CSSProperties | undefined;
	let defenceScoreStyle: React.CSSProperties | undefined;
	let speedStyle: React.CSSProperties | undefined;
	let stabilityStyle: React.CSSProperties | undefined;
	let defenceStyle: React.CSSProperties | undefined;
	let oprDprStyle: React.CSSProperties | undefined;
	let winLossStyle: React.CSSProperties | undefined;
	let scoreStyle: React.CSSProperties | undefined;
	if (average !== undefined) {
		autoStyle = {
			color: getColour(info.averageAutoScore, average.averageAutoScore, 6),
		};
		teleopStyle = {
			color: getColour(
				info.averageTeleopScore,
				average.averageTeleopScore,
				8,
			),
		};
		climbStyle = {
			color: getColour(info.averageClimbScore, average.averageClimbScore, 6),
		};
		defenceScoreStyle = {
			color: getColour(
				info.averageDefenceScore,
				average.averageDefenceScore,
				4,
			),
		};
		speedStyle = {
			color: getColour(info.overallSpeed, average.overallSpeed, 3),
		};
		stabilityStyle = {
			color: getColour(info.overallStability, average.overallStability, 3),
		};
		defenceStyle = {
			color: getColour(info.overallDefence, average.overallDefence, 3),
		};
		oprDprStyle = {
			color: getColour(info.opr - info.dpr, average.opr - average.dpr, 1),
		};
		winLossStyle = {
			color: getColour(info.winCount / info.lossCount, 1.0, 1),
		};
		scoreStyle = {
			color: getColour(
				info.averageAutoScore +
					info.averageTeleopScore +
					info.averageClimbScore +
					info.averageDefenceScore,
				average.averageAutoScore +
					average.averageTeleopScore +
					average.averageClimbScore +
					average.averageDefenceScore,
				20,
			),
		};
	}
	return (
		<div key={idx} className={`teamRow ${idx % 2 === 0 ? "even" : "odd"}`}>
			<span className="teamNumber">
				{info.teamNumber === 0 ? "Average" : info.teamNumber}
			</span>
			<span className="autoScore" style={autoStyle}>
				{roundNum(info.averageAutoScore)}&#32; (
				{info.averageAutoScore > 0
					? formatPercent(info.averageAutoBallEfficiency)
					: "-"}
				&nbsp;acc;{" "}
				{formatRatio(info.averageAutoHighGoals, info.averageAutoLowGoals)}
				&nbsp;h/l)
			</span>
			<span className="teleopScore" style={teleopStyle}>
				{roundNum(info.averageTeleopScore)}&#32; (
				{info.averageTeleopScore > 0
					? formatPercent(info.averageTeleopBallEfficiency)
					: "-"}
				&nbsp;acc;{" "}
				{formatRatio(
					info.averageTeleopHighGoals,
					info.averageTeleopLowGoals,
				)}
				&nbsp;h/l)
			</span>
			<span className="climbScore" style={climbStyle}>
				{roundNum(info.averageClimbScore).toString()} (
				{info.climbAttemptCounts
					.map(([attempts, successes]) => {
						if (attempts === 0) {
							return "-";
						}
						return formatPercent(successes / attempts);
					})
					.join(", ")}
				; {formatPercent(info.climbBeforeEndgameRate)}&nbsp;early)
			</span>
			<span className="defenceScore" style={defenceScoreStyle}>
				{info.averageDefenceScore === 0
					? "-"
					: roundNum(info.averageDefenceScore).toString()}
			</span>
			<span className="general">
				<span style={speedStyle}>
					Speed:&nbsp;{roundNum(info.overallSpeed * 2.0)}/10
				</span>
				<span style={stabilityStyle}>
					Stability:&nbsp;{roundNum(info.overallStability * 2.0)}/10
				</span>
				<span style={defenceStyle}>
					Defence:&nbsp;{roundNum(info.overallDefence * 2.0)}/10
				</span>
			</span>
			<span className="oprDpr" style={oprDprStyle}>
				{info.opr.toFixed(2)}&#8203;/&#8203;
				{info.dpr.toFixed(2)}
			</span>
			<span className="matches" style={winLossStyle}>
				{roundNum(info.matches).toString()}&#8203;/&#8203;
				{formatRatio(info.winCount, info.lossCount)}
			</span>
			<span className="score" style={scoreStyle}>
				{roundNum(
					info.averageAutoScore +
						info.averageTeleopScore +
						info.averageClimbScore,
				).toString()}{" "}
				({roundNum(info.rankingPoints)})
			</span>
		</div>
	);
}

/**
 * Component to display a list of teams.
 *
 * @param props - Component props: A list of team information structs.
 * @returns The component.
 */
export default function TeamList(props: TeamListProps): React.ReactElement {
	const averageTeam = props.data.reduce(
		(p: undefined | TeamInfo, info) =>
			p ?? (info.teamNumber === 0 ? info : undefined),
		undefined,
	);
	return (
		<div className="teamListContainer">
			<div className="teamList pinned">
				<div className="teamRow header">
					<span className="teamNumber">Team</span>
					<span className="autoScore">Auto Score (accuracy)</span>
					<span className="teleopScore">Teleop Score (accuracy)</span>
					<span className="climbScore">Climb Score (accuracy)</span>
					<span className="defenceScore">Defence Score</span>
					<span className="general">Overall</span>
					<span className="oprDpr">OPR / DPR</span>
					<span className="matches">Matches</span>
					<span className="score">Score Contribution (RP)</span>
				</div>
				{props.data
					.filter(
						(info) =>
							COMPARISON_TEAM_NUMBERS.indexOf(info.teamNumber) > -1,
					)
					.flatMap((info, idx) =>
						renderSingleTeam(info, idx, averageTeam),
					)}
			</div>
			<div className="teamList">
				{props.data
					.filter(
						(info) => COMPARISON_TEAM_NUMBERS.indexOf(info.teamNumber) < 0,
					)
					.flatMap((info, idx) =>
						renderSingleTeam(info, idx, averageTeam),
					)}
			</div>
		</div>
	);
}
