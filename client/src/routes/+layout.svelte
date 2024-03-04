<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import Sidebar from '../lib/components/Sidebar.svelte';
	import Toolbar from '../lib/components/Toolbar.svelte';
	import PostingsContainer from '../lib/components/PostingsContainer.svelte';
	import SourceDrawer from '../lib/components/SourceDrawer.svelte';
	import NotificationContainer from '../lib/components/NotificationContainer.svelte';
</script>

<div class="flex flex-column">
	<div class="static flex flex-column">
		<Sidebar></Sidebar>

		<NotificationContainer></NotificationContainer>
	</div>

	<div class="grow">
		<div>
			<Toolbar></Toolbar>
		</div>
		<div class="p-8 flex justify-center">
			<PostingsContainer></PostingsContainer>
		</div>
	</div>

	<div>
		<slot />
	</div>
</div>
