export const ssr = false;
export const csr = true;
export const trailingSlash = 'always';

import '../app.postcss';
import { SettingsHandler } from '../lib/types/settings';
import { Sources } from '../lib/types/sources';
import { Postings } from '../lib/types/postings';
import { Filters } from '../lib/types/filters';

import type { PageLoad } from './$types';
import { NotificationHandler } from '../lib/types/notifications';

export const load: PageLoad = async () => {
	let settingsHandler = new SettingsHandler();
	let sourcesHandler = new Sources();
	let postingsHandler = new Postings();
	let filtersHandler = new Filters();
	let notificationHandler = new NotificationHandler();

	settingsHandler.refresh().then((res) => {
		if (!res.isSuccessful) {
			notificationHandler.addError('Cannot get settings', res.message);
		}
	});

	sourcesHandler.refresh().then((res) => {
		if (!res.isSuccessful) {
			notificationHandler.addError('Cannot get sources', res.message);
		}
	});

	postingsHandler.refresh(true).then((res) => {
		if (!res.isSuccessful) {
			notificationHandler.addError('Cannot get postings', res.message);
		}
	});

	filtersHandler.refresh().then((res) => {
		if (!res.isSuccessful) {
			notificationHandler.addError('Cannot get filters', res.message);
		}
	});

	return {};
};
