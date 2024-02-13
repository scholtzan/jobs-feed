import { goto } from '$app/navigation';
import { Source } from "../../../lib/types";
import { page } from '$app/stores';
import { writable, get } from 'svelte/store';
import { postings } from "../../../lib/store"; 

import type { PageLoad } from "./$types";


/** @type {import('./$types').PageLoad} */
export const load: PageLoad = async ({ params }) => {
    let sourceId = params.slug;

    let source = null;
    if (sourceId != "new") {
        source = await fetch("/sources/" + sourceId).then((data) => data.json()) as Source;
    }

    return {
        source: source
    }
}
