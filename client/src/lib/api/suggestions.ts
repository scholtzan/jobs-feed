import { error, success } from '.';
import { Suggestion } from '../types/suggestions';
import { constants } from '../constants';

/**
 * Functions to make calls against the suggestions API.
 */
export class SuggestionsApi {
	constructor() {}

	/**
	 * Make API call to get all suggestions.
	 * @returns request response
	 */
	public getSuggestions = async () => {
		return fetch(`/api/${constants.API_VERSION}/suggestions`, {
			method: 'GET'
		}).then((response) => {
			if (response.status == 200) {
				return response.json().then((json) => {
					return success(json as Suggestion[]);
				});
			} else {
				return error(`Could not get suggestions: ${response.statusText}`);
			}
		});
	};

	/**
	 * Make API call to get suggestions for a specific source.
	 * @param sourceId ID of source to get suggestions for
	 * @returns request response
	 */
	public getSourceSuggestions = async (sourceId: number | string | null) => {
		return fetch(`/api/${constants.API_VERSION}/sources/${sourceId}/suggestions`, {
			method: 'GET'
		}).then((response) => {
			if (response.status == 200) {
				return response.json().then((json) => {
					return success(json as Suggestion[]);
				});
			} else {
				return error(`Could not get suggestions for source: ${response.statusText}`);
			}
		});
	};

	/**
	 * Make API call to refresh suggestions for a specific source.
	 * @param sourceId ID of source to refresh suggestions for
	 * @returns request response
	 */
	public refreshSourceSuggestions = async (sourceId: number | string | null) => {
		return fetch(`/api/${constants.API_VERSION}/sources/${sourceId}/suggestions/refresh`, {
			method: 'PUT'
		}).then((response) => {
			if (response.status == 200) {
				return response.json().then((json) => {
					return success(json as Suggestion[]);
				});
			} else {
				return error(`Could not refresh suggestions for source: ${response.statusText}`);
			}
		});
	};
}
