import { goto } from '$app/navigation';
import { Posting } from "../../../lib/types";
import { page } from '$app/stores';
import { writable, get } from 'svelte/store';
import { postings } from "../../../lib/store"; 

import type { PageLoad } from "./$types";


/** @type {import('./$types').PageLoad} */
export const load: PageLoad = async ({ params }) => {
    let postingId = params.slug;

    return {
        postingId: postingId
    }
}
