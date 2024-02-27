<script lang="ts">
	import { goto } from '$app/navigation';
	import { Sources, Source } from '../../../lib/types/sources';
	import { NotificationHandler } from '../../../lib/types/notifications';
	import ValidatedInput from '../../../lib/components/ValidatedInput.svelte';

	let notificationHandler = new NotificationHandler();
	let drawerOpen = true;
	export let data: PageData;
	let isNewSource = data.sourceId == 'new';
	let sourcesHandler = new Sources();
	let source = new Source();

	if (isNewSource) {
		source = new Source();
	} else {
		sourcesHandler.refresh().then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not get sources.');
				source = new Source();
			} else {
				source = sourcesHandler.sourceById(data.sourceId);
				if (source == undefined) {
					source = new Source();
					notificationHandler.addError('No such source');
				}
			}
		});
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

	function openSource() {
		let s = sourcesHandler.sourceById(source.id);

		if (s != undefined) {
			window.open(s.url, '_blank');
		} else {
			notificationHandler.addError('Could not open link to source. Source does not exist');
		}
	}

	function deleteSource() {
		sourcesHandler.deleteSource(source.id).then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not delete source', res.message);
			}
		});
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
						<button class="btn btn-ghost btn-square" on:click={openSource}>
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
						</button>
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
					<details class="collapse bg-base-200 collapse-arrow border border-base-400">
						<summary class="collapse-title font-medium">Advanced Settings</summary>
						<div class="collapse-content">
							<label class="form-control w-full max-w">
								<div class="label">
									<span class="label-text items-center"
										>Pagination
										<div
											class="tooltip tooltip-right"
											data-tip="CSS Selector for the pagination link"
										>
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
													d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 5.25h.008v.008H12v-.008Z"
												/>
											</svg>
										</div>
									</span>
								</div>
								<input
									type="text"
									placeholder="Link with pagination"
									class="input input-bordered w-full max-w"
									bind:value={source.pagination}
								/>
							</label>

							<label class="form-control w-full max-w">
								<div class="label items-center">
									<span class="label-text"
										>Selector
										<div
											class="tooltip tooltip-right"
											data-tip="CSS Selector for the page element that contains postings to reduce cost"
										>
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
													d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 5.25h.008v.008H12v-.008Z"
												/>
											</svg>
										</div>
									</span>
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
								<div class="form-control w-full max-w flex-row flex justify-between py-2">
									<div class="label grow">
										<span class="label-text"
											>Reset the source cache. During the next refresh all current job postings will
											be evaluated.</span
										>
									</div>
									<button class="btn btn-outline btn-error w-1/5" on:click={resetCache}>
										Reset Cache
									</button>
								</div>
								<div class="form-control w-full max-w flex-row flex justify-between py-2">
									<div class="label grow">
										<span class="label-text">Delete the source and all associated data.</span>
									</div>
									<button
										class="btn btn-outline btn-error w-1/5"
										onclick="confirm_remove_modal.showModal()"
									>
										Remove
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

<dialog id="confirm_remove_modal" class="modal">
	<div class="modal-box">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button>
		</form>
		<h3 class="font-bold text-lg">Remove '{source.name}'?</h3>
		<p class="py-4">Please confirm to remove the source '{source.name}'.</p>

		<form method="dialog" class="modal-backdrop">
			<div class="py-8 flex-none">
				<button class="btn btn-active">Cancel</button>
				<button class="btn btn-active btn-error" on:click={deleteSource}>Yes, Remove</button>
			</div>
		</form>
	</div>
</dialog>
