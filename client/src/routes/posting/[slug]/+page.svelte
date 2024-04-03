<script lang="ts">
	import { browser } from '$app/environment';
	import type { PageData } from './$types';
	import { Postings, Posting } from '../../../lib/types/postings';
	import { NotificationHandler } from '../../../lib/types/notifications';
	import SvelteMarkdown from 'svelte-markdown';
	import { Sources } from '../../../lib/types/sources';

	let notificationHandler = new NotificationHandler();
	let postingsHandler = new Postings();
	let drawerOpen = true;
	let posting = new Posting();
	let sourcesHandler = new Sources();
	export let data: PageData;
	let postingId = data.postingId;
	postingsHandler.postingById(postingId).then((res) => {
		if (!res.isSuccessful) {
			notificationHandler.addError('Could not fetch posting', res.message);
		} else {
			posting = res.data;
		}
	});

	postingsHandler.subscribe((_) => {
		postingsHandler.postingById(postingId).then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not fetch posting', res.message);
			} else {
				posting = res.data;
			}
		});
	});

	function bookmark() {
		postingsHandler.bookmarkPosting(postingId).then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not bookmark posting', res.message);
			}
		});
	}

	function like() {
		postingsHandler.likePosting(postingId).then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not like posting', res.message);
			}
		});
	}

	function dislike() {
		postingsHandler.dislikePosting(postingId).then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not dislike posting', res.message);
			}
		});
	}

	function closeDrawer() {
		if (browser) window.history.back();
	}

	function getContent() {
		let content = posting.content;
		let startIndex = content.indexOf(posting.title);

		if (startIndex != -1) {
			content = content.substring(startIndex);
		}

		let endIndex = content.indexOf('Apply Now');

		if (endIndex != -1) {
			content = content.substring(0, endIndex);
		}

		endIndex = content.indexOf('©');

		if (endIndex != -1) {
			content = content.substring(0, endIndex);
		}

		return content;
	}
</script>

<div class="drawer drawer-end">
	<input
		id="posting-drawer"
		type="checkbox"
		class="drawer-toggle"
		checked={drawerOpen}
		on:click|preventDefault={closeDrawer}
	/>

	<div class="drawer-side">
		<label for="posting-drawer" aria-label="close sidebar" class="drawer-overlay"></label>

		<div class="lg:w-3/4 w-[95%] min-h-full bg-base-200 text-base-content">
			<nav class="navbar py-4">
				<div class="flex-none">
					<button class="btn btn-square btn-ghost" title="Close" on:click={closeDrawer}>
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
								d="m18.75 4.5-7.5 7.5 7.5 7.5m-6-15L5.25 12l7.5 7.5"
							/>
						</svg>
					</button>
				</div>
				<div class="flex-1"></div>
				<div class="flex-none px-6">
					<button class="btn btn-ghost btn-square px-2" on:click={bookmark}>
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
					<button class="btn btn-ghost btn-square px-2" on:click={like}>
						{#if posting.is_match}
							<svg
								xmlns="http://www.w3.org/2000/svg"
								viewBox="0 0 24 24"
								fill="currentColor"
								class="w-6 h-6"
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
								class="w-6 h-6"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									d="M6.633 10.25c.806 0 1.533-.446 2.031-1.08a9.041 9.041 0 0 1 2.861-2.4c.723-.384 1.35-.956 1.653-1.715a4.498 4.498 0 0 0 .322-1.672V2.75a.75.75 0 0 1 .75-.75 2.25 2.25 0 0 1 2.25 2.25c0 1.152-.26 2.243-.723 3.218-.266.558.107 1.282.725 1.282m0 0h3.126c1.026 0 1.945.694 2.054 1.715.045.422.068.85.068 1.285a11.95 11.95 0 0 1-2.649 7.521c-.388.482-.987.729-1.605.729H13.48c-.483 0-.964-.078-1.423-.23l-3.114-1.04a4.501 4.501 0 0 0-1.423-.23H5.904m10.598-9.75H14.25M5.904 18.5c.083.205.173.405.27.602.197.4-.078.898-.523.898h-.908c-.889 0-1.713-.518-1.972-1.368a12 12 0 0 1-.521-3.507c0-1.553.295-3.036.831-4.398C3.387 9.953 4.167 9.5 5 9.5h1.053c.472 0 .745.556.5.96a8.958 8.958 0 0 0-1.302 4.665c0 1.194.232 2.333.654 3.375Z"
								/>
							</svg>
						{/if}
					</button>
					<button class="btn btn-ghost btn-square px-2" on:click={dislike}>
						{#if posting.is_match == false}
							<svg
								xmlns="http://www.w3.org/2000/svg"
								viewBox="0 0 24 24"
								fill="currentColor"
								class="w-6 h-6"
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
								class="w-6 h-6"
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
			</nav>

			<div class="px-8">
				<h1 class="flex grow lg:text-4xl text-2xl font-bold py-4">
					{#if sourcesHandler.sourceById(posting.source_id) != undefined}
						<img
							width="48"
							height="48"
							class="mr-2 block lg:w-10 lg:h-10 h-8 w-8"
							alt=""
							src="https://www.google.com/s2/favicons?sz=48&domain={sourcesHandler.sourceById(
								posting.source_id
							).favicon != null
								? sourcesHandler.sourceById(posting.source_id).favicon
								: sourcesHandler.sourceById(posting.source_id).url}&amp;alt=feed"
						/>
					{/if}
					{posting.title}
				</h1>

				<p class="w-full max-w">
					{#if sourcesHandler.sourceById(posting.source_id) != undefined}
						<p class="pb-2 text-slate-500">
							{sourcesHandler.sourceById(posting.source_id).name} // {new Date(
								posting.created_at
							).toLocaleString()}
						</p>
					{/if}

					{#if posting.content}
						<SvelteMarkdown source={getContent()} />
					{:else if posting.description}
						{posting.description}
					{:else}
						Could not extract posting contents.
					{/if}
				</p>

				<div class="py-8 flex-none">
					{#if sourcesHandler.sourceById(posting.source_id) != undefined}
						<a
							href={posting.url ? posting.url : sourcesHandler.sourceById(posting.source_id).url}
							target="_blank"
							class="btn btn-active w-full max-w">Go To Posting</a
						>
					{/if}
				</div>
			</div>
		</div>
	</div>
</div>
