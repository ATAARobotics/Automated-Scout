import * as React from "react";

import TitleIcon from "../components/TitleIcon";
import { fetchState } from "../util";
import { FullTeamInfo, MatchInfo, TeamInfo } from "../lib";
import { useParams } from "react-router";
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
/*To change:
Remove:
Doing Stuff VA
Friendly  VA
Pickup Type VA
Floor pickup range VA
Everybot VA

Change:
Drive train - make it a string VA

Add:
Certainty meter VA
Reversable Bumpers (bumper type) VA
Battery Quantity VA
Can they charge their batteries VA
Amount of motors (drive) VA
Amount of motors (other) VA
Scouting method VA
Auto settings VA
Able to read Tape/AprilTags VA
Prefer to play defence or offence VA
Prefer to play high middle or low VA

(V means added to Data)
(A means added to Analysis)
*/
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
					// ???
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
					// Making a construct of a few values, so that if the data is unknown it will show as unknown
					const preferredPlay = [
						"Defence",
						"Prefer Defence",
						"Prefer Offence",
						"Offence",
						"Unknown",
					];
					const preferredStack = [
						"None",
						"Hybrid",
						"Middle",
						"High",
						"Unknown",
					];
					const humanPickupRange = [
						"None",
						"Chute",
						"Slide Shelf",
						"Both",
						"Unknown",
					];
					const stackType = [
						"None",
						"Cone",
						"Cube",
						"Both",
						"Unknown",
					];
					const chargeBattery = [
						"No",
						"Yes",
						"Unknown",
					];
					const visionType = [
						"None",
						"Tape",
						"AprilTag",
						"Both",
						"Unknown",
					];
					const bumperType = [
						"None",
						"Swap",
						"Reversable",
						"Unknown",
					];
					// Returns this info onto the site. Anything in here will be put on the site including comments!
					// Match scouting comments currently don't display on the site for some reason
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
										visit.pit.scoutingMethod.length > 0 ||
										visit.robot.comments || visit.robot.driveType || visit.robot.autoSettings
									) {
										return (
											<p key={`pit-${i}`}>
												<b>Pit Visit #{visit.scoutingTime}: </b>
												<span>{visit.pit.scoutingMethod} </span>
												<span>{visit.robot.driveType}</span>
												<span>{visit.robot.autoSettings}</span>
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
													label="Chaos Level"
													value={visit.pit.chaos}
												/>
												<KeyValueBox
													label="People in Pit"
													value={visit.pit.pitPeople}
												/>
												<KeyValueBox
													label="Confidence Level"
													value={visit.pit.confidenceLevel}
												/>
												<KeyValueBox
													label="Preferred Play"
													value={
														preferredPlay[
															visit.robot.preferredPlay ?? 4
														]
													}
												/>
												<KeyValueBox
													label="Charge Battery"
													value={
														chargeBattery[
															visit.robot.chargeBattery ?? 2
														]
													}
												/>
												<KeyValueBox
													label="Vision Type"
													value={
														visionType[
															visit.robot.visionType ?? 4
														]
													}
												/>
												<KeyValueBox
													label="Stack Type"
													value={
														stackType[
															visit.robot.stackType ?? 4
														]
													}
												/>
												<KeyValueBox
													label="Bumper Type"
													value={
														bumperType[
															visit.robot.bumperType ?? 3
														]
													}
												/>
												<KeyValueBox
													label="Balance Time"
													value={visit.robot.balanceTime}
												/>
												<KeyValueBox
													label="Battery Amount"
													value={visit.robot.batteryAmount}
												/>
												<KeyValueBox
													label="Drive Motor Amount"
													value={visit.robot.driveMotorAmount}
												/>
												<KeyValueBox
													label="Other Motor Amount"
													value={visit.robot.otherMotorAmount}
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
