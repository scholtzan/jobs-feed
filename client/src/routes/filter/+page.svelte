<script lang="ts">
	import { browser } from '$app/environment';
	import { FiltersHandler, Filter } from '../../lib/types/filters';
	import { NotificationHandler } from '../../lib/types/notifications';

	let notificationHandler = new NotificationHandler();
	let filtersHandler = new FiltersHandler();

	// some suggestions for criteria to filter postings on
	const filterSuggestions = ['Skills', 'Location', 'Job Title'];
	// get stored filters
	let filters = filtersHandler.filters;
	// whether the filters drawer dialog is open or closed
	let drawerOpen = true;

	// get filter data if it has changed
	filtersHandler.subscribe((_value) => {
		filters = filtersHandler.filters;
	});

	/**
	 * Close the filters drawer dialog.
	 */
	function closeDrawer() {
		if (browser) window.history.back();
	}

	/**
	 * Update filters.
	 */
	function updateFilters() {
		filtersHandler.updateFilters(filters).then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not update filters', res.message);
			}

			if (browser) window.history.back();
		});
	}

	/**
	 * Delete the passed filter
	 * @param filter Filter
	 */
	function removeFilter(filter: Filter) {
		filters = filters.filter((f) => f != filter);
	}

	/**
	 * Add a new filter.
	 */
	function addFilter() {
		let newFilter = new Filter();
		filters = [...filters, newFilter];
	}
</script>

<div class="drawer drawer-end">
	<!-- Checkbox for keeping track of whether filter drawer dialog is open -->
	<input
		id="filters-drawer"
		type="checkbox"
		class="drawer-toggle"
		checked={drawerOpen}
		on:click|preventDefault={closeDrawer}
	/>

	<div class="drawer-side">
		<!-- Background overlay -->
		<label for="filters-drawer" aria-label="close sidebar" class="drawer-overlay"></label>

		<div class="lg:w-3/4 w-[95%] min-h-full bg-base-200 text-base-content">
			<nav class="navbar py-4">
				<!-- Close button -->
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
				<div class="flex-none"></div>
			</nav>

			<form class="px-8" id="filters-form" on:submit|preventDefault={updateFilters}>
				<!-- Header -->
				<h1 class="lg:text-4xl text-2xl font-bold py-8">Filters</h1>

				<!-- Create a separate form input for each existing filter -->
				{#each filters as filter}
					<div class="md:flex items-end gap-2 py-2">
						<label class="form-control md:w-1/3 max-w w-full">
							<!-- Filter name input -->
							<div class="label">
								<span class="label-text">Filter Name</span>
							</div>

							<div class="dropdown">
								<!-- Filter value input -->
								<input
									type="text"
									placeholder="Filter name"
									tabIndex="0"
									class="input input-bordered w-full max-w"
									bind:value={filter.name}
								/>

								<!-- Dropdown with filter suggestions -->
								{#if !filterSuggestions.every((s) => filters.find((f) => f.name == s))}
									<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
									<ul
										tabindex="0"
										class="dropdown-content dropdown-open menu p-2 shadow bg-base-100 rounded-box w-full z-40"
									>
										{#each filterSuggestions as filterSuggestion}
											{#if !filters.find((f) => f.name == filterSuggestion)}
												<li>
													<!-- svelte-ignore a11y-no-static-element-interactions -->
													<!-- svelte-ignore a11y-click-events-have-key-events -->
													<span on:click={() => (filter.name = filterSuggestion)}
														>{filterSuggestion}</span
													>
												</li>
											{/if}
										{/each}
									</ul>
								{/if}
							</div>
						</label>

						<!-- Filter value input -->
						<label class="form-control md:w-3/5 max-w w-full">
							<div class="label">
								<span class="label-text">
									Filter Value
									<div
										class="tooltip tooltip-right"
										data-tip="Separate multiple filter values by comma (value 1, value 2, ...)"
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
								placeholder="Filter value 1, Filter value 2"
								class="input input-bordered w-full max-w"
								bind:value={filter.value}
							/>
						</label>

						<!-- Button to delete filter -->
						<button
							title="Remove Filter"
							class="btn btn-active md:btn-square max-sm:mt-4"
							on:click|preventDefault={() => removeFilter(filter)}
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
								stroke-width="1.5"
								stroke="currentColor"
								class="w-6 h-6 max-sm:hidden"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"
								/>
							</svg>
							<span class="max-sm:block hidden">Delete Filter</span>
						</button>
					</div>
				{/each}

				<!-- Button to add new filter -->
				<button title="Add Filter" class="btn btn-active mt-2" on:click|preventDefault={addFilter}>
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
							d="M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
						/>
					</svg>
					Add Filter
				</button>

				<!-- Save and cancel button -->
				<div class="py-8 flex-none">
					<button class="btn btn-active btn-primary" form="filters-form">Save</button>
					<a href="/" class="btn btn-active">Cancel</a>
				</div>
			</form>
		</div>
	</div>
</div>
