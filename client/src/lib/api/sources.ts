import { error, success } from '.';
import { Source } from '../types/sources';
import { constants } from '../constants';

/**
 * Functions to make calls against the sources API.
 */
export class SourcesApi {
	constructor() {}

	/**
	 * Make API call to get all sources.
	 * @returns request response
	 */
	public getSources = async () => {
		return fetch(`/api/${constants.API_VERSION}/sources`, {
			method: 'GET'
		}).then((response) => {
			if (response.status == 200) {
				return response.json().then((json) => {
					return success(json as Source[]);
				});
			} else {
				return error(`Could not get sources: ${response.statusText}`);
			}
		});
	};

	/**
	 * Make API call to create a new source.
	 * @param source data of new source
	 * @returns request response
	 */
	public createSource = async (source: Source) => {
		return fetch(`/api/${constants.API_VERSION}/sources`, {
			method: 'POST',
			body: JSON.stringify(source)
		}).then((response) => {
			if (response.status == 200) {
				return response.json().then((json) => {
					let storedSource: Source = Object.assign(new Source(), json);
					return success(storedSource);
				});
			} else {
				return error(`Cannot add source. ${response.statusText}`);
			}
		});
	};

	/**
	 * Make API call to update a specific source.
	 * @param source updated source data
	 * @returns request response
	 */
	public updateSource = async (source: Source) => {
		return fetch(`/api/${constants.API_VERSION}/sources/${source.id}`, {
			method: 'PUT',
			body: JSON.stringify(source)
		}).then((response) => {
			if (response.status == 200) {
				return response.json().then((json) => {
					let storedSource: Source = Object.assign(new Source(), json);
					return success(storedSource);
				});
			} else {
				return error(`Could not update source: ${response.statusText}`);
			}
		});
	};

	/**
	 * Make API call to remove a specific source.
	 * @param sourceId ID of source to delete
	 * @returns request response
	 */
	public deleteSource = async (sourceId: number | null) => {
		return fetch(`/api/${constants.API_VERSION}/sources/${sourceId}`, {
			method: 'DELETE'
		}).then((response) => {
			if (response.status == 200) {
				return success({});
			} else {
				return error(`Could not delete source: ${response.statusText}`);
			}
		});
	};

	/**
	 * Delete cached content for a specific source.
	 * @param sourceId ID of source to reset cache
	 * @returns request response
	 */
	public resetSourceCache = async (sourceId: number | null) => {
		return fetch(`/api/${constants.API_VERSION}/sources/${sourceId}/reset`, {
			method: 'PUT'
		}).then((response) => {
			if (response.status == 200) {
				return success({});
			} else {
				return error(`Could not reset source cache: ${response.statusText}`);
			}
		});
	};
}
