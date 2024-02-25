import { error, success } from '.';
import { constants } from '../constants';
import { Posting } from '../types/postings';

export class PostingsApi {
	constructor() {}

	public getPostingById = async (id: number) => {
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

	public refreshPostings = async () => {
		return fetch(`/api/${constants.API_VERSION}/postings/refresh`, {
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

	public getReadPostingsOfSource = async (sourceId: number | string) => {
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
