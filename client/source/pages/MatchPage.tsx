import * as React from "react";
import { Link, useNavigate, useParams } from "react-router-dom";

import TitleIcon from "../components/TitleIcon";
import { MatchInfo, TeamInfo } from "../lib";
import { fetchState } from "../util";

function TeamMatchInfo(
	data: TeamInfo[],
	teamNumber: number
): React.ReactElement {
	const teamData = data.find((t) => t.teamNumber === teamNumber);
	if (!teamData) {
		return <p key={teamNumber}>{teamNumber} - Not found!</p>;
	}
	return (
		<tr key={teamNumber}>
			<td key="num">{teamData.teamNumber}</td>
			<td key="name">{teamData.teamName}</td>
			<td key="scouted" className="right">
				{teamData.matchesScouted} / {teamData.matchesPlayed}
			</td>
			<td key="auto" className="right">
				{teamData.averageAutoScore.toFixed(1)}
			</td>
			<td key="tele" className="right">
				{teamData.averageTeleopScore.toFixed(1)}
			</td>
			<td key="climb" className="right">
				{teamData.averageClimbScore.toFixed(1)}
			</td>
			<td key="total" className="right">
				{(
					teamData.averageAutoScore +
					teamData.averageTeleopScore +
					teamData.averageClimbScore
				).toFixed(1)}
			</td>
			<td key="opr" className="right">
				{teamData.opr.toFixed(1)}
			</td>
			<td key="dpr" className="right">
				{teamData.dpr.toFixed(1)}
			</td>
		</tr>
	);
}

function AllianceMatchTotal(
	data: TeamInfo[],
	teams: number[],
	alliance: "red" | "blue"
): React.ReactElement {
	const teamDatas = teams
		.map((teamNumber) => data.find((t) => t.teamNumber === teamNumber))
		.filter((d): d is TeamInfo => !!d);
	return (
		<tr key={`totals-${alliance}`}>
			<td key="name" colSpan={2}>
				Totals
			</td>
			<td key="scouted"></td>
			<td key="auto" className="right">
				{teamDatas
					.reduce((acc, t) => acc + t.averageAutoScore, 0)
					.toFixed(1)}
			</td>
			<td key="tele" className="right">
				{teamDatas
					.reduce((acc, t) => acc + t.averageTeleopScore, 0)
					.toFixed(1)}
			</td>
			<td key="climb" className="right">
				{teamDatas
					.reduce((acc, t) => acc + t.averageClimbScore, 0)
					.toFixed(1)}
			</td>
			<td key="total" className="right">
				{teamDatas
					.reduce(
						(acc, t) =>
							acc +
							t.averageAutoScore +
							t.averageTeleopScore +
							t.averageClimbScore,
						0
					)
					.toFixed(1)}
			</td>
			<td key="opr" className="right">
				{teamDatas.reduce((acc, t) => acc + t.opr, 0).toFixed(1)}
			</td>
			<td key="dpr" className="right">
				{teamDatas.reduce((acc, t) => acc + t.dpr, 0).toFixed(1)}
			</td>
		</tr>
	);
}

function Matchup(props: {
	data: TeamInfo[];
	number: number;
}): React.ReactElement {
	const matchData = fetchState<MatchInfo>(`/api/match/Quals/${props.number}`);

	if (matchData === undefined) {
		return <div>Loading...</div>;
	} else if (matchData.error) {
		return <div>Error: {matchData.message}</div>;
	} else {
		return (
			<>
				<TitleIcon
					title={`Automated Scout 2022 - Quals #${props.number}`}
				/>
				<table>
					<tbody>
						<tr key="red-header">
							<td key="name" colSpan={2}>
								Red Alliance
							</td>
							<td key="scouted">Scouted</td>
							<td key="auto">Auto</td>
							<td key="tele">Tele</td>
							<td key="climb">Climb</td>
							<td key="total">Score</td>
							<td key="opr">OPR</td>
							<td key="dpr">DPR</td>
						</tr>
						{matchData.result.alliances.red.teams.map((team) =>
							TeamMatchInfo(props.data, team)
						)}
						{AllianceMatchTotal(
							props.data,
							matchData.result.alliances.red.teams,
							"red"
						)}
						<tr key="blue-header">
							<td colSpan={2}>Blue Alliance</td>
							<td key="scouted">Scouted</td>
							<td key="auto">Auto</td>
							<td key="tele">Tele</td>
							<td key="climb">Climb</td>
							<td key="total">Score</td>
							<td key="opr">OPR</td>
							<td key="dpr">DPR</td>
						</tr>
						{matchData.result.alliances.blue.teams.map((team) =>
							TeamMatchInfo(props.data, team)
						)}
						{AllianceMatchTotal(
							props.data,
							matchData.result.alliances.blue.teams,
							"blue"
						)}
					</tbody>
				</table>
			</>
		);
	}
}

/**
 * Match page.
 *
 * @returns The page as a react component.
 */
function MatchPage(): React.ReactElement {
	const data = fetchState<TeamInfo[]>("/api/analysis");
	let [selectedNumber, setSelectedNumber] = React.useState(1);

	if (data === undefined) {
		return <div>Loading...</div>;
	} else if (data.error) {
		return <div>Error: {data.message}</div>;
	} else {
		return (
			<div className="matchPage">
				<label htmlFor="matchNumber">Match Number:</label>
				<input
					type="number"
					min={1}
					max={200}
					id="matchNumber"
					name="matchNumber"
					value={selectedNumber}
					onChange={(event) =>
						setSelectedNumber(event.target.value as any)
					}
				></input>

				<Matchup data={data.result} number={selectedNumber}></Matchup>
			</div>
		);
	}
}

export default MatchPage;
