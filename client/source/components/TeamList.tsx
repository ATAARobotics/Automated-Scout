import * as React from "react";
import { Table, Column, HeaderCell, Cell, RowDataType } from "rsuite-table";

import { TeamInfo } from "../lib";
import { formatPercent, formatRatio, formatProbList } from "../util";
import { Link } from "react-router-dom";

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
	spread: number
): string {
	const relativeScore = Math.min(
		Math.max((score - averageScore) / spread, -1),
		1
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

const order: [
	string,
	(match: TeamInfo) => number,
	(match: TeamInfo) => string,
	number,
	"left" | "right" | false,
	number | false
][] = [
	[
		"Team",
		(match: TeamInfo) => match.teamNumber,
		(match: TeamInfo) =>
			(match.teamNumber === 0 ? "Avg." : match.teamNumber.toFixed(0)) +
			(match.teamName !== null ? ` (${match.teamName})` : ""),
		2,
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
		"Tele Acc",
		(match: TeamInfo) =>
			match.averageAutoHighGoalAccuracy + match.averageAutoLowGoalAccuracy,
		(match: TeamInfo) =>
			formatPercent(match.averageAutoHighGoalAccuracy) +
			"H, " +
			formatPercent(match.averageAutoLowGoalAccuracy) +
			"L",
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
		(match: TeamInfo) =>
			match.averageTeleopHighGoalAccuracy +
			match.averageTeleopLowGoalAccuracy,
		(match: TeamInfo) =>
			formatPercent(match.averageTeleopHighGoalAccuracy) +
			"H, " +
			formatPercent(match.averageTeleopLowGoalAccuracy) +
			"L",
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
			formatPercent(1.0 - match.climbFailRate) +
			" (" +
			match.climbAttemptCounts
				.map((n) => ((n[1] / n[0]) * 100.0).toFixed(0) + "%")
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
		"Defence Sc.",
		(match: TeamInfo) => match.averageDefenceScore,
		(match: TeamInfo) => match.averageDefenceScore.toFixed(1),
		0.75,
		false,
		1.0,
	],
	[
		"Luck Sc.",
		(match: TeamInfo) => match.averageLuckScore,
		(match: TeamInfo) => match.averageLuckScore.toFixed(1),
		0.75,
		false,
		-1.0,
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
		-5.0,
	],
	[
		"Start Loc.",
		(match: TeamInfo) =>
			match.startLeftRate * 3 +
			match.startMiddleRate * 2 +
			match.startRightRate,
		(match: TeamInfo) =>
			formatProbList(
				["Left", "Mid", "Right"],
				[match.startLeftRate, match.startMiddleRate, match.startRightRate]
			),
		1,
		false,
		false,
	],
	[
		"Shoot Loc.",
		(match: TeamInfo) => match.shootHubRate - match.shootFarRate,
		(match: TeamInfo) =>
			formatProbList(
				["Hub", "Far"],
				[match.shootHubRate, match.shootFarRate]
			),
		1,
		false,
		false,
	],
	[
		"Friendly",
		(match: TeamInfo) => (match.friendly ? 1.0 : 0.0),
		(match: TeamInfo) => (match.friendly ? "Yes" : "No"),
		1,
		false,
		0.01,
	],
	[
		"Pit Peopl.",
		(match: TeamInfo) => match.averagePeopleInPit,
		(match: TeamInfo) => match.averagePeopleInPit.toFixed(0),
		1,
		false,
		false,
	],
	[
		"Pit Busy",
		(match: TeamInfo) => match.averagePitBusiness,
		(match: TeamInfo) => match.averagePitBusiness.toFixed(2),
		1,
		false,
		false,
	],
	[
		"Pit Chaos",
		(match: TeamInfo) => match.averagePitChaos,
		(match: TeamInfo) => match.averagePitChaos.toFixed(2),
		1,
		false,
		false,
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

type TeamDisplay = { [p: string]: { sortValue: number; prettyValue: string } };

interface TeamListProps {
	pinnedTeams?: { [teamNumber: number]: boolean };
	setPinnedTeam?: (teamNumber: number, pin: boolean) => void;
	fillHeight?: boolean;
	data: TeamInfo[];
}

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
			order.reduce<TeamDisplay>(
				(o, col) => ({
					...o,
					[col[0]]: { sortValue: col[1](row), prettyValue: col[2](row) },
				}),
				{}
			)
		)
		.sort((a, b) => {
			const diff = a[sortColumn].sortValue - b[sortColumn].sortValue;
			if (sortType === "desc") {
				return -diff;
			}
			return diff;
		});
	const averageTeam = data.reduce<undefined | TeamDisplay>(
		(p, info) => p ?? (info["Team"].sortValue === 0 ? info : undefined),
		undefined
	);
	return (
		<Table
			wordWrap
			headerHeight={80}
			fillHeight={props.fillHeight ?? false}
			data={data}
			sortColumn={sortColumn}
			sortType={sortType}
			onSortColumn={(newSortColumn, newSortType) => {
				setSortColumn(newSortColumn);
				setSortType(newSortType ?? "asc");
			}}
		>
			{(() => {
				if (props.pinnedTeams !== undefined) {
					return (
						<Column fixed="left" flexGrow={0.5} key={"Pinned"}>
							<HeaderCell>Pinned</HeaderCell>
							<Cell>
								{(
									rowData: RowDataType,
									rowIndex: number | undefined
								) => {
									const canChange =
										rowIndex !== undefined &&
										props.setPinnedTeam !== undefined;
									const teamNumber =
										data[rowIndex ?? 0]["Team"].sortValue;
									const pinned =
										rowIndex != undefined &&
										((props.pinnedTeams ?? {})[teamNumber] ?? false);
									return (
										<input
											type="checkbox"
											checked={pinned}
											onChange={(ev) => {
												if (props.setPinnedTeam !== undefined) {
													props.setPinnedTeam(
														teamNumber,
														ev.target.checked
													);
												}
											}}
											disabled={!canChange}
										/>
									);
								}}
							</Cell>
						</Column>
					);
				}
			})()}
			{order.map((col, colIdx) => (
				<Column
					fixed={col[4]}
					flexGrow={col[3]}
					minWidth={100 * col[3]}
					key={col[0]}
					sortable
				>
					<HeaderCell>{col[0]}</HeaderCell>
					<Cell dataKey={col[0]}>
						{(rowData: RowDataType, rowIndex: number | undefined) => {
							const val = data[rowIndex ?? 0][col[0]];
							let colour = "inherit";
							if (col[5] !== false) {
								if (averageTeam) {
									colour = getColour(
										val.sortValue,
										averageTeam[col[0]].sortValue,
										col[5]
									);
								}
							}
							if (colIdx === 0) {
								return (
									<Link to={`/team/${val.sortValue}`}>
										{val.prettyValue}
									</Link>
								);
							} else {
								return (
									<span style={{ color: colour }}>
										{val.prettyValue}
									</span>
								);
							}
						}}
					</Cell>
				</Column>
			))}
		</Table>
	);
}
