<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { Sources, Source } from '../../../lib/types/sources';
	import { NotificationHandler } from '../../../lib/types/notifications';
	import ValidatedInput from '../../../lib/components/ValidatedInput.svelte';

	let notificationHandler = new NotificationHandler();
	let drawerOpen = true;
	export let data: PageData;
	let isNewSource = data.sourceId == 'new';
	let sourcesHandler = new Sources();
	let source;

	if (isNewSource) {
		source = new Source();
	} else {
		source = sourcesHandler.sourceById(data.sourceId);
	}

	let validation = {
		nameValidation: null,
		urlValidation: null
	};

	function closeDrawer(e) {
		goto('/');
	}

	function saveSource() {
		if (source.name.trim() == '') {
			validation.nameValidation = 'Give this source a name';
		}

		if (source.url.trim() == '') {
			validation.urlValidation = 'Set a URL for this source';
		}

		try {
			let urlToValidate = source.url;
			if (!source.url.includes('://')) {
				urlToValidate = 'http://' + source.url;
			}
			new URL(urlToValidate);
		} catch (_) {
			validation.urlValidation = 'Set a valid URL for this source';
		}

		if (validation.nameValidation == null && validation.urlValidation == null) {
			if (isNewSource) {
				sourcesHandler.createSource(source).then((res) => {
					if (!res.isSuccessful) {
						notificationHandler.addError('Could not add source', res.message);
					}

					goto('/');
				});
			} else {
				sourcesHandler.updateSource(source).then((res) => {
					if (!res.isSuccessful) {
						notificationHandler.addError('Could not update source', res.message);
					}

					goto('/');
				});
			}
		}
	}

	function resetCache() {
		sourcesHandler.resetSourceCache(source.id).then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not reset source cache', res.message);
			}
		});
		goto('/');
	}
</script>

<div class="drawer drawer-end">
	<input
		id="new-source-drawer"
		type="checkbox"
		class="drawer-toggle"
		checked={drawerOpen}
		on:click|preventDefault={closeDrawer}
	/>

	<div class="drawer-side">
		<label for="new-source-drawer" aria-label="close sidebar" class="drawer-overlay"></label>

		<div class="w-3/4 min-h-full bg-base-200 text-base-content">
			<nav class="navbar py-4">
				<div class="flex-none">
					<a href="/" class="btn btn-square btn-ghost btn-sm">
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
					</a>
				</div>
				<div class="flex-1"></div>
				<div class="flex-none"></div>
			</nav>

			<div class="px-8">
				<h1 class="text-4xl font-bold py-8">
					{#if isNewSource}
						New Source
					{:else}
						{source.name}
					{/if}
				</h1>

				<ValidatedInput
					name={'Source Name'}
					placeholder={'Super cool company'}
					bind:validation={validation.nameValidation}
					bind:value={source.name}
				/>

				<ValidatedInput
					name={'URL'}
					placeholder={'example.com'}
					bind:validation={validation.urlValidation}
					bind:value={source.url}
				/>

				<div class="py-4">
					<details class="collapse bg-base-200 collapse-arrow border border-base-300">
						<summary class="collapse-title font-medium">Advanced Settings</summary>
						<div class="collapse-content">
							<label class="form-control w-full max-w">
								<div class="label">
									<span class="label-text">Pagination</span>
								</div>
								<input
									type="text"
									placeholder="Link with pagination"
									class="input input-bordered w-full max-w"
									bind:value={source.pagination}
								/>
							</label>

							<label class="form-control w-full max-w">
								<div class="label">
									<span class="label-text">Selector</span>
								</div>
								<input
									type="text"
									placeholder="CSS Path to element with content"
									class="input input-bordered w-full max-w"
									bind:value={source.selector}
								/>
							</label>
						</div>
					</details>
				</div>

				<div class="py-8 flex-none">
					<button class="btn btn-active btn-primary" on:click={saveSource}>Save</button>
					<a href="/" class="btn btn-active">Cancel</a>
				</div>

				{#if !isNewSource}
					<div class="py-4">
						<div tabindex="0" class="collapse collapse-open bg-base-200 border border-red-400">
							<div class="collapse-title font-medium">Danger Zone</div>
							<div class="collapse-content">
								<div class="form-control w-full max-w flex-row flex justify-between">
									<div class="label grow">
										<span class="label-text"
											>Reset the source cache. During the next refresh all current job postings will
											be evaluated.</span
										>
									</div>
									<button
										placeholder="Link with pagination"
										class="btn btn-outline btn-error w-1/5"
										on:click={resetCache}
									>
										Reset Cache
									</button>
								</div>
							</div>
						</div>
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>
