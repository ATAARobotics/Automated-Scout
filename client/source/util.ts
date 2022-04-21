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

/**
 * Format a number between 0 and 1 as a percentage with one decimal place.
 *
 * @param num The number to format.
 * @returns The formatted percentage.
 */
export function formatPercent(num: number): string {
	return (num * 100.0).toFixed(1) + "%";
}

/**
 * Find the greatest common divisor of two numbers.
 *
 * @param a The first number.
 * @param b The second number.
 * @returns The greatest common divisor.
 */
function gcd(a: number, b: number): number {
	if (b === 0) {
		return a;
	}
	return gcd(b, a % b);
}

/**
 * Format two numbers as a ratio.
 * This simplifies the ratio by dividing by the greatest common factor.
 *
 * @param a The first (right) number.
 * @param b The second (left) number.
 * @returns The formatted ratio.
 */
export function formatRatio(a: number, b: number): string {
	a = Math.floor(a);
	b = Math.floor(b);
	let gcf = gcd(a, b);
	if (a === 0 || b === 0) {
		gcf = 1;
	}
	return `${a / gcf}:${b / gcf}`;
}

/**
 * Format a list of probabilities as a string with the name of the most likely.
 */
export function formatProbList(names: string[], probs: number[]): string {
	let highest = 0;
	for (let i = 0; i < probs.length; i++) {
		if (probs[i] > probs[highest]) {
			highest = i;
		}
	}
	return `${formatPercent(probs[highest])} ${names[highest]}`;
}

export interface FetchError {
	error: true;
	message: string;
}

const cachedResults: {
	[key: string]: { error: false; result: unknown } | FetchError;
} = {};

/**
 * Fetch an api endpoint as a React useState object.
 *
 * @param path - The api endpoint.
 * @returns The result as a useState object, and a function to refresh from the api.
 */
export function fetchState<T>(
	path: string
): { error: false; result: T } | FetchError | undefined {
	const [data, setData] = React.useState<
		{ error: false; result: T } | FetchError
	>();
	React.useEffect(() => {
		if (cachedResults[path]) {
			setData(
				cachedResults[path] as { error: false; result: T } | FetchError
			);
		} else {
			setData(undefined);
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
		}
	}, [path]);
	return data;
}
