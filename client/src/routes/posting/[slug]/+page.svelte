<script lang="ts">
    import { goto } from '$app/navigation';
    import { Posting } from "../../../lib/types";
    import { page } from '$app/stores';
    import { writable, get } from 'svelte/store';
    import { postings } from "../../../lib/store"; 
    import type { PageData } from "./$types";
    import { Postings } from "../../../lib/types/postings";

    let postingsHandler = new Postings();
    let drawerOpen = true;
    export let data: PageData;
    let postingId = data.postingId;
    let posting = postingsHandler.postingById(postingId);

    postingsHandler.subscribe((_) => {
        posting = postingsHandler.postingById(postingId)
    });

    function bookmark() {
        postingsHandler.bookmarkPosting(postingId).then((res) => {
            if (!res.isSuccessful) {
                console.log(res.message);
            }
        });
    }

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
                <h1 class="text-4xl font-bold py-8">{ posting.title }
                    <button class="btn btn-ghost btn-square px-2" on:click={bookmark}>
                        {#if posting.bookmarked}
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6">
                                <path fill-rule="evenodd" d="M6.32 2.577a49.255 49.255 0 0 1 11.36 0c1.497.174 2.57 1.46 2.57 2.93V21a.75.75 0 0 1-1.085.67L12 18.089l-7.165 3.583A.75.75 0 0 1 3.75 21V5.507c0-1.47 1.073-2.756 2.57-2.93Z" clip-rule="evenodd" />
                            </svg>
                        {:else}                
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M17.593 3.322c1.1.128 1.907 1.077 1.907 2.185V21L12 17.25 4.5 21V5.507c0-1.108.806-2.057 1.907-2.185a48.507 48.507 0 0 1 11.186 0Z" />
                            </svg>
                        {/if}       
                    </button>
                </h1>

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
