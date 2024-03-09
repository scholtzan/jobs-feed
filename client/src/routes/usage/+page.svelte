<script lang="ts">
	import { browser } from '$app/environment';
	import { Usages } from '../../lib/types/usage';
	import { Sources } from '../../lib/types/sources';
	import { NotificationHandler } from '../../lib/types/notifications';

	let notificationHandler = new NotificationHandler();
	let usageHandler = new Usages();
	let sourcesHandler = new Sources();
	let drawerOpen = true;
	let usage = {};

	getUsage(7);
	getUsage(14);

	function closeDrawer() {
		if (browser) window.history.back();
	}

	function getUsage(days: number) {
		usageHandler.getUsageCost(days).then((res) => {
			if (res.isSuccessful) {
				res.data.forEach((u) => {
					if (usage[u.source_id] == undefined) {
						usage[u.source_id] = {
							name: u.source_name
						};
					}

					usage[u.source_id][String(days)] = u.cost;
				});
			} else {
				notificationHandler.addError('Could not get usage information', res.message);
			}
		});
	}
</script>

<div class="drawer drawer-end">
	<input
		id="usage-drawer"
		type="checkbox"
		class="drawer-toggle"
		checked={drawerOpen}
		on:click|preventDefault={closeDrawer}
	/>

	<div class="drawer-side">
		<label for="filters-drawer" aria-label="close sidebar" class="drawer-overlay"></label>

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
			<div class="px-8 justify-center">
				<h1 class="text-4xl font-bold py-8">
					Usage
					<a
						title="Go To OpenAI Usage"
						class="btn btn-ghost btn-square"
						href="https://platform.openai.com/usage"
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
								d="M13.5 6H5.25A2.25 2.25 0 0 0 3 8.25v10.5A2.25 2.25 0 0 0 5.25 21h10.5A2.25 2.25 0 0 0 18 18.75V10.5m-10.5 6L21 3m0 0h-5.25M21 3v5.25"
							/>
						</svg>
					</a>
				</h1>

				<div class="stats shadow">
					<div class="stat">
						<div class="stat-figure text-primary">
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
									d="M12 21a9.004 9.004 0 0 0 8.716-6.747M12 21a9.004 9.004 0 0 1-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 0 1 7.843 4.582M12 3a8.997 8.997 0 0 0-7.843 4.582m15.686 0A11.953 11.953 0 0 1 12 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0 1 21 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0 1 12 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 0 1 3 12c0-1.605.42-3.113 1.157-4.418"
								/>
							</svg>
						</div>
						<div class="stat-title">Total Sources</div>
						<div class="stat-value text-primary">{sourcesHandler.sources.length}</div>
					</div>

					<div class="stat">
						<div class="stat-figure text-secondary">
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
									d="M12 6v12m-3-2.818.879.659c1.171.879 3.07.879 4.242 0 1.172-.879 1.172-2.303 0-3.182C13.536 12.219 12.768 12 12 12c-.725 0-1.45-.22-2.003-.659-1.106-.879-1.106-2.303 0-3.182s2.9-.879 4.006 0l.415.33M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
								/>
							</svg>
						</div>
						<div class="stat-title">Total Cost</div>
						<div class="stat-value text-secondary">
							${Object.entries(usage)
								.map((u) => u[1]['7'])
								.reduce((a, b) => a + b, 0)
								.toLocaleString('en-us', { minimumFractionDigits: 2 })}
						</div>
						<div class="stat-desc">Last 7 Days</div>
					</div>

					<div class="stat">
						<div class="stat-figure text-secondary">
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
									d="M12 6v12m-3-2.818.879.659c1.171.879 3.07.879 4.242 0 1.172-.879 1.172-2.303 0-3.182C13.536 12.219 12.768 12 12 12c-.725 0-1.45-.22-2.003-.659-1.106-.879-1.106-2.303 0-3.182s2.9-.879 4.006 0l.415.33M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
								/>
							</svg>
						</div>
						<div class="stat-title">Total Cost</div>
						<div class="stat-value text-secondary">
							${Object.entries(usage)
								.map((u) => u[1]['14'])
								.reduce((a, b) => a + b, 0)
								.toLocaleString('en-us', { minimumFractionDigits: 2 })}
						</div>
						<div class="stat-desc">Last 14 Days</div>
					</div>
				</div>

				<div class="overflow-x-auto py-8 w-3/4">
					<table class="table table-xs">
						<thead>
							<tr>
								<th>Source Name</th>
								<th>Cost Last 7 Days</th>
								<th>Cost Last 14 Days</th>
							</tr>
						</thead>
						<tbody>
							{#each Object.entries(usage) as [_, u]}
								<tr>
									<td>{u.name}</td>
									<td>${u['7']}</td>
									<td>${u['14']}</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			</div>
		</div>
	</div>
</div>
