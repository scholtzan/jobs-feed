import { filters } from '../store';
import { get } from 'svelte/store';
import { FiltersApi } from '../api/filters';
import type { RequestResponse } from '.';

/**
 * Handler for interacting with the filters API.
 */
export class FiltersHandler {
	// filters data
	filters: Filter[] = [];
	// filters API instance
	api: FiltersApi;

	/**
	 * Create a new filters handler instance.
	 */
	constructor() {
		this.filters = get(filters);
		this.api = new FiltersApi();

		filters.subscribe((_) => {
			this.filters = get(filters);
		});
	}

	/**
	 * Write the filters data to the Svelte store.
	 */
	public store(): void {
		filters.set(this.filters);
	}

	/**
	 * Subscribe to filter data changes.
	 * @param callback function to call when filters data changes
	 */
	public subscribe(callback: (value: any) => void): void {
		filters.subscribe(callback);
	}

	/**
	 * Fetch filters data from the server.
	 * @returns request response
	 */
	public refresh(): Promise<RequestResponse> {
		return this.api.getFilters().then((res) => {
			if (res.isSuccessful) {
				this.filters = res.data as Filter[];
				this.store();
			}

			return res;
		});
	}

	/**
	 * Update the filters data.
	 * @param filters updated filters data
	 * @returns request response
	 */
	public updateFilters(filters: Filter[]): Promise<RequestResponse> {
		return this.api.updateFilters(filters).then((res) => {
			if (res.isSuccessful) {
				this.filters = res.data as Filter[];
				this.store();
			}

			return res;
		});
	}
}

/**
 * Data container for a single filter.
 */
export class Filter {
	id: number | null = null;
	name: string = '';
	value: string = '';
}
