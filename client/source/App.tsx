import * as React from "react";
import { Switch, Route } from "react-router-dom";

import RootPage from "./pages/RootPage";
import UnknownPage from "./pages/UnknownPage";

/**
 * Entry point component for the app.
 *
 * @returns The app as a react component.
 */
function App(): React.ReactElement {
	return <div className="app">
		<div className="topbar">
			<h2>Automation McAutoface</h2>
		</div>
		<Switch>
			<Route exact path="/" component={RootPage} />
			<Route path="/" component={UnknownPage} />
		</Switch>
	</div>;
}

export default App;
