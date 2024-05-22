import type { PageLoad } from './$types';
import { SourcesHandler } from '../../../lib/types/sources';

/** @type {import('./$types').PageLoad} */
export const load: PageLoad = async ({ params }) => {
	// get slug for selected source and propagate to rest of application
	let sourceSelected = params.slug;
	let sourcesHandler = new SourcesHandler();
	sourcesHandler.setSelectedSource(sourceSelected);

	return {};
};
