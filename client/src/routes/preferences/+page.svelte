<script lang="ts">
	import { browser } from '$app/environment';
	import { SettingsHandler } from '../../lib/types/settings';
	import { NotificationHandler } from '../../lib/types/notifications';
	import ValidatedInput from '../../lib/components/ValidatedInput.svelte';

	let notificationHandler = new NotificationHandler();
	let settingsHandler = new SettingsHandler();

	// whether the settings drawer dialog is open
	let drawerOpen = true;
	// fetch settings
	let settings = settingsHandler.settings;
	// get OpenAI models
	let models: string[] = [];
	getModels();
	// validation results for certain form inputs
	let validation: { apiKeyValidation: null | string } = {
		apiKeyValidation: null
	};

	// get most recent settings whenever data changes
	settingsHandler.subscribe((_value) => {
		settings = settingsHandler.settings;
	});

	/**
	 * Close the settings drawer
	 * @param _e close event
	 */
	function closeDrawer(_e: any) {
		if (browser) window.history.back();
	}

	/**
	 * Send updated settings to the server.
	 */
	function updateSettings() {
		if (settings.api_key == null || settings.api_key.trim() == '') {
			validation.apiKeyValidation = 'Please provide an API key';
		}

		if (validation.apiKeyValidation == null) {
			settingsHandler.updateSettings(settings).then((res) => {
				if (!res.isSuccessful) {
					notificationHandler.addError('Could not update settings', res.message);
				}

				closeDrawer(null);
			});
		}
	}

	/**
	 * Get available OpenAI models from the server.
	 */
	function getModels() {
		return settingsHandler.getModels().then((res) => {
			if (!res.isSuccessful) {
				notificationHandler.addError('Could not retrieve model information', res.message);
			} else {
				models = res.data as string[];
			}
		});
	}
</script>

<div class="drawer drawer-end">
	<!-- Checkbox to keep track of whether settings drawer is open or closed -->
	<input
		id="settings-drawer"
		type="checkbox"
		class="drawer-toggle"
		checked={drawerOpen}
		on:click|preventDefault={closeDrawer}
	/>

	<!-- Drawer content -->
	<div class="drawer-side">
		<!-- Background overlay -->
		<label for="settings-drawer" aria-label="close sidebar" class="drawer-overlay"></label>

		<div class="lg:w-3/4 w-[95%] min-h-full bg-base-200 text-base-content">
			<!-- Close button -->
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
				<div class="flex-none"></div>
			</nav>

			<div class="px-8">
				<!-- Header -->
				<h1 class="lg:text-4xl text-2xl font-bold py-8">Settings</h1>

				<!-- API key input -->
				<ValidatedInput
					label={'API Key'}
					placeholder={'******'}
					masked={true}
					bind:value={settings.api_key}
					bind:validation={validation.apiKeyValidation}
				/>

				<!-- Model dropdown -->
				<label class="form-control w-full max-w">
					<div class="label">
						<span class="label-text">OpenAI Model</span>
					</div>
					<select bind:value={settings.model} class="select select-bordered">
						{#each models as model}
							<option>{model}</option>
						{/each}
					</select>
				</label>

				<!-- Close and save button -->
				<div class="py-8 flex-none">
					<button class="btn btn-active btn-primary" on:click={updateSettings}>Save</button>
					<a href="/" class="btn btn-active">Close</a>
				</div>
			</div>
		</div>
	</div>
</div>
