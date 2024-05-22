<!--
  @component
  Top navigation bar
-->
<script lang="ts">
	import { SettingsHandler } from '../types/settings';
	import { PostingsHandler } from '../types/postings';
	import { NotificationHandler } from '../types/notifications';
	import { SourcesHandler } from '../types/sources';
	import { get } from 'svelte/store';
	import { showSidebar } from '../store';

	let settingsHandler = new SettingsHandler();
	let settings = settingsHandler.settings;
	let postingsHandler = new PostingsHandler();
	let notificationHandler = new NotificationHandler();
	let sourcesHandler = new SourcesHandler();

	// whether the source sidebar is shown
	let isSidebarVisible = get(showSidebar);
	// whether postings are refreshing
	let isRefreshing = false;

	settingsHandler.subscribe((_value) => {
		settings = settingsHandler.settings;
	});

	showSidebar.subscribe((_) => (isSidebarVisible = get(showSidebar)));

	/**
	 * Refresh postings.
	 */
	function refreshPostings() {
		isRefreshing = true;
		postingsHandler.refresh(false).then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not refresh postings', res.message);
			}

			isRefreshing = false;
			sourcesHandler.refresh().then((res) => {
				if (!res.isSuccessful) {
					notificationHandler.addError('Could not refresh sources', res.message);
				}
			});
		});
	}

	/**
	 * Show or hide the source side bar.
	 */
	function toggleSidebar() {
		isSidebarVisible = !get(showSidebar);
		showSidebar.set(isSidebarVisible);
	}
</script>

<nav class="navbar border-b border-base-300 h-16">
	<div class="flex-none lg:hidden {isSidebarVisible ? 'hidden' : 'visible'}">
		<!-- Button to toggle the side bar, only visible on smaller devices -->
		<button class="btn btn-ghost btn-square" title="Show sources" on:click={toggleSidebar}>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="w-7 h-7"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
				/>
			</svg>
		</button>

		<!-- Button to add a new source -->
		<a title="Add Source" class="btn btn-ghost btn-square" href="/source/new">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="w-7 h-7"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
				/>
			</svg>
		</a>

		<!-- Button to manage filters -->
		<a title="Set Filters" class="btn btn-ghost btn-square" href="/filter">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="w-7 h-7"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M10.5 6h9.75M10.5 6a1.5 1.5 0 1 1-3 0m3 0a1.5 1.5 0 1 0-3 0M3.75 6H7.5m3 12h9.75m-9.75 0a1.5 1.5 0 0 1-3 0m3 0a1.5 1.5 0 0 0-3 0m-3.75 0H7.5m9-6h3.75m-3.75 0a1.5 1.5 0 0 1-3 0m3 0a1.5 1.5 0 0 0-3 0m-9.75 0h9.75"
				/>
			</svg>
		</a>

		<!-- Button to refresh postings -->
		<button
			title={settings.api_key ? 'Refresh Postings' : 'OpenAI API Key needs to be set in Settings.'}
			class="btn btn-ghost btn-square {isRefreshing || !settings.api_key ? 'btn-disabled' : ''}"
			on:click={refreshPostings}
		>
			{#if isRefreshing}
				<!-- Disable button while refresh is in progress -->
				<span class="loading loading-spinner"></span>
			{:else}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke-width="1.5"
					stroke="currentColor"
					class="w-7 h-7"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"
					/>
				</svg>
			{/if}
		</button>
	</div>

	<div class="flex-1"></div>
	<div class="flex-none">
		<!-- Button to open the settings drawer dialog -->
		<a title="Settings" class="btn btn-square btn-ghost" href="/preferences">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="w-7 h-7"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.28Z"
				/>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
				/>
			</svg>
		</a>
	</div>
</nav>
