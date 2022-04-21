import * as React from "react";
import { Routes, Route } from "react-router-dom";

import { CustomProvider } from "rsuite";
import "rsuite/dist/rsuite.min.css";

import RootPage from "./pages/RootPage";
import TeamView from "./pages/TeamView";
import UnknownPage from "./pages/UnknownPage";

/**
 * Entry point component for the app.
 *
 * @returns The app as a react component.
 */
function App(): React.ReactElement {
	return (
		<div className="app">
			<CustomProvider theme="dark">
				<div className="topbar">
					<h2>Automation McAutoface</h2>
				</div>
				<Routes>
					<Route path="/" element={ <RootPage/> } />
					<Route path="/team/:team" element={ <TeamView/> } />
					<Route path="/*" element={ <UnknownPage/> } />
				</Routes>
			</CustomProvider>
		</div>
	);
}

export default App;
