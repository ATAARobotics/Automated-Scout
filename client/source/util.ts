import * as React from "react";

/**
 * Convert an identifier string like `foo-bar_baz` or `fooBarBaz` into title case: `Foo Bar Baz`.
 *
 * @param str - The string to convert.
 * @returns The converted string.
 */
export function toTitleCase(str: string): string {
	// I really miss rust's return syntax or at least the semi-colon checking
	return str
		.replace(/([A-Z])/g, (s) => " " + s)
		.replace(/[-_.:\s]/, " ")
		.toLowerCase()
		.split(" ")
		.map((s) => s.replace(/(.)/, (c) => c.toUpperCase()))
		.join(" ");
}

export interface FetchError {
	error: true;
	message: string;
}

/**
 * Fetch an api endpoint as a React useState object.
 *
 * @param path - The api endpoint.
 * @returns The result as a useState object, and a function to refresh from the api.
 */
export function fetchState<T>(
	path: string,
): [{ error: false; result: T } | FetchError | undefined, () => void] {
	const [data, setData] = React.useState<
		{ error: false; result: T } | FetchError
	>();
	const [shouldUpdate, setShouldUpdate] = React.useState(false);
	React.useEffect(() => {
		fetch(path)
			.then((response) => {
				if (response.ok) {
					return response.json();
				}
				throw new Error("Response not ok.");
			})
			.then((rawResult) => {
				const result = rawResult as
					| { success: false }
					| { success: true; data: T };
				if (result.success) {
					return result.data;
				} else {
					throw new Error("Response not success.");
				}
			})
			.then((object) => {
				setData({ error: false, result: object });
			})
			.catch((e) => {
				setData({ error: true, message: e.message });
			});
	}, [shouldUpdate]);
	return [data, () => setShouldUpdate(!shouldUpdate)];
}
