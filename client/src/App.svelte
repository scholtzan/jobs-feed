<script lang="ts">
	import { onMount, onDestroy } from "svelte";
    import Sidebar from "./components/Sidebar.svelte";
    import Toolbar from "./components/Toolbar.svelte";
    import PostingsContainer from "./components/PostingsContainer.svelte";
    import SourceDrawer from "./components/SourceDrawer.svelte";

	onMount(async () => {
		
	});

	onDestroy(() => {

	});
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
        <SourceDrawer></SourceDrawer>
    </div>
</div>
