import { settings } from '../store';
import { get, writable } from 'svelte/store';
import { SettingsApi } from '../api/settings';

export class SettingsHandler {
	settings: Settings;
	api: SettingsApi;

	constructor() {
		this.settings = get(settings);
		this.api = new SettingsApi();

		settings.subscribe((_) => {
			this.settings = get(settings);
		});
	}

	public store(): void {
		settings.set(this.settings);
	}

	public subscribe(callback: (value: any) => void): void {
		settings.subscribe(callback);
	}

	public refresh() {
		return this.api.getSettings().then((res) => {
			if (res.isSuccessful) {
				this.settings = res.data;
				this.store();
			}

			return res;
		});
	}

	public updateSettings(settings: Settings) {
		return this.api.updateSettings(settings).then((res) => {
			if (res.isSuccessful) {
				this.settings = res.data;
				this.store();
			}

			return res;
		});
	}
}

export class Settings {
	id: number | null = null;
	api_key: string | null = null;
}
