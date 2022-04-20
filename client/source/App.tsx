import * as React from "react";
import { Routes, Route } from "react-router-dom";

import RootPage from "./pages/RootPage";
import UnknownPage from "./pages/UnknownPage";

/**
 * Entry point component for the app.
 *
 * @returns The app as a react component.
 */
function App(): React.ReactElement {
	return (
		<div className="app">
			<div className="topbar">
				<h2>Automation McAutoface</h2>
			</div>
			<Routes>
				<Route path="/" element={ <RootPage/> } />
				<Route path="/*" element={ <UnknownPage/> } />
			</Routes>
		</div>
	);
}

export default App;
