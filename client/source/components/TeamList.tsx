import * as React from "react";

import { TeamInfo } from "../lib";

const COMPARISON_TEAM_NUMBERS = [0, 4421];

interface TeamListProps {
	data: TeamInfo[];
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
 * Render a single team's information.
 *
 * @param info The team's information.
 * @param idx The index of this row.
 * @returns The rendered team.
 */
function renderSingleTeam(info: TeamInfo, idx: number): JSX.Element {
	return (
		<div key={idx} className={`teamRow ${idx % 2 === 0 ? "even" : "odd"}`}>
			<span className="teamNumber">
				{info.teamNumber === 0 ? "Average" : info.teamNumber}
			</span>
			<span className="autoScore">{Math.round(info.averageAutoScore)}</span>
			<span className="autoEfficiency">
				{info.averageAutoScore > 0
					? formatPercent(info.averageAutoBallEfficiency)
					: "-"}
			</span>
			<span className="autoHighLowGoal">
				{formatRatio(info.averageAutoHighGoals, info.averageAutoLowGoals)}
			</span>
			<span className="teleopScore">
				{Math.round(info.averageTeleopScore)}
			</span>
			<span className="teleopEfficiency">
				{info.averageTeleopScore > 0
					? formatPercent(info.averageTeleopBallEfficiency)
					: "-"}
			</span>
			<span className="teleopHighLowGoal">
				{formatRatio(
					info.averageTeleopHighGoals,
					info.averageTeleopLowGoals,
				)}
			</span>
			<span className="climbScore">
				{Math.round(info.averageClimbScore).toString()}
			</span>
			<span className="climbAccuracy">
				{info.climbAttemptCounts
					.map(([attempts, successes]) => {
						if (attempts === 0) {
							return "-";
						}
						return formatPercent(successes / attempts);
					})
					.join(", ")}
			</span>
			<span className="climbStartEarlyRate">
				{formatPercent(info.climbBeforeEndgameRate)}
			</span>
			<span className="defenceScore">
				{info.averageDefenceScore === 0
					? "-"
					: Math.round(info.averageDefenceScore).toString()}
			</span>
			<span className="winLossRatio">
				{formatRatio(info.winCount, info.lossCount)}
			</span>
			<span className="general">
				Speed:&nbsp;{Math.round(info.overallSpeed * 10.0)}/10
				Stability:&nbsp;{Math.round(info.overallStability * 10.0)}/10
				Defence:&nbsp;{Math.round(info.overallDefence * 10.0)}/10
			</span>
			<span className="score">
				{Math.round(
					info.averageAutoScore +
						info.averageTeleopScore +
						info.averageClimbScore,
				).toString()}
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
	return (
		<div className="teamListContainer">
			<div className="teamList pinned">
				<div className="teamRow">
					<span className="teamNumber">Team</span>
					<span className="autoScore">Auto Score</span>
					<span className="autoEfficiency">Auto Efficiency</span>
					<span className="autoHighLowGoal">Auto High:Low Goal Ratio</span>
					<span className="teleopScore">Teleop Score</span>
					<span className="teleopEfficiency">Teleop Efficiency</span>
					<span className="teleopHighLowGoal">
						Teleop High:Low Goal Ratio
					</span>
					<span className="climbScore">Climb Score</span>
					<span className="climbAccuracy">Climb Accuracy</span>
					<span className="climbStartEarlyRate">
						Climb Start Early Rate
					</span>
					<span className="defenceScore">Defence Score</span>
					<span className="winLossRatio">Win:Loss Ratio</span>
					<span className="general">Overall</span>
					<span className="score">Score Contribution</span>
				</div>
				{props.data
					.filter(
						(info) =>
							COMPARISON_TEAM_NUMBERS.indexOf(info.teamNumber) > -1,
					)
					.flatMap((info, idx) => renderSingleTeam(info, idx))}
			</div>
			<div className="teamList">
				{props.data
					.filter(
						(info) => COMPARISON_TEAM_NUMBERS.indexOf(info.teamNumber) < 0,
					)
					.flatMap((info, idx) => renderSingleTeam(info, idx))}
			</div>
		</div>
	);
}
