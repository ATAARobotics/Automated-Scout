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
		<>
			<h3 key={`${teamNumber}-h3`}>
				{teamData.teamNumber} - {teamData.teamName}
			</h3>
			<p key={`${teamNumber}-data`}>
				Scouted {teamData.matchesScouted} / {teamData.matchesPlayed}
				<br></br>Auto: {teamData.averageAutoScore}
				<br></br>Tele: {teamData.averageTeleopScore}
				<br></br>Climb: {teamData.averageClimbScore}
				<br></br>Sum:{" "}
				{teamData.averageAutoScore +
					teamData.averageTeleopScore +
					teamData.averageClimbScore}
				<br></br>OPR: {teamData.opr}
				<br></br>DPR: {teamData.dpr}
			</p>
		</>
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
		<>
			<h3 key={`totals-${alliance}-h3`}>Totals</h3>
			<p key={`totals-${alliance}-data`}>
				Auto: {teamDatas.reduce((acc, t) => acc + t.averageAutoScore, 0)}
				<br></br>Tele:{" "}
				{teamDatas.reduce((acc, t) => acc + t.averageTeleopScore, 0)}
				<br></br>Climb:{" "}
				{teamDatas.reduce((acc, t) => acc + t.averageClimbScore, 0)}
				<br></br>Sum:{" "}
				{teamDatas.reduce(
					(acc, t) =>
						acc +
						t.averageAutoScore +
						t.averageTeleopScore +
						t.averageClimbScore,
					0
				)}
			</p>
		</>
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
				<h2 key="red">Red Alliance</h2>
				{matchData.result.alliances.red.teams.map((team) =>
					TeamMatchInfo(props.data, team)
				)}
				{AllianceMatchTotal(
					props.data,
					matchData.result.alliances.red.teams,
					"red"
				)}
				<h2 key="blue">Blue Alliance</h2>
				{matchData.result.alliances.blue.teams.map((team) =>
					TeamMatchInfo(props.data, team)
				)}
				{AllianceMatchTotal(
					props.data,
					matchData.result.alliances.blue.teams,
					"blue"
				)}
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
