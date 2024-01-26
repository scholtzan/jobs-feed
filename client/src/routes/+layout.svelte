<script lang="ts">
	import { onMount, onDestroy } from "svelte";
    import Sidebar from "../lib/components/Sidebar.svelte";
    import Toolbar from "../lib/components/Toolbar.svelte";
    import PostingsContainer from "../lib/components/PostingsContainer.svelte";
    import SourceDrawer from "../lib/components/SourceDrawer.svelte";
    import type { PageData } from "./$types";

	export let data: PageData;
    let sources = data.sources;
    console.log(sources);
</script>

<div class="flex flex-column">
    <Sidebar></Sidebar>

	<div class="grow">
        <div>
            <Toolbar></Toolbar>
        </div>
        <div class="p-8">
            <PostingsContainer></PostingsContainer>
        </div>
	</div>

    <div>
        <slot></slot>
    </div>
</div>
