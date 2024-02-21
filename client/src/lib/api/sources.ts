import { error, success } from '.';
import { Source } from '../types/sources';
import { constants } from '../constants';

export class SourcesApi {
	constructor() {}

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

	public deleteSource = async (sourceId: number) => {
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

	public resetSourceCache = async (sourceId: number) => {
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
