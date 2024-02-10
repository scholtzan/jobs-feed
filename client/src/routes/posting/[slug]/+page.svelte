<script lang="ts">
    import { goto } from '$app/navigation';
    import { Posting } from "../../../lib/types";
    import { page } from '$app/stores';
    import { writable, get } from 'svelte/store';
    import { postings } from "../../../lib/store"; 
    import type { PageData } from "./$types";

    let drawerOpen = true;
    export let data: PageData;
    let posting = data.posting;

    // /** @type {import('./$types').PageLoad} */
    // export function load({ params }) {
    //     console.log("ddddd");
    //     let postingId = params.slug;

    //     let storedPostings = get(postings);
    //     console.log(storedPostings);
    //     console.log(postingId);
    //     posting = storedPostings.find((p) => p.id == postingId);
    // }

    function closeDrawer(e) {
        goto("/");
    }
</script>


<div class="drawer drawer-end">
    <input id="posting-drawer" type="checkbox" class="drawer-toggle" checked={drawerOpen} on:click|preventDefault={closeDrawer}/>

    <div class="drawer-side">
        <label for="posting-drawer" aria-label="close sidebar" class="drawer-overlay"></label>

        <div class="w-3/4 min-h-full bg-base-200 text-base-content">
            <nav class="navbar py-4">
                <div class="flex-none">
                    <a href="/" class="btn btn-square btn-ghost btn-sm">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="m18.75 4.5-7.5 7.5 7.5 7.5m-6-15L5.25 12l7.5 7.5" />
                        </svg>
                    </a>
                </div>
                <div class="flex-1">

                </div>
                <div class="flex-none">

                </div>
            </nav>

            <div class="px-8">
                <h1 class="text-4xl font-bold py-8">{ posting.title }</h1>

                <p class="w-full max-w">
                    { posting.description }
                </p>

                <div class="py-8 flex-none">
                    <a href="{ posting.url }" target="_blank" class="btn btn-active w-full max-w">Go To Posting</a>
                </div>
            </div>
        </div>
    </div>
</div>
