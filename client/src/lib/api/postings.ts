import { error, success } from '.';
import { constants } from '../constants';
import { Posting } from '../types/postings';

/**
 * Functions to make calls against the postings API.
 */
export class PostingsApi {
	constructor() {}

	/**
	 * Make API call to get data for a specific posting.
	 * @param id ID of posting to get data for
	 * @returns request response
	 */
	public getPostingById = async (id: number | null) => {
		return fetch(`/api/${constants.API_VERSION}/postings/${id}`, {
			method: 'GET'
		}).then((response) => {
			if (response.status == 200) {
				return response.json().then((json) => {
					return success(json as Posting);
				});
			} else {
				return error('Could not get posting');
			}
		});
	};

	/**
	 * Make API call to refresh postings.
	 * @param source_id [optional] only refresh postings for this soruce
	 * @returns request response
	 */
	public refreshPostings = async (source_id: number | null = null) => {
		let url = `/api/${constants.API_VERSION}/postings/refresh`;
		if (source_id != null) {
			url += `?source_id=${source_id}`;
		}

		return fetch(url, {
			method: 'GET'
		}).then((response) => {
			if (response.status == 200) {
				return response.json().then((json) => {
					return success(json.map((p) => Object.assign(new Posting(), p)));
				});
			} else {
				return error('Could not refresh postings');
			}
		});
	};

	/**
	 * Make API call to get all unseen postings.
	 * @returns request response
	 */
	public getUnreadPostings = async () => {
		return fetch(`/api/${constants.API_VERSION}/postings/unread`, {
			method: 'GET'
		}).then((response) => {
			if (response.status == 200) {
				return response.json().then((json) => {
					return success(json as Posting[]);
				});
			} else {
				return error(`Could not get postings: ${response}`);
			}
		});
	};

	/**
	 * Make API call to get all bookmarked postings.
	 * @returns request response
	 */
	public getBookmarkedPostings = async () => {
		return fetch(`/api/${constants.API_VERSION}/postings/bookmarked`, {
			method: 'GET'
		}).then((response) => {
			if (response.status == 200) {
				return response.json().then((json) => {
					return success(json as Posting[]);
				});
			} else {
				return error('Could not get bookmarked postings');
			}
		});
	};

	/**
	 * Make API call to mark a set of postings as seen.
	 * @returns request response
	 */
	public markPostingsAsRead = async (ids: number[]) => {
		return fetch(`/api/${constants.API_VERSION}/postings/mark_read`, {
			method: 'PUT',
			body: JSON.stringify(ids)
		}).then((response) => {
			if (response.status == 200) {
				return success({});
			} else {
				return error('Cannot mark postings as read');
			}
		});
	};

	/**
	 * Make API call to udpate data of a specific posting.
	 * @param posting updated posting data
	 * @returns request response
	 */
	public updatePosting = async (posting: Posting) => {
		return fetch(`/api/${constants.API_VERSION}/postings/${posting.id}`, {
			method: 'PUT',
			body: JSON.stringify(posting)
		}).then((response) => {
			if (response.status == 200) {
				return response.json().then((json) => {
					return success(json as Posting);
				});
			} else {
				return error('Could not bookmark posting');
			}
		});
	};

	/**
	 * Make API request to get all read postings of a specific source.
	 * @param sourceId [optional] ID of source to get postings for
	 * @returns request response
	 */
	public getReadPostingsOfSource = async (sourceId: number | string | null) => {
		let sourceFilter = '';
		if (sourceId != 'all' && sourceId != 'today') {
			sourceFilter = `source_id=${sourceId}&`;
		}

		return fetch(`/api/${constants.API_VERSION}/postings?${sourceFilter}read=true`, {
			method: 'GET'
		}).then((response) => {
			if (response.status == 200) {
				return response.json().then((json) => {
					return success(json as Posting[]);
				});
			} else {
				return error(`Could not get read postings ${response.statusText}`);
			}
		});
	};
}
