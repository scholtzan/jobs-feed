import type { RequestResponse } from '.';
import { SuggestionsApi } from '../api/suggestions';

/**
 * Handler to call the suggestions API.
 */
export class SuggestionsHandler {
	// API instance
	api: SuggestionsApi;

	/**
	 * Create a new suggestion handler instance.
	 */
	constructor() {
		this.api = new SuggestionsApi();
	}

	/**
	 * Get all suggestions.
	 * @returns request response
	 */
	public getSuggestions(): Promise<RequestResponse> {
		return this.api.getSuggestions();
	}

	/**
	 * Get suggestions similar to a specific source.
	 * @param sourceId ID of source to get suggestions for
	 * @returns request response
	 */
	public getSourceSuggestions(sourceId: number | string | null): Promise<RequestResponse> {
		return this.api.getSourceSuggestions(sourceId);
	}

	/**
	 * Refresh suggestions for a specific source.
	 * @param sourceId ID of source to refresh suggestions for
	 * @returns request response
	 */
	public refreshSourceSuggestions(sourceId: number | string | null): Promise<RequestResponse> {
		return this.api.refreshSourceSuggestions(sourceId);
	}
}

/**
 * Data container for a source suggestion.
 */
export class Suggestion {
	id: number | null = null;
	name: string = '';
	url: string = '';
	source_id: number | null = null;
}
