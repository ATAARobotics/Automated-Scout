import * as React from "react";

import TitleIcon from "../components/TitleIcon";
import { fetchState } from "../util";
import { FullTeamInfo, MatchInfo, TeamInfo } from "../lib";
import { useParams } from "react-router";

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

/**
 * Page not found page.
 *
 * @returns The page as a React component.
 */
function TeamView(): React.ReactElement {
	const { team: teamString } = useParams();
	const teamNumber = parseInt(teamString ?? "0");
	const teamInfos = fetchState<TeamInfo[]>("/api/analysis")[0];
	const fullTeamInfo = fetchState<FullTeamInfo>(
		`/api/team_info?team=${teamNumber}`
	)[0];
	return (
		<div className="teamView">
			<TitleIcon
				title={`Team ${teamNumber} · Automated Scout`}
				icon="icon-progress.png"
			/>
			<h1>Team {teamNumber}</h1>
			{(() => {
				if (teamInfos === undefined || fullTeamInfo === undefined) {
					return <p>Loading...</p>;
				} else if (teamInfos.error || fullTeamInfo.error) {
					return (
						<>
							<h1>Error</h1>
							<p>{teamInfos.error ? teamInfos.message : ""}</p>
							<p>{fullTeamInfo.error ? fullTeamInfo.message : ""}</p>
						</>
					);
				} else {
					const ourData = fullTeamInfo.result.matches.flatMap((match) =>
						match.blueTeams
							.concat(match.redTeams)
							.filter(
								(team) => team[1] !== null && team[0] === teamNumber
							)
							.map((team) => team[1] as MatchInfo)
					);
					const teamInfo = teamInfos.result.find(
						(team) => team.teamNumber === teamNumber
					) as TeamInfo;
					const shooterCapabilities = [
						"None",
						"Low",
						"High",
						"Both",
						"Unknown",
					];
					const shooterRanges = ["N/A", "Close", "Far", "Any", "Unknown"];
					const climbHeights = [
						"None",
						"Low",
						"Mid",
						"High",
						"Traversal",
						"Unknown",
					];
					return (
						<>
							<h1>
								{teamInfo.teamName} ({teamInfo.teamRookieYear})
							</h1>
							<h2>Images</h2>
							<div className="images">
								{fullTeamInfo.result.images.map((image, i) => (
									<img
										key={i}
										src={image.replace(
											"{AUTOSCOUT_URL}",
											window.location.origin
										)}
										alt={`Team ${teamNumber} image ${i}.`}
									/>
								))}
							</div>
							<h2>Comments</h2>
							<div>
								{fullTeamInfo.result.pitVisits.map((visit, i) => {
									if (
										visit.pit.comments.length > 0 ||
										visit.robot.comments
									) {
										return (
											<p key={`pit-${i}`}>
												<b>Pit Visit #{visit.scoutingTime}: </b>
												<span>{visit.pit.comments} </span>
												<span>{visit.robot.comments}</span>
											</p>
										);
									} else {
										return <span key={`pit-${i}`} />;
									}
								})}

								{ourData.map((matchInfo, i) => {
									if (matchInfo.notes.length > 0) {
										return (
											<p key={`match-${i}`}>
												<b>Match #{matchInfo.match}: </b>
												<span>{matchInfo.notes}</span>
											</p>
										);
									} else {
										return <span key={`match-${i}`} />;
									}
								})}
							</div>
							<h2>Pit Visits</h2>
							<div>
								{fullTeamInfo.result.pitVisits.map((visit, i) => {
									return (
										<div key={`pit-${i}`}>
											<h3>Pit Visit #{visit.scoutingTime}: </h3>
											<div className="propertyList">
												<KeyValueBox
													label="Business"
													value={visit.pit.busy}
												/>
												<KeyValueBox
													label="Chaos Level"
													value={visit.pit.chaos}
												/>
												<KeyValueBox
													label="People in Pit"
													value={visit.pit.pitPeople}
												/>
												<KeyValueBox
													label="Friendly"
													value={visit.pit.friendly}
												/>
												<KeyValueBox
													label="Cube Capacity"
													value={visit.robot.cubeCapacity}
												/>
												<KeyValueBox
													label="Autonomous Balls"
													value={visit.robot.autoBallCount}
												/>
												<KeyValueBox
													label="Shooter Goal"
													value={
														shooterCapabilities[
															visit.robot.shooterCapability ?? 4
														]
													}
												/>
												<KeyValueBox
													label="Shooter Range"
													value={
														shooterRanges[
															visit.robot.shooterRange ?? 4
														]
													}
												/>
												<KeyValueBox
													label="Climb Max Height"
													value={
														climbHeights[
															visit.robot.climbHeight ?? 5
														]
													}
												/>
												<KeyValueBox
													label="Climb Time"
													value={visit.robot.climbTime}
												/>
												<KeyValueBox
													label="Climb using Everybot"
													value={visit.robot.climbEverybot}
												/>
											</div>
										</div>
									);
								})}
							</div>
						</>
					);
				}
				return <></>;
			})()}
		</div>
	);
}

export default TeamView;
