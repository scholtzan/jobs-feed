import { postings } from '../store';
import { get } from 'svelte/store';
import { PostingsApi } from '../api/postings';
import type { RequestResponse } from '.';

/**
 * Handler to interact with postings API
 */
export class PostingsHandler {
	// posting data
	postings: Posting[] = [];
	// posting API instance
	api: PostingsApi;

	/**
	 * Create a new posting handler instance.
	 */
	constructor() {
		this.postings = get(postings);
		postings.subscribe(() => (this.postings = get(postings)));
		this.api = new PostingsApi();
	}

	/**
	 * Write posting data to Svelte store.
	 */
	public store(): void {
		postings.set(this.postings);
	}

	/**
	 * Call callback when posting data changes.
	 * @param callback function to call when data changes
	 */
	public subscribe(callback: (value: any) => void): void {
		postings.subscribe(callback);
	}

	/**
	 * Get bookmarked postings.
	 * @returns request response
	 */
	public async getBookmarked(): Promise<RequestResponse> {
		return this.api.getBookmarkedPostings();
	}

	/**
	 * Get a specific posting by its ID.
	 * @param id posting ID
	 * @returns request response
	 */
	public postingById(id: number | null): Promise<RequestResponse> {
		return this.api.getPostingById(id);
	}

	/**
	 * Get all postings that were fetched today.
	 * @returns postings
	 */
	public getTodaysPostings(): Posting[] {
		let currentDate = new Date();
		return this.postings.filter((p) => {
			let postingDate = new Date(p.created_at);
			return (
				postingDate.getFullYear() === currentDate.getFullYear() &&
				postingDate.getMonth() === currentDate.getMonth() &&
				postingDate.getDate() === currentDate.getDate()
			);
		});
	}

	/**
	 * Fetch postings.
	 * @param useCached whether cached postings should be returned, or postings should be scraped from sources.
	 * @param source_id [optional] ID of source to get postings for
	 * @returns request response
	 */
	public refresh(
		useCached: boolean = true,
		source_id: number | null = null
	): Promise<RequestResponse> {
		if (useCached) {
			if (source_id == null) {
				return this.api.getUnreadPostings().then((res) => {
					if (res.isSuccessful) {
						if (source_id == null) {
							this.postings = res.data as Posting[];
						} else {
							for (var posting of res.data as Posting[]) {
								if (this.postings.indexOf(posting) == -1) {
									this.postings.push(posting);
								}
							}
						}
						this.store();
					}

					return res;
				});
			} else {
				return this.getReadPostingsOfSource(source_id);
			}
		} else {
			return this.api.refreshPostings(source_id).then((res) => {
				if (res.isSuccessful) {
					this.postings = res.data as Posting[];
					this.store();
				}

				return res;
			});
		}
	}

	/**
	 * Group postings by source ID.
	 * @returns
	 */
	public postingsBySource(): Partial<Record<number, Posting[]>> {
		return Object.groupBy(this.postings, (p: Posting) => p.source_id as number);
	}

	/**
	 * Mark specific posts as seen.
	 * @param ids IDs of posts that should be marked as seen.
	 * @returns request response
	 */
	public markAsRead(ids: number[]): Promise<RequestResponse> {
		return this.api.markPostingsAsRead(ids).then((res) => {
			if (res.isSuccessful) {
				this.postings.forEach((p) => {
					if (ids != null && ids.includes(p.id as number)) {
						p.seen = true;
					}
				});

				this.store();
			}

			return res;
		});
	}

	/**
	 * Bookmark a specific posting
	 * @param id ID of posting to bookmark
	 * @returns request response
	 */
	public bookmarkPosting(id: number | null): Promise<RequestResponse> {
		return this.postingById(id).then((res) => {
			if (!res.isSuccessful) {
				return res;
			} else {
				let posting = res.data as Posting;
				posting.bookmarked = !posting.bookmarked;

				return this.api.updatePosting(posting).then((res) => {
					if (res.isSuccessful) {
						let postingIndex = this.postings.findIndex((p) => p.id == id);
						if (postingIndex != -1) {
							this.postings[postingIndex] = res.data as Posting;
						}
						this.store();
					}

					return res;
				});
			}
		});
	}

	/**
	 * Mark a specific posting as "liked".
	 * @param id ID of posting
	 * @returns request response
	 */
	public likePosting(id: number | null): Promise<RequestResponse> {
		return this.postingById(id).then((res) => {
			if (!res.isSuccessful) {
				return res;
			} else {
				let posting = res.data as Posting;
				if (posting.is_match == true) {
					posting.is_match = null;
				} else {
					posting.is_match = true;
				}

				return this.api.updatePosting(posting).then((res) => {
					if (res.isSuccessful) {
						let postingIndex = this.postings.findIndex((p) => p.id == id);
						if (postingIndex != -1) {
							this.postings[postingIndex] = res.data as Posting;
						}
						this.store();
					}

					return res;
				});
			}
		});
	}

	/**
	 * Mark as specific posting as disliked.
	 * @param id ID of posting
	 * @returns request response
	 */
	public dislikePosting(id: number | null): Promise<RequestResponse> {
		return this.postingById(id).then((res) => {
			if (!res.isSuccessful) {
				return res;
			} else {
				let posting = res.data as Posting;
				if (posting.is_match == false) {
					posting.is_match = null;
				} else {
					posting.is_match = false;
				}

				return this.api.updatePosting(posting).then((res) => {
					if (res.isSuccessful) {
						let postingIndex = this.postings.findIndex((p) => p.id == id);
						if (postingIndex != -1) {
							this.postings[postingIndex] = res.data as Posting;
						}
						this.store();
					}

					return res;
				});
			}
		});
	}

	/**
	 * Get all postings of a specific source that were seen before.
	 * @param sourceId ID of source
	 * @returns request response
	 */
	public getReadPostingsOfSource(sourceId: number | null | string): Promise<RequestResponse> {
		return this.api.getReadPostingsOfSource(sourceId).then((res) => {
			return res;
		});
	}
}

/**
 * Data container for a posting.
 */
export class Posting {
	id: number | null = null;
	title: string = '';
	description: string = '';
	url: string = '';
	seen: boolean = false;
	source_id: number | null = null;
	created_at: Date = new Date();
	bookmarked: boolean = false;
	content: string = '';
	is_match: boolean | null = null;
	match_similarity: number | null = null;
}
