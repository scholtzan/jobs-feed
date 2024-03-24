<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import Sidebar from '../lib/components/Sidebar.svelte';
	import Toolbar from '../lib/components/Toolbar.svelte';
	import PostingsContainer from '../lib/components/PostingsContainer.svelte';
	import SourceDrawer from '../lib/components/SourceDrawer.svelte';
	import NotificationContainer from '../lib/components/NotificationContainer.svelte';
	import { showSidebar } from '../lib/store';
	import { get } from 'svelte/store';

	let isSidebarVisible = false;
	showSidebar.subscribe((_) => (isSidebarVisible = get(showSidebar)));

	function toggleSidebar() {
		isSidebarVisible = !get(showSidebar);
		showSidebar.set(isSidebarVisible);
	}
</script>

<div class="drawer lg:drawer-open min-h-screen">
	<input
		id="sidebar-drawer"
		type="checkbox"
		class="drawer-toggle"
		checked={isSidebarVisible}
		on:click={toggleSidebar}
	/>
	<div class=" drawer-side overflow-visible">
		<label for="sidebar-drawer" aria-label="close sidebar" class="drawer-overlay"></label>

		<Sidebar></Sidebar>
	</div>

	<div class="drawer-content">
		<div>
			<Toolbar></Toolbar>
		</div>
		<PostingsContainer></PostingsContainer>
		<NotificationContainer></NotificationContainer>
	</div>

	<div>
		<slot />
	</div>
</div>
