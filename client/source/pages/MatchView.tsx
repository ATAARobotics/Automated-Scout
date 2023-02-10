import * as React from "react";

import TitleIcon from "../components/TitleIcon";
import { fetchState } from "../util";
import { FullMatchInfo, MatchInfo, TeamInfo } from "../lib";
import { useParams } from "react-router";
import { order } from "../components/TeamList";
// This is for the team specific page, contains pit scouting data mostly
function KeyValueBox(props: {
	label: string;
	value: number | boolean | string | undefined;
}): React.ReactElement {
	return (
		<div className="key-value-box">
			<span className="key">{props.label}: </span>
			<span className="value">
				{props.value === true
					? "Yes"
					: props.value === false
					? "No"
					: typeof props.value === "number"
					? props.value.toString()
					: props.value === undefined
					? "Unknown"
					: props.value}
			</span>
		</div>
	);
}

function MatchView(): React.ReactElement {
	const { match: matchString } = useParams();
	const matchNumber = parseInt(matchString ?? "0");
	const teamInfos = fetchState<TeamInfo[]>("/api/analysis")[0];
	const fullMatchInfo = fetchState<FullMatchInfo>(
		`/api/match_info?match=${matchNumber}`
	)[0];
	return (
		<div className="matchView">
			<TitleIcon
				title={`Match ${matchNumber} · Automated Scout`}
				icon="icon-progress.png"
			/>
			<h1>Match {matchNumber}</h1>
			{(() => {
				if (teamInfos === undefined || fullMatchInfo === undefined) {
					return <p>Loading...</p>;
				} else if (teamInfos.error || fullMatchInfo.error) {
					return (
						<>
							<h1>Error</h1>
							<p>{teamInfos.error ? teamInfos.message : ""}</p>
							<p>{fullMatchInfo.error ? fullMatchInfo.message : ""}</p>
						</>
					);
				} else {
					
					// Returns this info onto the site. Anything in here will be put on the site including comments!
					return (
						<>
						{fullMatchInfo.result.teams.blueTeams.concat(fullMatchInfo.result.teams.redTeams).map(team => {
							<div key={team[0]}>
								{(()=>{
									const teamInfo = teamInfos.result.find(teamInfo => teamInfo.teamNumber = team[0]);
									if (teamInfo !== undefined) {
										return <div> {order.map(([label, _, getValue, _1, _2, _3]) => {
											return KeyValueBox(label, getValue(teamInfo));
										})} </div>;
									} else {
										return <></>;
									}
								})()}
								</div>
						})}
						</>
					);
				}
				return <></>;
			})()}
		</div>
	);
}

export default MatchView;
