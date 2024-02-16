import type { PageLoad } from './$types';

/** @type {import('./$types').PageLoad} */
export const load: PageLoad = async ({ params }) => {
	let sourceId = params.slug;

	return {
		sourceId: sourceId
	};
};
