import { error, success } from '.';
import { Suggestion } from '../types/suggestions';
import { constants } from '../constants';

export class SuggestionsApi {
	constructor() {}

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

	public getSourceSuggestions = async (sourceId: number) => {
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

	public refreshSourceSuggestions = async (sourceId: number) => {
		return fetch(`/api/${constants.API_VERSION}/sources/${sourceId}/suggestions`, {
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
