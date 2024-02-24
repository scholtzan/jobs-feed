import { error, success } from '.';
import { Usage } from '../types/usage';
import { constants } from '../constants';

export class UsageApi {
	constructor() {}

	public getUsageCost = async (days: number | null) => {
		let daysFilter = '';
		if (days != null) {
			daysFilter = `?days=${days}`;
		}

		return fetch(`/api/${constants.API_VERSION}/usage/cost/${daysFilter}`, {
			method: 'GET'
		}).then((response) => {
			if (response.status == 200) {
				return response.json().then((json) => {
					return success(json as Usage[]);
				});
			} else {
				return error(`Could not get usage: ${response}`);
			}
		});
	};
}
