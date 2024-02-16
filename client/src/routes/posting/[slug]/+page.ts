import type { PageLoad } from "./$types";


/** @type {import('./$types').PageLoad} */
export const load: PageLoad = async ({ params }) => {
    let postingId = params.slug;

    return {
        postingId: postingId
    }
}
