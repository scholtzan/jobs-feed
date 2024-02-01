import { goto } from '$app/navigation';
import { Posting } from "../../../lib/types";
import { page } from '$app/stores';
import { writable, get } from 'svelte/store';
import { postings } from "../../../lib/store"; 

import type { PageLoad } from "./$types";


/** @type {import('./$types').PageLoad} */
export const load: PageLoad = async ({ params }) => {
    let postingId = params.slug;
    let posting = await fetch("/postings/" + postingId).then((data) => data.json()) as Posting;

    return {
        posting: posting
    }
}