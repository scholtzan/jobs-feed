<script lang="ts">
	import { goto } from '$app/navigation';
	import { Postings } from '../types/postings';
	import { Sources, SelectedSource } from '../types/sources';
	import { NotificationHandler } from '../types/notifications';

	let notificationHandler = new NotificationHandler();
	let sourcesHandler = new Sources();
	let postingsHandler = new Postings();
	let sourceSelected = sourcesHandler.selectedSource;
	let sources = sourcesHandler.sources;
	let postings = postingsHandler.postings;

	postingsHandler.subscribe((_) => {
		getPostingsForSelectedSource();
	});

	sourcesHandler.subscribeSelectedSource((value) => {
		sourceSelected = sourcesHandler.selectedSource;
		getPostingsForSelectedSource();
	});

	function getPostingsForSelectedSource() {
		if (sourceSelected == SelectedSource.All) {
			postings = postingsHandler.postings;
		} else if (sourceSelected == SelectedSource.Today) {
			postings = postingsHandler.getTodaysPostings();
		} else if (sourceSelected == SelectedSource.Bookmarked) {
			postingsHandler.getBookmarked().then((res) => {
				if (!res.isSuccessful) {
					notificationHandler.addError('Could not get bookmarked postings', res.message);
				} else {
					postings = res.data;
				}
			});
		} else {
			let postingsBySource = postingsHandler.postingsBySource();
			if (sourceSelected in postingsBySource) {
				postings = postingsBySource[sourceSelected];
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
		postingsHandler.getReadPostingsOfSource(sourceSelected).then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not fetch read postings', res.message);
			} else {
				postings = res.data;
			}
		});
	}
</script>

<div class="w-2/3">
	<div class="flex flex-row grow justify-end px-4">
		<button class="btn btn-ghost btn-square" on:click={() => markAsRead(postings.map((p) => p.id))}>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="w-9 h-9"
			>
				<path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
			</svg>
		</button>
	</div>
	<h1 class="flex grow text-4xl font-bold py-8 px-4">
		{#if sourceSelected == SelectedSource.All}
			All Job Postings
		{:else if sourceSelected == SelectedSource.Today}
			Today's Job Postings
		{:else if sourceSelected == SelectedSource.Bookmarked}
			Bookmarked Postings
		{:else}
			{sourcesHandler.sourceById(sourceSelected) == undefined
				? goto('/')
				: sourcesHandler.sourceById(sourceSelected).name}
		{/if}
	</h1>
	{#each postings as posting}
		<div
			class="card card-compact w-full group {posting.seen &&
			sourceSelected != SelectedSource.Bookmarked
				? 'text-slate-500'
				: ''}"
		>
			<div class="card-body items-left text-left">
				<div class="flex flex-row grow">
					<a href="/posting/{posting.id}" on:click={() => markAsRead([posting.id])}>
						<h2 class="card-title flex grow mb-0">
							{posting.title}
						</h2>
					</a>

					<div class="flex flex-row grow justify-end px-4 gap-2">
						<button
							class="btn btn-ghost btn-square btn-xs hidden group-hover:block"
							on:click={() => markAsRead([posting.id])}
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
								stroke-width="1.5"
								stroke="currentColor"
								class="w-6 h-6"
							>
								<path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
							</svg>
						</button>
						<button
							class="btn btn-ghost btn-square btn-xs hidden group-hover:block"
							on:click={() => bookmark(posting.id)}
						>
							{#if posting.bookmarked}
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
										d="M17.593 3.322c1.1.128 1.907 1.077 1.907 2.185V21L12 17.25 4.5 21V5.507c0-1.108.806-2.057 1.907-2.185a48.507 48.507 0 0 1 11.186 0Z"
									/>
								</svg>
							{/if}
						</button>
					</div>
				</div>
				<a href="/posting/{posting.id}" on:click={() => markAsRead([posting.id])}>
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
	{:else if sourceSelected != 'bookmarked' && postings.length == 0}
		<div class="py-8 flex-none px-4 justify-items-center grid max-w w-full">
			<button on:click={viewRead} class="btn btn-active w-1/4">View Read Postings</button>
		</div>
	{/if}
</div>
