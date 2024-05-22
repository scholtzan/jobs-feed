import { error, success } from '.';
import { Settings } from '../types/settings';
import { constants } from '../constants';

/**
 * Functions to make calls against the settings API.
 */
export class SettingsApi {
	constructor() {}

	/**
	 * Make API call to update stored settings.
	 * @param settings settings data to store
	 * @returns request response
	 */
	public updateSettings = async (settings: Settings) => {
		return fetch(`/api/${constants.API_VERSION}/settings`, {
			method: 'PUT',
			body: JSON.stringify(settings)
		}).then((response) => {
			if (response.status == 200) {
				return response.json().then((json) => {
					let storedSettings: Settings = Object.assign(new Settings(), json);
					return success(storedSettings);
				});
			} else {
				return error(`Could not update settings: ${response}`);
			}
		});
	};

	/**
	 * Make API call to get stored settings data.
	 * @returns request response
	 */
	public getSettings = async () => {
		return fetch(`/api/${constants.API_VERSION}/settings`, {
			method: 'GET'
		}).then((response) => {
			if (response.status == 200) {
				return response.json().then((json) => {
					let storedSettings: Settings;
					if (json == null) {
						storedSettings = new Settings();
					} else {
						storedSettings = Object.assign(new Settings(), json);
					}
					return success(storedSettings);
				});
			} else {
				return error(`Could not get settings: ${response}`);
			}
		});
	};

	/**
	 * Make API call to get available OpenAI models.
	 * @returns request response
	 */
	public getModels = async () => {
		return fetch(`/api/${constants.API_VERSION}/settings/models`, {
			method: 'GET'
		}).then((response) => {
			if (response.status == 200) {
				return response.json().then((json) => {
					return success(json);
				});
			} else {
				return error(`Could not get models: ${response}`);
			}
		});
	};
}
