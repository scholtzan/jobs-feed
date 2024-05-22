<!--
  @component
  Sidebar showing available sources
-->
<script lang="ts">
	import { Posting, PostingsHandler } from '../types/postings';
	import { SourcesHandler } from '../types/sources';
	import { NotificationHandler } from '../types/notifications';
	import { goto } from '$app/navigation';
	import { SettingsHandler } from '../types/settings';
	import { constants } from '../constants';
	import { showSidebar } from '../store';
	import { get } from 'svelte/store';

	let notificationHandler = new NotificationHandler();
	let postingsHandler = new PostingsHandler();
	let sourcesHandler = new SourcesHandler();
	let settingsHandler = new SettingsHandler();

	// context menu state
	let sourceContextMenu: null | number = null;
	let lastSourceContextMenu: null | number = null;
	let sourceNameContextMenu: null | string = null;
	let contextMenuPosition = { x: 0, y: 0 };

	let settings = settingsHandler.settings;
	let newPostings: Posting[] = postingsHandler.postings;
	let postingsToday = postingsHandler.getTodaysPostings();
	let postingsPerSource = postingsHandler.postingsBySource();
	let storedSources = sourcesHandler.sortedSources();
	let selected = sourcesHandler.selectedSource;

	// side bar state
	let isRefreshing = false;
	let isSidebarVisible = false;

	settingsHandler.subscribe((_value) => {
		settings = settingsHandler.settings;
	});

	sourcesHandler.subscribe((_value) => {
		storedSources = sourcesHandler.sortedSources();
	});

	sourcesHandler.subscribeSelectedSource((_value) => {
		selected = sourcesHandler.selectedSource;
	});

	showSidebar.subscribe((_) => (isSidebarVisible = get(showSidebar)));

	postingsHandler.subscribe((_) => {
		newPostings = postingsHandler.postings.filter((p) => !p.seen);
		postingsToday = postingsHandler.getTodaysPostings().filter((p) => !p.seen);
		postingsPerSource = postingsHandler.postingsBySource();
	});

	/**
	 * Refresh postings by requesting server to scrape sources.
	 * @param source_id [optional] ID of source to refresh postings for
	 */
	function refreshPostings(source_id: number | null = null): void {
		isRefreshing = true;
		var doneRefreshing = 0;
		for (var source of storedSources) {
			if ((source_id != null && source.id == source_id) || source_id == null) {
				source.refreshing = true;
				storedSources = storedSources;
				let sid = source.id;

				postingsHandler.refresh(false, source.id).then((res) => {
					doneRefreshing += 1;
					let index = storedSources
						.map(function (x) {
							return x.id;
						})
						.indexOf(sid);
					storedSources[index].refreshing = false;

					isRefreshing = doneRefreshing < storedSources.length && source_id == null;

					if (!res.isSuccessful) {
						notificationHandler.addError(
							`Could not refresh postings for ${storedSources[index].name}`,
							res.message
						);
					}

					if (!isRefreshing) {
						sourcesHandler.refresh().then((res) => {
							if (!res.isSuccessful) {
								notificationHandler.addError('Could not refresh sources', res.message);
							}
						});
					}
				});
			}
		}
	}

	/**
	 * Show context menu based on right-click event on a source entry.
	 * @param event right-click event, used to postition the context menu
	 * @param sourceId ID of source that was right-clicked on
	 * @param sourceName name of source that was clicked on
	 */
	function contextMenu(
		event: MouseEvent,
		sourceId: null | number,
		sourceName: null | string
	): void {
		event.preventDefault();
		sourceContextMenu = sourceId;
		sourceNameContextMenu = sourceName;
		contextMenuPosition = { x: event.pageX, y: event.pageY };
	}

	/**
	 * Remove a specific source.
	 * @param sourceId ID of source to remove
	 */
	function deleteSource(sourceId: number | null): void {
		lastSourceContextMenu = null;
		sourcesHandler.deleteSource(sourceId).then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not delete source', res.message);
			}
		});
	}

	/**
	 * Open the URL to a specific source.
	 * @param sourceId ID of source to open URL for
	 */
	function openSource(sourceId: number | null) {
		let source = sourcesHandler.sourceById(sourceId);

		if (source != undefined) {
			window.open(source.url, '_blank');
		} else {
			notificationHandler.addError('Could not open link to source. Source does not exist');
		}
	}

	/**
	 * Hide or show source side bar.
	 */
	function toggleSidebar() {
		isSidebarVisible = !get(showSidebar);
		showSidebar.set(isSidebarVisible);
	}
</script>

<div class="max-w-[20em] min-w-[16em]">
	<aside class="h-screen sticky top-0 flex flex-col bg-base-200">
		<!-- Header -->
		<div class="flex gap-x-20 justify-between p-2 h-16 border-b border-base-300 align-bottom">
			<!-- Button to toggle source side bar -->
			<button
				class="btn btn-ghost btn-square lg:hidden visible {isSidebarVisible ? 'btn-active' : ''}"
				title="Show sources"
				on:click={toggleSidebar}
			>
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

			<!-- Logo -->
			<a title="Jobs Feed" href="/" class="h-12 w-12 lg:block hidden">
				<svg
					width="100%"
					height="100%"
					viewBox="0 0 1182 1182"
					version="1.1"
					xmlns="http://www.w3.org/2000/svg"
					xmlns:xlink="http://www.w3.org/1999/xlink"
					xml:space="preserve"
					xmlns:serif="http://www.serif.com/"
					style="fill-rule:evenodd;clip-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:1.5;"
				>
					<path
						d="M1181.1,105.837L1181.1,1075.27C1181.1,1133.68 1133.68,1181.1 1075.27,1181.1L105.837,1181.1C47.424,1181.1 0,1133.68 0,1075.27L0,105.837C0,47.424 47.424,0 105.837,0L1075.27,0C1133.68,0 1181.1,47.424 1181.1,105.837Z"
						style="fill:url(#_Radial1);"
					/>
					<g transform="matrix(0.974472,0,0,0.974472,15.0757,27.0862)">
						<path
							d="M472.441,578.74L708.661,578.74L590.551,578.74C590.551,578.74 590.073,331.65 590.551,254.258C590.602,246.012 600.231,176.574 688.788,175.14C771.217,173.805 792.505,215.415 803.15,236.22"
							style="fill:none;stroke:white;stroke-width:132.55px;"
						/>
					</g>
					<g transform="matrix(-0.974472,0,0,-0.974472,1166.03,1154.02)">
						<path
							d="M472.441,578.74L708.661,578.74L590.551,578.74C590.551,578.74 590.073,331.65 590.551,254.258C590.602,246.012 600.231,176.574 688.788,175.14C771.217,173.805 792.505,215.415 803.15,236.22"
							style="fill:none;stroke:white;stroke-width:132.55px;"
						/>
					</g>
					<defs>
						<radialGradient
							id="_Radial1"
							cx="0"
							cy="0"
							r="1"
							gradientUnits="userSpaceOnUse"
							gradientTransform="matrix(755.906,0,0,755.906,590.551,590.551)"
							><stop offset="0" style="stop-color:rgb(255,116,0);stop-opacity:1" /><stop
								offset="0.65"
								style="stop-color:rgb(251,114,2);stop-opacity:1"
							/><stop
								offset="1"
								style="stop-color:rgb(226,104,12);stop-opacity:1"
							/></radialGradient
						>
					</defs>
				</svg>
			</a>

			<!-- Button to add new source -->
			<div class="flex flex-row justify-end">
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
					title={settings.api_key
						? 'Refresh Postings'
						: 'OpenAI API Key needs to be set in Settings.'}
					class="btn btn-ghost btn-square {isRefreshing || !settings.api_key ? 'btn-disabled' : ''}"
					on:click={() => refreshPostings()}
				>
					{#if isRefreshing}
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
		</div>

		<!-- Body -->
		<div class="flex flex-col border-base-300 px-2 pt-4 grow overflow-y-auto">
			<!-- Links -->
			<div class="flex flex-col divide-y divide-base-300">
				<ul class="menu menu-sm px-0">
					<!-- Button to show today's postings -->
					<li class="font-bold">
						<button
							title="Postings Added Today"
							on:click={() => {
								toggleSidebar();
								goto('/postings/today');
							}}
							class={selected == 'today' ? 'active' : ''}
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
								stroke-width="1.5"
								stroke="currentColor"
								class="w-6 h-6"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									d="M15.59 14.37a6 6 0 0 1-5.84 7.38v-4.8m5.84-2.58a14.98 14.98 0 0 0 6.16-12.12A14.98 14.98 0 0 0 9.631 8.41m5.96 5.96a14.926 14.926 0 0 1-5.841 2.58m-.119-8.54a6 6 0 0 0-7.381 5.84h4.8m2.581-5.84a14.927 14.927 0 0 0-2.58 5.84m2.699 2.7c-.103.021-.207.041-.311.06a15.09 15.09 0 0 1-2.448-2.448 14.9 14.9 0 0 1 .06-.312m-2.24 2.39a4.493 4.493 0 0 0-1.757 4.306 4.493 4.493 0 0 0 4.306-1.758M16.5 9a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Z"
								/>
							</svg>
							Today
							{#if postingsToday.length > 0}
								<div class="badge badge-neutral">{postingsToday.length}</div>
							{/if}
						</button>
					</li>

					<!-- Button to show all postings -->
					<li class="font-bold">
						<button
							title="All Postings"
							on:click={() => {
								toggleSidebar();
								goto('/postings/all');
							}}
							class={selected == 'all' ? 'active' : ''}
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
								stroke-width="1.5"
								stroke="currentColor"
								class="w-6 h-6"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									d="M3.75 12h16.5m-16.5 3.75h16.5M3.75 19.5h16.5M5.625 4.5h12.75a1.875 1.875 0 0 1 0 3.75H5.625a1.875 1.875 0 0 1 0-3.75Z"
								/>
							</svg>
							All
							{#if newPostings.length > 0}
								<div class="badge badge-neutral">{newPostings.length}</div>
							{/if}
						</button>
					</li>

					<li class="font-bold">
						<!-- Button to show bookmarked postings -->
						<button
							title="Bookmarked Postings"
							on:click={() => {
								toggleSidebar();
								goto('/postings/bookmarked');
							}}
							class={selected == 'bookmarked' ? 'active' : ''}
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								viewBox="0 0 24 24"
								fill="currentColor"
								class="w-6 h-6"
							>
								<path
									fill-rule="evenodd"
									d="M6.32 2.577a49.255 49.255 0 0 1 11.36 0c1.497.174 2.57 1.46 2.57 2.93V21a.75.75 0 0 1-1.085.67L12 18.089l-7.165 3.583A.75.75 0 0 1 3.75 21V5.507c0-1.47 1.073-2.756 2.57-2.93Z"
									clip-rule="evenodd"
								/>
							</svg>
							Bookmarked
						</button>
					</li>

					<h2 class="menu-title">Sources</h2>

					{#if storedSources.length == 0}
						<!-- Show add source button if no source has been created so far -->
						<div class="flex justify-center">
							<a href="/source/new" class="btn btn-sm btn-active w-1/2">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									fill="none"
									viewBox="0 0 24 24"
									stroke-width="1.5"
									stroke="currentColor"
									class="w-5 h-5"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										d="M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
									/>
								</svg>
								Add Source
							</a>
						</div>
					{/if}

					{#each storedSources as source}
						<!-- Source buttons -->
						<li class="font-bold">
							<button
								title={source.name}
								on:click={() => {
									toggleSidebar();
									goto(`/postings/${source.id}`);
								}}
								class={selected == source.id ? 'active' : ''}
								on:contextmenu={(e) => contextMenu(e, source.id, source.name)}
							>
								<!-- Source favicon -->
								<img
									alt=""
									height="16"
									width="16"
									src="https://www.google.com/s2/favicons?sz=48&domain={source.favicon == null
										? source.url
										: source.favicon}&amp;alt=feed"
								/>
								{source.name}

								{#if source.id != null && source.id in postingsPerSource && postingsPerSource[source.id].filter((p) => !p.seen).length > 0}
									<div class="badge badge-neutral">
										{postingsPerSource[source.id].filter((p) => !p.seen).length}
									</div>
								{/if}

								<!-- Indicator whether source postings are refreshing -->
								{#if source.refreshing}
									<span
										title="Looking for new job postings."
										class="loading loading-spinner loading-sm"
									></span>
								{:else if source.unreachable}
									<div title="Unreachable" class="badge badge-neutral">
										<svg
											xmlns="http://www.w3.org/2000/svg"
											fill="none"
											viewBox="0 0 24 24"
											stroke-width="1.5"
											stroke="currentColor"
											class="w-4 h-4"
										>
											<path
												stroke-linecap="round"
												stroke-linejoin="round"
												d="m3 3 8.735 8.735m0 0a.374.374 0 1 1 .53.53m-.53-.53.53.53m0 0L21 21M14.652 9.348a3.75 3.75 0 0 1 0 5.304m2.121-7.425a6.75 6.75 0 0 1 0 9.546m2.121-11.667c3.808 3.807 3.808 9.98 0 13.788m-9.546-4.242a3.733 3.733 0 0 1-1.06-2.122m-1.061 4.243a6.75 6.75 0 0 1-1.625-6.929m-.496 9.05c-3.068-3.067-3.664-7.67-1.79-11.334M12 12h.008v.008H12V12Z"
											/>
										</svg>
									</div>
								{:else if source.content != null && source.content.length >= constants.SOURCE_CONTENT_LENGTH_WARN}
									<div
										title="High source volume. Some postings might be missing"
										class="badge badge-neutral"
									>
										!
									</div>
								{/if}
							</button>
						</li>
					{/each}
				</ul>
			</div>
		</div>
	</aside>
</div>

<!-- Context menu -->
{#if sourceContextMenu != null}
	<ul
		id="contextMenu"
		class="menu bg-base-200 w-56 rounded-box absolute bg-base-100 shadow-xl"
		style="position: absolute; left: {contextMenuPosition.x}px; top: {contextMenuPosition.y}px;"
	>
		<li>
			<button>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 24 24"
					fill="currentColor"
					class="w-6 h-6"
				>
					<path
						fill-rule="evenodd"
						d="M19.916 4.626a.75.75 0 0 1 .208 1.04l-9 13.5a.75.75 0 0 1-1.154.114l-6-6a.75.75 0 0 1 1.06-1.06l5.353 5.353 8.493-12.74a.75.75 0 0 1 1.04-.207Z"
						clip-rule="evenodd"
					/>
				</svg>
				Mark as Read
			</button>
		</li>
		<li>
			<a href="/source/{sourceContextMenu}">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke-width="1.5"
					stroke="currentColor"
					class="w-6 h-6"
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
				Edit
			</a>
		</li>
		<li class={storedSources.find((s) => s.id == sourceContextMenu)?.refreshing ? 'disabled' : ''}>
			<button on:click={() => refreshPostings(sourceContextMenu)}>
				{#if storedSources.find((s) => s.id == sourceContextMenu)?.refreshing}
					<span class="loading loading-spinner"></span>
				{:else}
					<svg
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						stroke-width="1.5"
						stroke="currentColor"
						class="w-6 h-6"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"
						/>
					</svg>
				{/if}
				Refresh
			</button>
		</li>
		<li>
			<button on:click={() => openSource(sourceContextMenu)}>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke-width="1.5"
					stroke="currentColor"
					class="w-6 h-6"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M13.5 6H5.25A2.25 2.25 0 0 0 3 8.25v10.5A2.25 2.25 0 0 0 5.25 21h10.5A2.25 2.25 0 0 0 18 18.75V10.5m-10.5 6L21 3m0 0h-5.25M21 3v5.25"
					/>
				</svg>
				Visit Page
			</button>
		</li>
		<li>
			<button onclick="confirm_remove_modal.showModal()">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke-width="1.5"
					stroke="currentColor"
					class="w-6 h-6"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"
					/>
				</svg>
				Remove
			</button>
		</li>
	</ul>
{/if}

<dialog id="confirm_remove_modal" class="modal">
	<div class="modal-box">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button>
		</form>
		<h3 class="font-bold text-lg">Remove '{sourceNameContextMenu}'?</h3>
		<p class="py-4">Please confirm to remove the source '{sourceNameContextMenu}'.</p>

		<form method="dialog" class="modal-backdrop">
			<div class="py-8 flex-none">
				<button class="btn btn-active">Cancel</button>
				<button
					class="btn btn-active btn-error"
					on:click={() => deleteSource(lastSourceContextMenu)}>Yes, Remove</button
				>
			</div>
		</form>
	</div>
</dialog>

<svelte:window
	on:click={() => {
		lastSourceContextMenu = sourceContextMenu;
		sourceContextMenu = null;
	}}
/>
