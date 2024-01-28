<script lang="ts">
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';
    import { writable, get } from 'svelte/store';
    import { filters } from "../../lib/store"; 
	import { Filter } from '../../lib/types';

    const filterSuggestions = [
        "Skills",
        "Location",
        "Job Title"
    ];
    let storedFilters = get(filters);
    let drawerOpen = true;
    filters.subscribe((value) => {
        storedFilters = get(filters);
    })


    function closeDrawer() {
        goto("/");
    }

    function updateFilters() {
        const res = fetch('/filters', {
            method: 'PUT',
            body: JSON.stringify(storedFilters)
        }).then((response) => {
            if (response.status == 200) {
                response.json().then((json) => {
                    const filtersResponse = json.map((f) => Object.assign(new Filter(), f));
                    filters.set(filtersResponse);
                    goto("/");
                })
                
            } else {
                // todo: error
                console.log("Cannot update filters");
            }
        });
    }

    function removeFilter(filter) {
        filters.set(storedFilters.filter((f) => f != filter));
    }

    function addFilter() {
        let newFilter = new Filter();
        filters.set([...storedFilters, newFilter]);
    }
</script>


<div class="drawer drawer-end">
    <input id="filters-drawer" type="checkbox" class="drawer-toggle" checked={drawerOpen} on:click|preventDefault={closeDrawer}/>

    <div class="drawer-side">
        <label for="filters-drawer" aria-label="close sidebar" class="drawer-overlay"></label>

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

            <form class="px-8" id="filters-form" on:submit|preventDefault={updateFilters}>
                <h1 class="text-4xl font-bold py-8">Filters</h1>

                {#each storedFilters as filter}
                <div class="flex items-end gap-2 py-2">
                    <label class="form-control w-1/3 max-w">
                        <div class="label">
                            <span class="label-text">Filter Name</span>
                        </div>

                        <!-- <div
                            class="inline-block {newSource.name.trim() == "" && nameValidation ? 'tooltip tooltip-open tooltip-error' : ''}"
                            data-tip={nameValidation || null}
                        > -->
                        <div class="dropdown">
                            <input type="text" placeholder="Filter name" 
                                tabIndex="0"
                                class="input input-bordered w-full max-w"
                                bind:value={filter.name} />
                        <!-- </div> -->
                            {#if !filterSuggestions.every(s => storedFilters.find(f => f.name == s))}
                            <ul
                                tabindex="0"
                                class="dropdown-content dropdown-open menu p-2 shadow bg-base-100 rounded-box w-full z-40"
                            >
                                {#each filterSuggestions as filterSuggestion}
                                    {#if !storedFilters.find(f => f.name == filterSuggestion) }
                                        <li>
                                            <span on:click={()=>filter.name=filterSuggestion}>{filterSuggestion}</span>
                                        </li>
                                    {/if}
                                {/each}
                            </ul>
                            {/if}
                        </div>
                    </label>

                    <label class="form-control w-3/5 max-w">
                        <div class="label">
                            <span class="label-text">Filter Value</span>
                        </div>

                        <!-- <div
                            class="inline-block {newSource.url.trim() == "" && urlValidation ? 'tooltip tooltip-open tooltip-error' : ''}"
                            data-tip={urlValidation || null}
                        > -->
                            <input type="text" placeholder="Filter value" 
                                class="input input-bordered w-full max-w" 
                                bind:value={filter.value} />
                        <!-- </div> -->
                    </label>

                    <button class="btn btn-active btn-square" on:click|preventDefault={() => removeFilter(filter)}>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0" />
                        </svg>                          
                    </button>
                </div>
                {/each}

                <button class="btn btn-active" on:click|preventDefault={addFilter}>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
                    </svg>
                    Add Filter                      
                </button>


                <div class="py-8 flex-none">
                    <button class="btn btn-active btn-primary" form="filters-form">Save</button>
                    <a href="/" class="btn btn-active">Cancel</a>
                </div>
            </form>
        </div>
    </div>
</div>
