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
		return <p>{teamNumber} - Not found!</p>;
	}
	return (
		<p>
			{teamData.teamNumber} - {teamData.teamName} A:{" "}
			{teamData.averageAutoScore}, T: {teamData.averageTeleopScore}, C:{" "}
			{teamData.averageClimbScore}; OPR: {teamData.opr}, DPR: {teamData.dpr}
		</p>
	);
}

/**
 * Match page.
 *
 * @returns The page as a react component.
 */
function MatchPage(): React.ReactElement {
	let { type, number } = useParams();
	const matchData = fetchState<MatchInfo>(`/api/match/${type}/${number}`);
	const data = fetchState<TeamInfo[]>("/api/analysis");

	if (!type || !number) {
		return <div>Error: match type or number not specified.</div>;
	} else if (data === undefined || matchData === undefined) {
		return <div>Loading...</div>;
	} else if (matchData.error) {
		return <div>Error: {matchData.message}</div>;
	} else if (data.error) {
		return <div>Error: {data.message}</div>;
	} else {
		return (
			<div className="content rootPage">
				<TitleIcon title={`Automated Scout 2022 - ${type} #${number}`} />
				<h2>Red Alliance</h2>
				{matchData.result.alliances.red.teams.map((team) =>
					TeamMatchInfo(data.result, team)
				)}
				<h2>Blue Alliance</h2>
				{matchData.result.alliances.blue.teams.map((team) =>
					TeamMatchInfo(data.result, team)
				)}
			</div>
		);
	}
}

export default MatchPage;
