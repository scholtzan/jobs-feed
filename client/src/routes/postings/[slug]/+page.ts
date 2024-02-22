import type { PageLoad } from './$types';
import { Sources } from '../../../lib/types/sources';

/** @type {import('./$types').PageLoad} */
export const load: PageLoad = async ({ params }) => {
	let sourceSelected = params.slug;
	let sourcesHandler = new Sources();
	sourcesHandler.setSelectedSource(sourceSelected);

	return {};
};
