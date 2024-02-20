<script lang="ts">
	import { goto } from '$app/navigation';
	import { Source } from '../../lib/types/sources';
	import { page } from '$app/stores';
	import { Settings, SettingsHandler } from '../../lib/types/settings';
	import { NotificationHandler } from '../../lib/types/notifications';
	import ValidatedInput from '../../lib/components/ValidatedInput.svelte';

	let drawerOpen = true;

	let notificationHandler = new NotificationHandler();
	let settingsHandler = new SettingsHandler();
	let settings = settingsHandler.settings;
	let validation = {
		apiKeyValidation: null
	};

	settingsHandler.subscribe((value) => {
		settings = settingsHandler.settings;
	});

	function closeDrawer(e) {
		goto('/');
	}

	function updateSettings() {
		if (settings.api_key.trim() == '') {
			validation.apiKeyValidation = 'Please provide an API key';
		}

		if (validation.apiKeyValidation == null) {
			settingsHandler.updateSettings(settings).then((res) => {
				if (!res.isSuccessful) {
					notificationHandler.addError('Could not update settings', res.message);
				}

				closeDrawer();
			});
		}
	}
</script>

<div class="drawer drawer-end">
	<input
		id="settings-drawer"
		type="checkbox"
		class="drawer-toggle"
		checked={drawerOpen}
		on:click|preventDefault={closeDrawer}
	/>

	<div class="drawer-side">
		<label for="settings-drawer" aria-label="close sidebar" class="drawer-overlay"></label>

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
				<h1 class="text-4xl font-bold py-8">Settings</h1>

				<ValidatedInput
					label={'API Key'}
					placeholder={'******'}
					masked={true}
					bind:value={settings.api_key}
					bind:validation={validation.apiKeyValidation}
				/>

				<div class="py-8 flex-none">
					<button class="btn btn-active btn-primary" on:click={updateSettings}>Save</button>
					<a href="/" class="btn btn-active">Close</a>
				</div>
			</div>
		</div>
	</div>
</div>
