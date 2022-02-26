import * as React from "react";

import { fetchState } from "../util";
import TeamList from "../components/TeamList";
import { TeamInfo } from "../lib";

/**
 * Root page.
 *
 * @returns The page as a react component.
 */
function Overview(): React.ReactElement {
	const data = fetchState<TeamInfo[]>("/api/analysis")[0];
	if (data === undefined) {
		return <div>Loading...</div>;
	} else if (data.error) {
		return <div>Error: {data.message}</div>;
	} else {
		return (
			<div className="content">
				<TeamList data={data.result} />
			</div>
		);
	}
}

export default Overview;
