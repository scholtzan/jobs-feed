<script lang="ts">
	import { goto } from '$app/navigation';
	import { Postings } from '../types/postings';
	import { Sources, SelectedSource, Source } from '../types/sources';
	import { NotificationHandler } from '../types/notifications';
	import { Filters } from '../types/filters';

	let notificationHandler = new NotificationHandler();
	let sourcesHandler = new Sources();
	let postingsHandler = new Postings();
	let filtersHandler = new Filters();
	let selectedSourceId = sourcesHandler.selectedSource;
	let sources = sourcesHandler.sources;
	let postings = postingsHandler.postings;
	let source;
	getPostingsForSelectedSource();

	postingsHandler.subscribe((_) => {
		getPostingsForSelectedSource();
	});

	sourcesHandler.subscribeSelectedSource((value) => {
		selectedSourceId = sourcesHandler.selectedSource;
		getPostingsForSelectedSource();
	});

	filtersHandler.subscribe((_) => {
		getPostingsForSelectedSource();
	});

	function getPostingsForSelectedSource() {
		if (selectedSourceId == SelectedSource.All) {
			postings = postingsHandler.postings;
		} else if (selectedSourceId == SelectedSource.Today) {
			postings = postingsHandler.getTodaysPostings();
		} else if (selectedSourceId == SelectedSource.Bookmarked) {
			postingsHandler.getBookmarked().then((res) => {
				if (!res.isSuccessful) {
					notificationHandler.addError('Could not get bookmarked postings', res.message);
				} else {
					postings = res.data;
				}
			});
		} else {
			source = sourcesHandler.sourceById(selectedSourceId);
			let postingsBySource = postingsHandler.postingsBySource();
			if (selectedSourceId in postingsBySource) {
				postings = postingsBySource[selectedSourceId];
			} else {
				postings = [];
			}
		}
	}

	sourcesHandler.subscribe((_) => {
		sources = sourcesHandler.sources;
	});

	function markAsRead(ids) {
		postingsHandler.markAsRead(ids).then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not mark posting as read', res.message);
			}
		});
	}

	function bookmark(id) {
		postingsHandler.bookmarkPosting(id).then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not bookmark posting', res.message);
			}
		});
	}

	function viewRead() {
		postingsHandler.getReadPostingsOfSource(selectedSourceId).then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not fetch read postings', res.message);
			} else {
				postings = res.data;
			}
		});
	}

	function openSource(sourceId: number) {
		let source = sourcesHandler.sourceById(sourceId);

		if (source != undefined) {
			window.open(source.url, '_blank');
		} else {
			notificationHandler.addError('Could not open link to source. Source does not exist');
		}
	}

	function getMatchingFilters(content: string) {
		let matchingFilters = [];

		if (content != null) {
			filtersHandler.filters.forEach((filter) => {
				filter.value.split(',').forEach((v) => {
					v = v.trim();
					if (content.includes(v)) {
						matchingFilters.push(v);
					}
				});
			});
		}

		return matchingFilters;
	}
</script>

<div class="w-2/3">
	<div class="flex flex-row grow justify-end px-4 gap-x-2">
		<button
			title="Mark As Read"
			class="btn btn-ghost btn-square"
			on:click={() => markAsRead(postings.map((p) => p.id))}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="w-7 h-7"
			>
				<path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
			</svg>
		</button>
		{#if ![SelectedSource.All, SelectedSource.Bookmarked, SelectedSource.Today].includes(selectedSourceId)}
			<a title="Edit Source" class="btn btn-ghost btn-square" href="/source/{selectedSourceId}">
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
			<button
				title="Visit Page"
				class="btn btn-ghost btn-square"
				on:click={() => openSource(selectedSourceId)}
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
						d="M13.5 6H5.25A2.25 2.25 0 0 0 3 8.25v10.5A2.25 2.25 0 0 0 5.25 21h10.5A2.25 2.25 0 0 0 18 18.75V10.5m-10.5 6L21 3m0 0h-5.25M21 3v5.25"
					/>
				</svg>
			</button>
		{/if}
	</div>
	<h1 class="flex grow text-4xl font-bold py-8 px-4">
		{#if selectedSourceId == SelectedSource.All}
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="w-11 h-11 mr-2"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M3.75 12h16.5m-16.5 3.75h16.5M3.75 19.5h16.5M5.625 4.5h12.75a1.875 1.875 0 0 1 0 3.75H5.625a1.875 1.875 0 0 1 0-3.75Z"
				/>
			</svg>
			All Job Postings
		{:else if selectedSourceId == SelectedSource.Today}
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="w-11 h-11 mr-2"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M15.59 14.37a6 6 0 0 1-5.84 7.38v-4.8m5.84-2.58a14.98 14.98 0 0 0 6.16-12.12A14.98 14.98 0 0 0 9.631 8.41m5.96 5.96a14.926 14.926 0 0 1-5.841 2.58m-.119-8.54a6 6 0 0 0-7.381 5.84h4.8m2.581-5.84a14.927 14.927 0 0 0-2.58 5.84m2.699 2.7c-.103.021-.207.041-.311.06a15.09 15.09 0 0 1-2.448-2.448 14.9 14.9 0 0 1 .06-.312m-2.24 2.39a4.493 4.493 0 0 0-1.757 4.306 4.493 4.493 0 0 0 4.306-1.758M16.5 9a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Z"
				/>
			</svg>
			Today's Job Postings
		{:else if selectedSourceId == SelectedSource.Bookmarked}
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				fill="currentColor"
				class="w-11 h-11 mr-2"
			>
				<path
					fill-rule="evenodd"
					d="M6.32 2.577a49.255 49.255 0 0 1 11.36 0c1.497.174 2.57 1.46 2.57 2.93V21a.75.75 0 0 1-1.085.67L12 18.089l-7.165 3.583A.75.75 0 0 1 3.75 21V5.507c0-1.47 1.073-2.756 2.57-2.93Z"
					clip-rule="evenodd"
				/>
			</svg>

			Bookmarked Postings
		{:else}
			{#if source != undefined}
				<img
					class="mr-2"
					width="40"
					height="40"
					alt=""
					src="https://www.google.com/s2/favicons?sz=32&domain={source.favicon != null
						? source.favicon
						: source.url}&amp;alt=feed"
				/>
			{/if}

			{source == undefined ? goto('/') : source.name}
		{/if}
	</h1>
	{#each postings as posting}
		<div
			class="card card-compact w-full group {posting.seen &&
			selectedSourceId != SelectedSource.Bookmarked
				? 'text-slate-500'
				: ''}"
		>
			<div class="card-body items-left text-left">
				<div class="flex flex-row grow">
					<a href="/posting/{posting.id}" on:click={() => markAsRead([posting.id])}>
						<h2 class="card-title flex grow mb-0">
							{#if sourcesHandler.sourceById(posting.source_id) != undefined}
								<img
									width="16"
									height="16"
									alt=""
									src="https://www.google.com/s2/favicons?sz=16&domain={sourcesHandler.sourceById(
										posting.source_id
									).favicon != null
										? sourcesHandler.sourceById(posting.source_id).favicon
										: sourcesHandler.sourceById(posting.source_id).url}&amp;alt=feed"
								/>
							{/if}
							{posting.title}
						</h2>
					</a>

					<div class="flex flex-row grow justify-end px-4 gap-2">
						<button
							title="Mark As Read"
							class="btn btn-ghost btn-square btn-xs hidden group-hover:block"
							on:click={() => markAsRead([posting.id])}
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
								stroke-width="1.5"
								stroke="currentColor"
								class="w-5 h-5"
							>
								<path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
							</svg>
						</button>
						<button
							title="Bookmark Posting"
							class="btn btn-ghost btn-square btn-xs hidden group-hover:block"
							on:click={() => bookmark(posting.id)}
						>
							{#if posting.bookmarked}
								<svg
									xmlns="http://www.w3.org/2000/svg"
									viewBox="0 0 24 24"
									fill="currentColor"
									class="w-5 h-5"
								>
									<path
										fill-rule="evenodd"
										d="M6.32 2.577a49.255 49.255 0 0 1 11.36 0c1.497.174 2.57 1.46 2.57 2.93V21a.75.75 0 0 1-1.085.67L12 18.089l-7.165 3.583A.75.75 0 0 1 3.75 21V5.507c0-1.47 1.073-2.756 2.57-2.93Z"
										clip-rule="evenodd"
									/>
								</svg>
							{:else}
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
										d="M17.593 3.322c1.1.128 1.907 1.077 1.907 2.185V21L12 17.25 4.5 21V5.507c0-1.108.806-2.057 1.907-2.185a48.507 48.507 0 0 1 11.186 0Z"
									/>
								</svg>
							{/if}
						</button>
					</div>
				</div>
				<a href="/posting/{posting.id}" on:click={() => markAsRead([posting.id])}>
					{#if sourcesHandler.sourceById(posting.source_id) != undefined}
						<p class="pb-1 text-slate-500">
							{sourcesHandler.sourceById(posting.source_id).name} // {new Date(
								posting.created_at
							).toLocaleString()}
						</p>
						{#if getMatchingFilters(posting.content).length > 0}
							<p class="flex grow pb-1 text-orange-400">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									viewBox="0 0 24 24"
									fill="currentColor"
									class="w-4 h-4 mr-1"
								>
									<path
										fill-rule="evenodd"
										d="M9 4.5a.75.75 0 0 1 .721.544l.813 2.846a3.75 3.75 0 0 0 2.576 2.576l2.846.813a.75.75 0 0 1 0 1.442l-2.846.813a3.75 3.75 0 0 0-2.576 2.576l-.813 2.846a.75.75 0 0 1-1.442 0l-.813-2.846a3.75 3.75 0 0 0-2.576-2.576l-2.846-.813a.75.75 0 0 1 0-1.442l2.846-.813A3.75 3.75 0 0 0 7.466 7.89l.813-2.846A.75.75 0 0 1 9 4.5ZM18 1.5a.75.75 0 0 1 .728.568l.258 1.036c.236.94.97 1.674 1.91 1.91l1.036.258a.75.75 0 0 1 0 1.456l-1.036.258c-.94.236-1.674.97-1.91 1.91l-.258 1.036a.75.75 0 0 1-1.456 0l-.258-1.036a2.625 2.625 0 0 0-1.91-1.91l-1.036-.258a.75.75 0 0 1 0-1.456l1.036-.258a2.625 2.625 0 0 0 1.91-1.91l.258-1.036A.75.75 0 0 1 18 1.5ZM16.5 15a.75.75 0 0 1 .712.513l.394 1.183c.15.447.5.799.948.948l1.183.395a.75.75 0 0 1 0 1.422l-1.183.395c-.447.15-.799.5-.948.948l-.395 1.183a.75.75 0 0 1-1.422 0l-.395-1.183a1.5 1.5 0 0 0-.948-.948l-1.183-.395a.75.75 0 0 1 0-1.422l1.183-.395c.447-.15.799-.5.948-.948l.395-1.183A.75.75 0 0 1 16.5 15Z"
										clip-rule="evenodd"
									/>
								</svg>
								{getMatchingFilters(posting.content).join(', ')}
							</p>
						{/if}
					{/if}
					<p class="text-justify">{posting.description}</p>
				</a>
			</div>
		</div>
	{/each}

	{#if postings && postings.filter((p) => !p.seen).length > 0}
		<div class="py-8 flex-none px-4">
			<button
				on:click={() => markAsRead(postings.map((p) => p.id))}
				class="btn btn-active w-full max-w">Mark All As Read</button
			>
		</div>
	{:else if selectedSourceId != 'bookmarked' && postings.length == 0}
		<div class="py-8 flex-none px-4 justify-items-center grid max-w w-full">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="w-20 h-20 text-slate-300"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M2.25 13.5h3.86a2.25 2.25 0 0 1 2.012 1.244l.256.512a2.25 2.25 0 0 0 2.013 1.244h3.218a2.25 2.25 0 0 0 2.013-1.244l.256-.512a2.25 2.25 0 0 1 2.013-1.244h3.859m-19.5.338V18a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18v-4.162c0-.224-.034-.447-.1-.661L19.24 5.338a2.25 2.25 0 0 0-2.15-1.588H6.911a2.25 2.25 0 0 0-2.15 1.588L2.35 13.177a2.25 2.25 0 0 0-.1.661Z"
				/>
			</svg>
			<p class="text-slate-300 mb-10 font-bold">You're all caught up</p>
			<button on:click={viewRead} class="btn btn-active w-1/4">View Read Postings</button>
		</div>
	{/if}
</div>
