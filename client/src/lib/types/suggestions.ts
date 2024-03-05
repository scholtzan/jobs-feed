import { SuggestionsApi } from '../api/suggestions';

export class Suggestions {
	api: SuggestionsApi;

	constructor() {
		this.api = new SuggestionsApi();
	}

	public getSuggestions() {
		return this.api.getSuggestions();
	}

	public getSourceSuggestions(sourceId: number) {
		return this.api.getSourceSuggestions(sourceId);
	}

	public refreshSourceSuggestions(sourceId: number) {
		return this.api.refreshSourceSuggestions(sourceId);
	}
}

export class Suggestion {
	id: number | null = null;
	name: string = '';
	url: string = '';
	source_id: number | null = null;
}
