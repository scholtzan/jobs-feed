<script lang="ts">
	import { goto } from '$app/navigation';
	import { Posting, PostingsHandler } from '../types/postings';
	import { SelectedSource, Source, SourcesHandler } from '../types/sources';
	import { NotificationHandler } from '../types/notifications';
	import { FiltersHandler } from '../types/filters';
	import { Suggestion, SuggestionsHandler } from '../types/suggestions';

	let notificationHandler = new NotificationHandler();
	let sourcesHandler = new SourcesHandler();
	let postingsHandler = new PostingsHandler();
	let filtersHandler = new FiltersHandler();
	let suggestionsHandler = new SuggestionsHandler();

	let selectedSourceId: string | number | null = sourcesHandler.selectedSource;
	let sources = sourcesHandler.sources;
	let postings = postingsHandler.postings;
	let source: Source | undefined;
	let suggestions: Suggestion[] = [];
	let isRefreshingSuggestions = false;

	// refresh postings data if any data changes
	getPostingsForSelectedSource();

	postingsHandler.subscribe((_) => {
		getPostingsForSelectedSource();
	});

	sourcesHandler.subscribeSelectedSource((value) => {
		selectedSourceId = sourcesHandler.selectedSource;
		postingsHandler.refresh();
		getPostingsForSelectedSource();
	});

	filtersHandler.subscribe((_) => {
		getPostingsForSelectedSource();
	});

	/**
	 * Fetch postings, based on which source is currently selected.
	 */
	function getPostingsForSelectedSource(): void {
		suggestions = [];
		if (selectedSourceId == SelectedSource.All) {
			postings = postingsHandler.postings;
		} else if (selectedSourceId == SelectedSource.Today) {
			postings = postingsHandler.getTodaysPostings();
		} else if (selectedSourceId == SelectedSource.Bookmarked) {
			postingsHandler.getBookmarked().then((res) => {
				if (!res.isSuccessful) {
					notificationHandler.addError('Could not get bookmarked postings', res.message);
				} else {
					postings = res.data as Posting[];
				}
			});
		} else {
			source = sourcesHandler.sourceById(selectedSourceId);
			let postingsBySource = postingsHandler.postingsBySource();
			if (selectedSourceId != null && selectedSourceId in postingsBySource) {
				postings = postingsBySource[selectedSourceId as number].filter((p: Posting) => !p.seen);
			} else {
				postings = [];
			}
			suggestionsHandler.getSourceSuggestions(selectedSourceId).then((res) => {
				if (!res.isSuccessful) {
					notificationHandler.addError('Could not get source suggestions', res.message);
				} else {
					suggestions = res.data as Suggestion[];
				}
			});
		}
	}

	sourcesHandler.subscribe((_) => {
		sources = sourcesHandler.sources;
	});

	/**
	 * Mark a set of postings as seen.
	 * @param ids set of postings IDs
	 */
	function markAsRead(ids: number[]) {
		postingsHandler.markAsRead(ids).then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not mark posting as read', res.message);
			}
		});
	}

	/**
	 * Bookmark a specific posting.
	 * @param id ID of posting to bookmark
	 */
	function bookmark(id: number | null) {
		postingsHandler.bookmarkPosting(id).then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not bookmark posting', res.message);
			}
		});
	}

	/**
	 * Mark a specific posting as liked
	 * @param id ID of posting to like
	 */
	function like(id: number | null) {
		postingsHandler.likePosting(id).then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not like posting', res.message);
			}
		});
	}

	/**
	 * Mark a specific posting as disliked
	 * @param id ID of posting to dislike
	 */
	function dislike(id: number | null) {
		postingsHandler.dislikePosting(id).then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not dislike posting', res.message);
			}
		});
	}

	/**
	 * Show only postings that were seen before.
	 */
	function viewRead() {
		postingsHandler.getReadPostingsOfSource(selectedSourceId).then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not fetch read postings', res.message);
			} else {
				postings = res.data as Posting[];
			}
		});
	}

	/**
	 * Open URL to a specific source page.
	 * @param sourceId ID of source to open URL for
	 */
	function openSource(sourceId: number | string | null) {
		let source = sourcesHandler.sourceById(sourceId);

		if (source != undefined) {
			window.open(source.url, '_blank');
		} else {
			notificationHandler.addError('Could not open link to source. Source does not exist');
		}
	}

	/**
	 * Filters that found matches in the posting content.
	 * @param content posting content to search for matching filter values
	 */
	function getMatchingFilters(content: string) {
		let matchingFilters: string[] = [];

		if (content != null) {
			filtersHandler.filters.forEach((filter) => {
				filter.value.split(',').forEach((v) => {
					v = v.trim();
					let regex = new RegExp(`\\b${v}\\b`, 'gi');
					if (content.match(regex)) {
						matchingFilters.push(v);
					}
				});
			});
		}

		return matchingFilters;
	}

	/**
	 * Refresh suggestions for a specific source.
	 */
	function refreshSuggestions() {
		isRefreshingSuggestions = true;
		suggestionsHandler.refreshSourceSuggestions(selectedSourceId).then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not refresh suggestions', res.message);
			} else {
				suggestions = res.data as Suggestion[];
			}

			isRefreshingSuggestions = false;
		});
	}
</script>

<div class="justify-center w-full flex mt-8">
	<div class="lg:max-w-[50em] lg:min-w-[35em] w-[95%]">
		<!-- Buttons next to selected source header -->
		<div class="flex flex-row grow justify-end px-4 gap-x-2">
			<!-- Button to mark all posts as read -->
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
				<!-- Edit source button -->
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

				<!-- Button to open source URL -->
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

		<!-- Headers based on selected sources -->
		<h1 class="flex grow lg:text-4xl text-2xl font-bold py-4 px-4">
			{#if selectedSourceId == SelectedSource.All}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke-width="1.5"
					stroke="currentColor"
					class="w-11 h-11 mr-2 max-md:w-8 max-md:h-8"
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
					class="w-11 h-11 mr-2 max-md:w-8 max-md:h-8"
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
					class="w-11 h-11 mr-2 max-md:w-8 max-md:h-8"
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
						class="mr-2 max-md:w-8 max-md:h-8"
						width="40"
						height="40"
						alt=""
						src="https://www.google.com/s2/favicons?sz=48&domain={source.favicon != null
							? source.favicon
							: source.url}&amp;alt=feed"
					/>
				{/if}

				{source == undefined ? goto('/') : source.name}
			{/if}
		</h1>

		<div class="flex justify-center w-full">
			<div class="w-full">
				{#if source != undefined && source.unreachable}
					<!-- Error text if source could not be reached to fetch postings -->
					<div role="alert" class="alert alert-error">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="stroke-current shrink-0 h-6 w-6"
							fill="none"
							viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
							/></svg
						>
						<span>Source unreachable.</span>
					</div>
				{/if}

				{#each postings as posting}
					<div
						class="card card-compact w-full group {posting.seen &&
						selectedSourceId != SelectedSource.Bookmarked
							? 'text-slate-500'
							: ''}"
					>
						<div class="card-body items-left text-left">
							<!-- Posting header -->
							<div class="flex flex-row grow">
								<a href="/posting/{posting.id}" on:click={() => markAsRead([posting.id])}>
									<h2
										class="card-title flex grow lg:text-xl text-base"
										style="margin-bottom: -4px;"
									>
										{#if sourcesHandler.sourceById(posting.source_id) != undefined}
											<!-- Source favicon -->
											<img
												width="16"
												height="16"
												alt=""
												src="https://www.google.com/s2/favicons?sz=48&domain={sourcesHandler.sourceById(
													posting.source_id
												).favicon != null
													? sourcesHandler.sourceById(posting.source_id).favicon
													: sourcesHandler.sourceById(posting.source_id).url}&amp;alt=feed"
											/>
										{/if}

										{posting.title}
									</h2>
								</a>

								<!-- Buttons next to posting title -->
								<div class="flex flex-row grow justify-end px-4 gap-2 w-48 max-md:hidden">
									<!-- Mark as read button -->
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
											<path
												stroke-linecap="round"
												stroke-linejoin="round"
												d="m4.5 12.75 6 6 9-13.5"
											/>
										</svg>
									</button>

									<!-- Bookmark button -->
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

									<!-- Like button -->
									<button
										title="More like this"
										class="btn btn-ghost btn-square btn-xs hidden group-hover:block"
										on:click={() => like(posting.id)}
									>
										{#if posting.is_match}
											<svg
												xmlns="http://www.w3.org/2000/svg"
												viewBox="0 0 24 24"
												fill="currentColor"
												class="w-5 h-5"
											>
												<path
													d="M7.493 18.5c-.425 0-.82-.236-.975-.632A7.48 7.48 0 0 1 6 15.125c0-1.75.599-3.358 1.602-4.634.151-.192.373-.309.6-.397.473-.183.89-.514 1.212-.924a9.042 9.042 0 0 1 2.861-2.4c.723-.384 1.35-.956 1.653-1.715a4.498 4.498 0 0 0 .322-1.672V2.75A.75.75 0 0 1 15 2a2.25 2.25 0 0 1 2.25 2.25c0 1.152-.26 2.243-.723 3.218-.266.558.107 1.282.725 1.282h3.126c1.026 0 1.945.694 2.054 1.715.045.422.068.85.068 1.285a11.95 11.95 0 0 1-2.649 7.521c-.388.482-.987.729-1.605.729H14.23c-.483 0-.964-.078-1.423-.23l-3.114-1.04a4.501 4.501 0 0 0-1.423-.23h-.777ZM2.331 10.727a11.969 11.969 0 0 0-.831 4.398 12 12 0 0 0 .52 3.507C2.28 19.482 3.105 20 3.994 20H4.9c.445 0 .72-.498.523-.898a8.963 8.963 0 0 1-.924-3.977c0-1.708.476-3.305 1.302-4.666.245-.403-.028-.959-.5-.959H4.25c-.832 0-1.612.453-1.918 1.227Z"
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
													d="M6.633 10.25c.806 0 1.533-.446 2.031-1.08a9.041 9.041 0 0 1 2.861-2.4c.723-.384 1.35-.956 1.653-1.715a4.498 4.498 0 0 0 .322-1.672V2.75a.75.75 0 0 1 .75-.75 2.25 2.25 0 0 1 2.25 2.25c0 1.152-.26 2.243-.723 3.218-.266.558.107 1.282.725 1.282m0 0h3.126c1.026 0 1.945.694 2.054 1.715.045.422.068.85.068 1.285a11.95 11.95 0 0 1-2.649 7.521c-.388.482-.987.729-1.605.729H13.48c-.483 0-.964-.078-1.423-.23l-3.114-1.04a4.501 4.501 0 0 0-1.423-.23H5.904m10.598-9.75H14.25M5.904 18.5c.083.205.173.405.27.602.197.4-.078.898-.523.898h-.908c-.889 0-1.713-.518-1.972-1.368a12 12 0 0 1-.521-3.507c0-1.553.295-3.036.831-4.398C3.387 9.953 4.167 9.5 5 9.5h1.053c.472 0 .745.556.5.96a8.958 8.958 0 0 0-1.302 4.665c0 1.194.232 2.333.654 3.375Z"
												/>
											</svg>
										{/if}
									</button>

									<!-- Dislike button -->
									<button
										title="Less like this"
										class="btn btn-ghost btn-square btn-xs hidden group-hover:block"
										on:click={() => dislike(posting.id)}
									>
										{#if posting.is_match == false}
											<svg
												xmlns="http://www.w3.org/2000/svg"
												viewBox="0 0 24 24"
												fill="currentColor"
												class="w-5 h-5"
											>
												<path
													d="M15.73 5.5h1.035A7.465 7.465 0 0 1 18 9.625a7.465 7.465 0 0 1-1.235 4.125h-.148c-.806 0-1.534.446-2.031 1.08a9.04 9.04 0 0 1-2.861 2.4c-.723.384-1.35.956-1.653 1.715a4.499 4.499 0 0 0-.322 1.672v.633A.75.75 0 0 1 9 22a2.25 2.25 0 0 1-2.25-2.25c0-1.152.26-2.243.723-3.218.266-.558-.107-1.282-.725-1.282H3.622c-1.026 0-1.945-.694-2.054-1.715A12.137 12.137 0 0 1 1.5 12.25c0-2.848.992-5.464 2.649-7.521C4.537 4.247 5.136 4 5.754 4H9.77a4.5 4.5 0 0 1 1.423.23l3.114 1.04a4.5 4.5 0 0 0 1.423.23ZM21.669 14.023c.536-1.362.831-2.845.831-4.398 0-1.22-.182-2.398-.52-3.507-.26-.85-1.084-1.368-1.973-1.368H19.1c-.445 0-.72.498-.523.898.591 1.2.924 2.55.924 3.977a8.958 8.958 0 0 1-1.302 4.666c-.245.403.028.959.5.959h1.053c.832 0 1.612-.453 1.918-1.227Z"
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
													d="M7.498 15.25H4.372c-1.026 0-1.945-.694-2.054-1.715a12.137 12.137 0 0 1-.068-1.285c0-2.848.992-5.464 2.649-7.521C5.287 4.247 5.886 4 6.504 4h4.016a4.5 4.5 0 0 1 1.423.23l3.114 1.04a4.5 4.5 0 0 0 1.423.23h1.294M7.498 15.25c.618 0 .991.724.725 1.282A7.471 7.471 0 0 0 7.5 19.75 2.25 2.25 0 0 0 9.75 22a.75.75 0 0 0 .75-.75v-.633c0-.573.11-1.14.322-1.672.304-.76.93-1.33 1.653-1.715a9.04 9.04 0 0 0 2.86-2.4c.498-.634 1.226-1.08 2.032-1.08h.384m-10.253 1.5H9.7m8.075-9.75c.01.05.027.1.05.148.593 1.2.925 2.55.925 3.977 0 1.487-.36 2.89-.999 4.125m.023-8.25c-.076-.365.183-.75.575-.75h.908c.889 0 1.713.518 1.972 1.368.339 1.11.521 2.287.521 3.507 0 1.553-.295 3.036-.831 4.398-.306.774-1.086 1.227-1.918 1.227h-1.053c-.472 0-.745-.556-.5-.96a8.95 8.95 0 0 0 .303-.54"
												/>
											</svg>
										{/if}
									</button>
								</div>
							</div>

							<!-- Posting sub header -->
							<a href="/posting/{posting.id}" on:click={() => markAsRead([posting.id])}>
								{#if sourcesHandler.sourceById(posting.source_id) != undefined}
									<p class="pb-1 text-slate-500">
										<!-- Created timestamp and source name -->
										{sourcesHandler.sourceById(posting.source_id).name} // {new Date(
											posting.created_at
										).toLocaleString()}

										<!-- Indicate whether the posting is a good match -->
										{#if posting.match_similarity != null && posting.match_similarity > 0.7}
											<span class="inline text-orange-400"> • Good Match </span>
										{/if}
									</p>

									<!-- Show filter values that matched the posting content -->
									{#if getMatchingFilters(posting.content + posting.title).length > 0}
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
											{getMatchingFilters(posting.content + posting.title).join(', ')}
										</p>
									{/if}
								{/if}
							</a>
						</div>
					</div>
				{/each}

				{#if postings && postings.filter((p) => !p.seen).length > 0}
					<!-- Bottom button to mark postings as read -->
					<div class="py-8 flex-none px-4">
						<button
							on:click={() => markAsRead(postings.map((p) => p.id))}
							class="btn btn-active w-full max-w">Mark All As Read</button
						>
					</div>
				{:else if selectedSourceId != 'bookmarked' && postings.length == 0}
					<!-- Bottom button to view read postings -->
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
						<button on:click={viewRead} class="btn btn-active w-1/4 min-w-[15em]"
							>View Read Postings</button
						>
					</div>
				{/if}
			</div>

			<!-- List of similar sources -->
			{#if ![SelectedSource.All, SelectedSource.Bookmarked, SelectedSource.Today].includes(selectedSourceId)}
				<div class="w-64 ml-6 md:block hidden">
					<div class="menu-title flex grow">
						Similar Companies

						<button
							class="btn btn-xs btn-ghost btn-circle {isRefreshingSuggestions
								? 'btn-disabled'
								: ''}"
							title="Refresh suggestions"
							on:click={refreshSuggestions}
						>
							{#if isRefreshingSuggestions}
								<span class="loading loading-xs loading-spinner"></span>
							{:else}
								<svg
									xmlns="http://www.w3.org/2000/svg"
									viewBox="0 0 24 24"
									fill="currentColor"
									class="w-3 h-3"
								>
									<path
										fill-rule="evenodd"
										d="M4.755 10.059a7.5 7.5 0 0 1 12.548-3.364l1.903 1.903h-3.183a.75.75 0 1 0 0 1.5h4.992a.75.75 0 0 0 .75-.75V4.356a.75.75 0 0 0-1.5 0v3.18l-1.9-1.9A9 9 0 0 0 3.306 9.67a.75.75 0 1 0 1.45.388Zm15.408 3.352a.75.75 0 0 0-.919.53 7.5 7.5 0 0 1-12.548 3.364l-1.902-1.903h3.183a.75.75 0 0 0 0-1.5H2.984a.75.75 0 0 0-.75.75v4.992a.75.75 0 0 0 1.5 0v-3.18l1.9 1.9a9 9 0 0 0 15.059-4.035.75.75 0 0 0-.53-.918Z"
										clip-rule="evenodd"
									/>
								</svg>
							{/if}
						</button>
					</div>
					{#each suggestions as suggestion}
						<div class="text-xs menu-title">
							<a class="link link-hover" target="_blank" href={suggestion.url}>
								<img
									class="inline-block mr-1"
									alt=""
									height="16"
									width="16"
									src="https://www.google.com/s2/favicons?sz=48&domain={suggestion.url}&amp;alt=feed"
								/>{suggestion.name}</a
							>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	</div>
</div>
