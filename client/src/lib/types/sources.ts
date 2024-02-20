import { sources, selectedSource } from '../store';
import { get, writable } from 'svelte/store';
import { SourcesApi } from '../api/sources';

export const SelectedSource = {
	All: 'all',
	Today: 'today',
	Bookmarked: 'bookmarked'
};

export class Sources {
	sources: Source[] = [];
	selectedSource: string | number | null;
	api: SourcesApi;

	constructor() {
		this.sources = get(sources);
		this.api = new SourcesApi();
		this.selectedSource = get(selectedSource);

		sources.subscribe((_) => {
			this.sources = get(sources);
		});

		selectedSource.subscribe((_) => {
			this.selectedSource = get(selectedSource);
		});
	}

	public store(): void {
		sources.set(this.sources);
	}

	public subscribe(callback: (value: any) => void): void {
		sources.subscribe(callback);
	}

	public subscribeSelectedSource(callback: (value: any) => void): void {
		selectedSource.subscribe(callback);
	}

	public sourceById(id: number) {
		return this.sources.find((s) => s.id == id);
	}

	public setSelectedSource(selected: string | number | null) {
		this.selectedSource = selected;
		selectedSource.set(selected);
	}

	public refresh() {
		return this.api.getSources().then((res) => {
			if (res.isSuccessful) {
				this.sources = res.data;
				this.store();
			}

			return res;
		});
	}

	public createSource(source: Source) {
		return this.api.createSource(source).then((res) => {
			if (res.isSuccessful) {
				this.sources.push(res.data);
				this.store();
			}

			return res;
		});
	}

	public updateSource(source: Source) {
		return this.api.updateSource(source).then((res) => {
			if (res.isSuccessful) {
				let filteredSources = this.sources.filter((s) => s.id != source.id);
				this.sources = [...filteredSources, res.data];
				this.store();
			}

			return res;
		});
	}

	public deleteSource(sourceId: number) {
		return this.api.deleteSource(sourceId).then((res) => {
			if (res.isSuccessful) {
				this.sources = this.sources.filter((s) => s.id != sourceId);
				this.store();
			}

			return res;
		});
	}
}

export class Source {
	name: string = '';
	url: string = '';
	id: number | null = null;
	pagination: string | null = null;
	selector: string | null = null;
}
