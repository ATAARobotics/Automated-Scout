import * as React from "react";
import { Link, useNavigate, useParams } from "react-router-dom";

import TitleIcon from "../components/TitleIcon";

/**
 * Root page.
 *
 * @returns The page as a react component.
 */
function SelectMatch(): React.ReactElement {
	let { type, number } = useParams();
	const navigate = useNavigate();
	let [selectedNumber, setSelectedNumber] = React.useState(1);

	if (!type) {
		return (
			<div>
				<TitleIcon title={`Select Match Type - Automated Scout 2022`} />
				<h1>Which match type?</h1>
				<ul>
					<li>
						<Link to={`/match/Practice`}>Practice</Link>
					</li>
					<li>
						<Link to={`/match/Quals`}>Quals</Link>
					</li>
				</ul>
			</div>
		);
	} else {
		return (
			<div>
				<TitleIcon title={`Select Match Number - Automated Scout 2022`} />
				<h1>Which match number?</h1>
				<label htmlFor="matchNumber">Match Number:</label>
				<input
					type="number"
					id="matchNumber"
					name="matchNumber"
					value={selectedNumber}
					onChange={(event) =>
						setSelectedNumber(event.target.value as any)
					}
				></input>
				<button
					onClick={() => navigate(`/match/${type}/${selectedNumber}`)}
				>
					Go!
				</button>
			</div>
		);
	}
}

export default SelectMatch;
