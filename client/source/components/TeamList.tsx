import * as React from "react";
import { Table, Column, HeaderCell, Cell, RowDataType } from "rsuite-table";

import { TeamInfo } from "../lib";
import { formatPercent, formatRatio, formatProbList } from "../util";
import { Link } from "react-router-dom";


/**
 * To Do List:
 * Know how to do:
 * Figure out what key information we want to display on automated scout
 * Automated Scout visual update?
 * Fix park to be part of the charge station selection for teleop :( 
 * Rearrange buttons? 
 * Code readability :( 
 * 
 * Don't know how to do
 * Add match notes to team page
 * A way to check individual match data?
 * 
 * Do later
 * Create static versions that always run
 * Get info if someone on team balanced and whether to exclude auto balance information because of that
 * Get blue alliance data
 */

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
		"Tele Sc.",
		(match: TeamInfo) => match.averageTeleopScore,
		(match: TeamInfo) => match.averageTeleopScore.toFixed(1),
		0.75,
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
		(match: TeamInfo) => match.overallSpeed.toFixed(1) + " / 5.0",
		1,
		false,
		5.0,
	],
	[
		"Stability",
		(match: TeamInfo) => match.overallStability,
		(match: TeamInfo) => match.overallStability.toFixed(1) + " / 5.0",
		1,
		false,
		5.0,
	],
	[
		"Defence",
		(match: TeamInfo) => match.overallDefence,
		(match: TeamInfo) => match.overallDefence.toFixed(1) + " / 5.0",
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
		"Auto Charge",
		(match: TeamInfo) =>
			match.chargeStationAutoOff * 3 +
			match.chargeStationAutoOn * 2 +
			match.chargeStationAutoCharged,
		(match: TeamInfo) =>
			formatProbList(
				["Off", "On", "Charged"],
				[match.chargeStationAutoOff, match.chargeStationAutoOn, match.chargeStationAutoCharged]
			),
		1,
		false,
		false,
	],
	[
		"Teleop Charge",
		(match: TeamInfo) =>
			match.chargeStationTeleopOff * 3 +
			match.chargeStationTeleopOn * 2 +
			match.chargeStationTeleopCharged,
		(match: TeamInfo) =>
			formatProbList(
				["Off", "On", "Charged"],
				[match.chargeStationTeleopOff, match.chargeStationTeleopOn, match.chargeStationTeleopCharged]
			),
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
			match.averageTeleopScore,
		(match: TeamInfo) =>
			(
				match.averageAutoScore +
				match.averageTeleopScore
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
