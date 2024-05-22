import type { PageLoad } from './$types';

/** @type {import('./$types').PageLoad} */
export const load: PageLoad = async ({ params }) => {
	// get posting ID from URL
	let postingId = params.slug;

	return {
		postingId: postingId
	};
};
