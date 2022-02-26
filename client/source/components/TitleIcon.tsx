import * as React from "react";

interface TitleIconProps {
	title: string;
	icon: string;
}

/**
 * Component to set the tab title and icon.
 *
 * @param props The title and icon name.
 * @returns The component.
 */
function TitleIcon(props: TitleIconProps): React.ReactElement {
	React.useEffect(() => {
		const titleElement = document.getElementById("window-title");
		if (titleElement instanceof HTMLTitleElement) {
			titleElement.innerHTML = props.title;
		}
		const iconElement = document.getElementById("window-icon");
		if (iconElement instanceof HTMLLinkElement) {
			iconElement.href = "/" + props.icon;
		}
	}, [props.title, props.icon]);
	return <></>;
}

export default TitleIcon;
