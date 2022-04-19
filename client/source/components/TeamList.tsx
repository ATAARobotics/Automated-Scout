import * as React from "react";
import { Table, Column, HeaderCell, Cell, RowDataType } from "rsuite-table";
import { CustomProvider } from "rsuite";
import "rsuite/dist/rsuite.min.css";

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
	return num.toFixed(1) + "%";
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

const order: [string, (match: TeamInfo) => string][] = [
	[
		"Team",
		(match: TeamInfo) => match.teamNumber,
		(match: TeamInfo) =>
			(match.teamNumber === 0 ? "Avg." : match.teamNumber.toFixed(0)),
		1,
		"left",
		false,
	],
	[
		"Auto Sc.",
		(match: TeamInfo) => match.averageAutoScore,
		(match: TeamInfo) => match.averageAutoScore.toFixed(1),
		0.75,
		false,
		5.0,
	],
	[
		"Auto Acc",
		(match: TeamInfo) => match.averageAutoBallEfficiency,
		(match: TeamInfo) => formatPercent(match.averageAutoBallEfficiency),
		1,
		false,
		5.0,
	],
	[
		"Auto H:L",
		(match: TeamInfo) =>
			match.averageAutoHighGoals / match.averageAutoLowGoals,
		(match: TeamInfo) =>
			formatRatio(match.averageAutoHighGoals, match.averageAutoLowGoals),
		1,
		false,
		5.0,
	],
	[
		"Tele Sc.",
		(match: TeamInfo) =>
			match.averageAutoHighGoals / match.averageAutoLowGoals,
		(match: TeamInfo) => match.averageTeleopScore.toFixed(1),
		0.75,
		false,
		5.0,
	],
	[
		"Tele Acc",
		(match: TeamInfo) => match.averageTeleopBallEfficiency,
		(match: TeamInfo) => formatPercent(match.averageTeleopBallEfficiency),
		1,
		false,
		5.0,
	],
	[
		"Tele H:L",
		(match: TeamInfo) =>
			match.averageTeleopHighGoals / match.averageTeleopLowGoals,
		(match: TeamInfo) =>
			formatRatio(match.averageTeleopHighGoals, match.averageTeleopLowGoals),
		1,
		false,
		5.0,
	],
	[
		"Climb Sc.",
		(match: TeamInfo) => match.averageClimbScore,
		(match: TeamInfo) => match.averageClimbScore.toFixed(1),
		0.75,
		false,
		5.0,
	],
	[
		"Climb Acc",
		(match: TeamInfo) => 1.0 - match.climbFailRate,
		(match: TeamInfo) =>
			formatPercent(100.0 - match.climbFailRate * 100.0) +
			" (" +
			match.climbAttemptCounts
				.map((n) => ((n[1] / n[0]) * 100).toFixed(0) + "%")
				.join(", ") +
			")",
		3,
		false,
		5.0,
	],
	[
		"Climb Erly.",
		(match: TeamInfo) => match.climbBeforeEndgameRate,
		(match: TeamInfo) => formatPercent(match.climbBeforeEndgameRate),
		1,
		false,
		5.0,
	],
	[
		"Speed",
		(match: TeamInfo) => match.overallSpeed,
		(match: TeamInfo) => match.overallSpeed.toFixed(1) + " / 5",
		1,
		false,
		5.0,
	],
	[
		"Stability",
		(match: TeamInfo) => match.overallStability,
		(match: TeamInfo) => match.overallStability.toFixed(1) + " / 5",
		1,
		false,
		5.0,
	],
	[
		"Defence",
		(match: TeamInfo) => match.overallDefence,
		(match: TeamInfo) => match.overallDefence.toFixed(1) + " / 5",
		1,
		false,
		5.0,
	],
	[
		"OPR",
		(match: TeamInfo) => match.opr,
		(match: TeamInfo) => match.opr.toFixed(1),
		1,
		false,
		5.0,
	],
	[
		"DPR",
		(match: TeamInfo) => match.dpr,
		(match: TeamInfo) => match.dpr.toFixed(1),
		1,
		false,
		5.0,
	],
	[
		"Matches",
		(match: TeamInfo) => match.matches,
		(match: TeamInfo) => match.matches.toFixed(0),
		0.75,
		false,
		false,
	],
	[
		"W:L",
		(match: TeamInfo) => match.winCount / match.lossCount,
		(match: TeamInfo) => formatRatio(match.winCount, match.lossCount),
		1,
		"right",
		5.0,
	],
	[
		"RP",
		(match: TeamInfo) => match.rankingPoints,
		(match: TeamInfo) => match.rankingPoints.toFixed(1),
		0.5,
		"right",
		5.0,
	],
	[
		"Avg. Sc.",
		(match: TeamInfo) =>
			match.averageAutoScore +
			match.averageTeleopScore +
			match.averageClimbScore,
		(match: TeamInfo) =>
			(
				match.averageAutoScore +
				match.averageTeleopScore +
				match.averageClimbScore
			).toFixed(1),
		1,
		"right",
		5.0,
	],
];

/**
 * Component to display a list of teams.
 *
 * @param props - Component props: A list of team information structs.
 * @returns The component.
 */
export default function TeamList(props: TeamListProps): React.ReactElement {
	const [sortColumn, setSortColumn] = React.useState("Avg. Sc.");
	const [sortType, setSortType] = React.useState<"desc" | "asc">("desc");
	console.log(sortColumn, sortType);
	const data = props.data
		.map((row) =>
			order.reduce(
				(o, col) => ({
					...o,
					[col[0]]: { sortValue: col[1](row), prettyValue: col[2](row) },
				}),
				{},
			),
		)
		.sort((a, b) => {
			const diff = a[sortColumn].sortValue - b[sortColumn].sortValue;
			if (sortType === "desc") {
				return -diff;
			}
			return diff;
		});
	const averageTeam = data.reduce(
		(p: undefined | TeamInfo, info) =>
			p ?? (info["Team"].sortValue === 0 ? info : undefined),
		undefined,
	);
	return (
		<CustomProvider theme="dark">
			<div className="teamListContainer">
				<Table
					wordWrap
					headerHeight={80}
					height={800}
					data={data}
					sortColumn={sortColumn}
					sortType={sortType}
					onSortColumn={(newSortColumn, newSortType) => {
						setSortColumn(newSortColumn);
						setSortType(newSortType);
					}}
				>
					{order.map((col) => (
						<Column
							fixed={col[4]}
							flexGrow={col[3]}
							key={col[0]}
							sortable
						>
							<HeaderCell>{col[0]}</HeaderCell>
							<Cell dataKey={col[0]}>
								{(
									rowData: RowDataType,
									rowIndex: number | undefined,
								) => {
									const val = data[rowIndex ?? 0][col[0]];
									let colour = "inherit";
									if (col[5] !== false) {
										colour = getColour(
											val.sortValue,
											averageTeam[col[0]].sortValue,
											col[5],
										);
									}
									return (
										<span style={{ color: colour }}>
											{val.prettyValue}
										</span>
									);
								}}
							</Cell>
						</Column>
					))}
				</Table>
			</div>
		</CustomProvider>
	);
}
