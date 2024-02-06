<script lang="ts">
    import { goto } from '$app/navigation';
    import { Source } from "../../lib/types";
    import { page } from '$app/stores';
    import { writable, get } from 'svelte/store';
    import { settings } from "../../lib/store"; 

    let drawerOpen = true;

    let storedSettings = get(settings);
    settings.subscribe((value) => {
        storedSettings = get(settings);
    });

    function closeDrawer(e) {
        goto("/");
    }

    function updateSettings() {
        const res = fetch('/settings', {
            method: 'PUT',
            body: JSON.stringify(settings)
        }).then((response) => {
            if (response.status == 200) {
                response.json().then((json) => {
                    let newSettings: Settings = Object.assign(new Settings(), json);                        
                    settings.set(newSettings);
                    goto("/");
                })
                
            } else {
                // todo: error
                console.log("Cannot update settings");
            }
        });
    }
</script>


<div class="drawer drawer-end">
    <input id="settings-drawer" type="checkbox" class="drawer-toggle" checked={drawerOpen} on:click|preventDefault={closeDrawer}/>

    <div class="drawer-side">
        <label for="settings-drawer" aria-label="close sidebar" class="drawer-overlay"></label>

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

            <form class="px-8" id="settings-form" on:submit|preventDefault={updateSettings}>
                <h1 class="text-4xl font-bold py-8">Settings</h1>

                <label class="form-control w-full max-w">
                    <div class="label">
                        <span class="label-text">API Key</span>
                    </div>

                    <input type="text" placeholder="API Key" 
                        class="input input-bordered w-full max-w"
                        bind:value={storedSettings.api_key} />
                </label>

                <div class="py-8 flex-none">
                    <button class="btn btn-active btn-primary" form="settings-form">Save</button>
                    <a href="/" class="btn btn-active">Close</a>
                </div>
            </form>
        </div>
    </div>
</div>
