import { error, success } from '.';
import { RequestResponse } from '../types/index';
import { Filter } from '../types/filters';

export class FiltersApi {
	constructor() {}

	public getFilters = async () => {
		return fetch('/filters', {
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

	public updateFilters = async (filters: Filter[]) => {
		return fetch('/filters', {
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
