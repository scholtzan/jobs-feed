import { sources, selectedSource } from '../store';
import { get } from 'svelte/store';
import { SourcesApi } from '../api/sources';
import type { RequestResponse } from '.';

/**
 * Types of sources that can be selected, besides specific source IDs.
 */
export const SelectedSource = {
	All: 'all',
	Today: 'today',
	Bookmarked: 'bookmarked'
};

/**
 * Handler to interact with source API.
 */
export class SourcesHandler {
	// sources data
	sources: Source[] = [];
	// source that is currently selected, used to filter postings
	selectedSource: string | number | null;
	// source API instance
	api: SourcesApi;

	/**
	 * Create a new source handler instance.
	 */
	constructor() {
		this.sources = get(sources);
		this.api = new SourcesApi();
		this.selectedSource = get(selectedSource);

		// get source data whenever it changes
		sources.subscribe((_) => {
			this.sources = get(sources);
		});

		// get current selected source whenever it changes
		selectedSource.subscribe((_) => {
			this.selectedSource = get(selectedSource);
		});
	}

	/**
	 * Write source data into the store.
	 */
	public store(): void {
		sources.set(this.sources);
	}

	/**
	 * Let consumer subscribe to any source data changes.
	 * @param callback function to call when source data changes
	 */
	public subscribe(callback: (value: any) => void): void {
		sources.subscribe(callback);
	}

	/**
	 * Let consumer subscribe to any selected source changes.
	 * @param callback function to call when selected source changes
	 */
	public subscribeSelectedSource(callback: (value: any) => void): void {
		selectedSource.subscribe(callback);
	}

	/**
	 * Get data of specific source.
	 * @param id ID of source to get data for
	 * @returns source data
	 */
	public sourceById(id: number | string | null): Source | undefined {
		return this.sources.find((s) => s.id == id);
	}

	/**
	 * Change the selected source.
	 * @param selected ID of source to be selected
	 */
	public setSelectedSource(selected: string | number | null): void {
		this.selectedSource = selected;
		selectedSource.set(selected);
	}

	/**
	 * Sort source data alphabetically.
	 * @returns sorted sources data
	 */
	public sortedSources(): Source[] {
		return this.sources.sort((s1, s2) => {
			const name1 = s1.name.toLowerCase();
			const name2 = s2.name.toLowerCase();
			if (name1 > name2) {
				return 1;
			}
			if (name1 < name2) {
				return -1;
			}
			return 0;
		});
	}

	/**
	 * Fetch source data from server.
	 * @returns request response
	 */
	public refresh(): Promise<RequestResponse> {
		return this.api.getSources().then((res) => {
			if (res.isSuccessful) {
				this.sources = res.data as Source[];
				this.store();
			}

			return res;
		});
	}

	/**
	 * Create a new source.
	 * @param source data of new source
	 * @returns request response
	 */
	public createSource(source: Source): Promise<RequestResponse> {
		return this.api.createSource(source).then((res) => {
			if (res.isSuccessful) {
				this.sources.push(res.data as Source);
				this.store();
			}

			return res;
		});
	}

	/**
	 * Update data of a specific source.
	 * @param source updated source data
	 * @returns request response
	 */
	public updateSource(source: Source): Promise<RequestResponse> {
		return this.api.updateSource(source).then((res) => {
			if (res.isSuccessful) {
				let filteredSources = this.sources.filter((s) => s.id != source.id);
				this.sources = [...(filteredSources as Source[]), res.data as Source];
				this.store();
			}

			return res;
		});
	}

	/**
	 * Delete a specific source by ID
	 * @param sourceId ID of source to delete
	 * @returns request response
	 */
	public deleteSource(sourceId: number | null): Promise<RequestResponse> {
		return this.api.deleteSource(sourceId).then((res) => {
			if (res.isSuccessful) {
				this.sources = this.sources.filter((s) => s.id != sourceId);
				this.store();
			}

			return res;
		});
	}

	/**
	 * Delete the content that has been cached for a specific source
	 * @param sourceId ID of source to remove cached content for
	 * @returns request response
	 */
	public resetSourceCache(sourceId: number | null): Promise<RequestResponse> {
		return this.api.resetSourceCache(sourceId).then((res) => {
			return res;
		});
	}
}

/**
 * Data container for a source.
 */
export class Source {
	name: string = '';
	url: string = '';
	id: number | null = null;
	content: string | null = '';
	pagination: string | null = null;
	selector: string | null = null;
	favicon: string | null = null;
	unreachable: boolean = false;
	deleted: boolean = false;
	refreshing: boolean = false;
}
