export const ssr = false;
export const csr = true;
export const trailingSlash = 'always';

import '../app.postcss';
import { SettingsHandler, Settings } from '../lib/types/settings';
import { SourcesHandler } from '../lib/types/sources';
import { PostingsHandler } from '../lib/types/postings';
import { FiltersHandler } from '../lib/types/filters';
import { NotificationHandler } from '../lib/types/notifications';

import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
	let settingsHandler = new SettingsHandler();
	let sourcesHandler = new SourcesHandler();
	let postingsHandler = new PostingsHandler();
	let filtersHandler = new FiltersHandler();
	let notificationHandler = new NotificationHandler();

	// refresh all data
	settingsHandler.refresh().then((res) => {
		if (!res.isSuccessful) {
			notificationHandler.addError('Cannot get settings', res.message);
		} else if (!res.data) {
			let settings = res.data as Settings;
			if (!settings.api_key) {
				notificationHandler.addMessage('[Please set an OpenAI API key in Settings](/preferences).');
			}
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
