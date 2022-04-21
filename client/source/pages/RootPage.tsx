import * as React from "react";

import { fetchState } from "../util";
import TeamList from "../components/TeamList";
import { TeamInfo } from "../lib";
import TitleIcon from "../components/TitleIcon";

/**
 * Root page.
 *
 * @returns The page as a react component.
 */
function Overview(): React.ReactElement {
	let oldPinnedTeams = localStorage.getItem("pinnedTeams");
	let initialPinnedTeams =
		oldPinnedTeams === null
			? { 4421: true, 0: true }
			: (JSON.parse(oldPinnedTeams) as { [teamNumber: number]: boolean });
	const [pinnedTeams, setPinnedTeams] = React.useState<{
		[teamNumber: number]: boolean;
	}>(initialPinnedTeams);
	React.useEffect(() => {
		localStorage.setItem("pinnedTeams", JSON.stringify(pinnedTeams));
	}, [pinnedTeams]);
	const data = fetchState<TeamInfo[]>("/api/analysis");
	const setPinnedTeam = (teamNumber: number, pinned: boolean) => {
		setPinnedTeams({ ...pinnedTeams, [teamNumber]: pinned });
	};
	if (data === undefined) {
		return <div>Loading...</div>;
	} else if (data.error) {
		return <div>Error: {data.message}</div>;
	} else {
		return (
			<div className="content rootPage">
				<TitleIcon title="All Teams - Automated Scout 2022" />
				<TeamList
					pinnedTeams={pinnedTeams}
					setPinnedTeam={setPinnedTeam}
					data={data.result.filter((team) => pinnedTeams[team.teamNumber])}
				/>
				<TeamList
					pinnedTeams={pinnedTeams}
					setPinnedTeam={setPinnedTeam}
					fillHeight
					data={data.result}
				/>
			</div>
		);
	}
}

export default Overview;
