<script lang="ts">
    import { goto } from '$app/navigation';
    import { Source } from "../../../lib/types";
    import { page } from '$app/stores';
    import { writable, get } from 'svelte/store';
    import { sources } from "../../../lib/store"; 

    let drawerOpen = true;
    let newSource = new Source();

    let urlValidation = null;
    let nameValidation = null;

    function closeDrawer(e) {
        goto("/");
    }

    function addSource() {
        if (newSource.name.trim() == "") {
            nameValidation = "Give this source a name";
        }

        if (newSource.url.trim() == "") {
            urlValidation = "Set a URL for this source";
        }

        if (newSource.url.trim() != "" && newSource.url.trim() != "") {
            const res = fetch('/sources', {
                method: 'POST',
                body: JSON.stringify(newSource)
            }).then((response) => {
                if (response.status == 200) {
                    const sourceJson = response.json().then((json) => {
                        let storedSource: Source = Object.assign(new Source(), json);                        
                        sources.set([ ...get(sources), storedSource]);
                        goto("/");
                    })
                    
                } else {
                    // todo: error
                    console.log("Cannot add source");
                }
            });
        }
        
    }
</script>


<div class="drawer drawer-end">
    <input id="new-source-drawer" type="checkbox" class="drawer-toggle" checked={drawerOpen} on:click|preventDefault={closeDrawer}/>

    <div class="drawer-side">
        <label for="new-source-drawer" aria-label="close sidebar" class="drawer-overlay"></label>

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

            <form class="px-8" id="new-source-form" on:submit|preventDefault={addSource}>
                <h1 class="text-4xl font-bold py-8">New Source</h1>

                <label class="form-control w-full max-w">
                    <div class="label">
                        <span class="label-text">Name</span>
                    </div>

                    <div
                        class="inline-block {newSource.name.trim() == "" && nameValidation ? 'tooltip tooltip-open tooltip-error' : ''}"
                        data-tip={nameValidation || null}
                    >
                        <input type="text" placeholder="Source name" 
                            class="input input-bordered w-full max-w {newSource.name.trim() == "" && nameValidation ? 'input-error' : ''}"
                            bind:value={newSource.name} />
                    </div>
                </label>

                <label class="form-control w-full max-w">
                    <div class="label">
                        <span class="label-text">URL</span>
                    </div>

                    <div
                        class="inline-block {newSource.url.trim() == "" && urlValidation ? 'tooltip tooltip-open tooltip-error' : ''}"
                        data-tip={urlValidation || null}
                    >
                        <input type="text" placeholder="URL to Source" 
                            class="input input-bordered w-full max-w {newSource.url.trim() == "" && urlValidation ? 'input-error' : ''}" 
                            bind:value={newSource.url} />
                    </div>
                </label>

                <div class="py-4">
                    <details class="collapse bg-base-200 collapse-arrow border border-base-300">
                        <summary class="collapse-title font-medium">Advanced Settings</summary>
                        <div class="collapse-content"> 
                            <label class="form-control w-full max-w">
                                <div class="label">
                                    <span class="label-text">Pagination</span>
                                </div>
                            <input type="text" placeholder="Link with pagination" class="input input-bordered w-full max-w" bind:value={newSource.pagination} />
                            </label>

                            <label class="form-control w-full max-w">
                                <div class="label">
                                    <span class="label-text">Selector</span>
                                </div>
                            <input type="text" placeholder="CSS Path to element with content" class="input input-bordered w-full max-w" bind:value={newSource.selector} />
                            </label>
                        </div>
                    </details>
                </div>

                <div class="py-8 flex-none">
                    <button class="btn btn-active btn-primary" form="new-source-form">Save</button>
                    <a href="/" class="btn btn-active">Cancel</a>
                </div>
            </form>
        </div>
    </div>
</div>
