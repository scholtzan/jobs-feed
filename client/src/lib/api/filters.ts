import { error, success } from '.';
import { Filter } from '../types/filters';
import { constants } from '../constants';

/**
 * Functions to make calls against the filters API.
 */
export class FiltersApi {
	constructor() {}

	/**
	 * Make API request to get all filters.
	 * @returns request response
	 */
	public getFilters = async () => {
		return fetch(`/api/${constants.API_VERSION}/filters`, {
			method: 'GET'
		}).then((response) => {
			if (response.status == 200) {
				return response.json().then((json) => {
					return success(json as Filter[]);
				});
			} else {
				return error(`Could not get filters: ${response}`);
			}
		});
	};

	/**
	 * Make API request to update all filters
	 * @param filters updated filters data
	 * @returns request response
	 */
	public updateFilters = async (filters: Filter[]) => {
		return fetch(`/api/${constants.API_VERSION}/filters`, {
			method: 'PUT',
			body: JSON.stringify(filters)
		}).then((response) => {
			if (response.status == 200) {
				return response.json().then((json) => {
					let storedFilters: Filter[] = json as Filter[];
					return success(storedFilters);
				});
			} else {
				return error(`Could not update filters: ${response}`);
			}
		});
	};
}
