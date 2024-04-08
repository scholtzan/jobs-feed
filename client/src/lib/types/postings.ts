import { postings } from '../store';
import { get } from 'svelte/store';
import { PostingsApi } from '../api/postings';

export class Postings {
	postings: Posting[] = [];
	api: PostingsApi;

	constructor() {
		this.postings = get(postings);
		this.api = new PostingsApi();
	}

	public store(): void {
		postings.set(this.postings);
	}

	public subscribe(callback: (value: any) => void): void {
		postings.subscribe(callback);
	}

	public async getBookmarked() {
		return this.api.getBookmarkedPostings();
	}

	public postingById(id: number) {
		return this.api.getPostingById(id);
	}

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

	public refresh(useCached: boolean = true, source_id: number | null = null) {
		if (useCached) {
			if (source_id == null) {
				return this.api.getUnreadPostings().then((res) => {
					if (res.isSuccessful) {
						if (source_id == null) {
							this.postings = res.data;
						} else {
							for (var posting of res.data) {
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

	public postingsBySource(): Posting[] {
		return Object.groupBy(this.postings, (p: Posting) => p.source_id);
	}

	public markAsRead(ids: number[]) {
		return this.api.markPostingsAsRead(ids).then((res) => {
			if (res.isSuccessful) {
				this.postings.forEach((p) => {
					if (ids.includes(p.id)) {
						p.seen = true;
					}
				});

				this.store();
			}

			return res;
		});
	}

	public bookmarkPosting(id: number) {
		return this.postingById(id).then((res) => {
			if (!res.isSuccessful) {
				return res;
			} else {
				let posting = res.data;
				posting.bookmarked = !posting.bookmarked;

				return this.api.updatePosting(posting).then((res) => {
					if (res.isSuccessful) {
						let postingIndex = this.postings.findIndex((p) => p.id == id);
						if (postingIndex != -1) {
							this.postings[postingIndex] = res.data;
						}
						this.store();
					}

					return res;
				});
			}
		});
	}

	public likePosting(id: number) {
		return this.postingById(id).then((res) => {
			if (!res.isSuccessful) {
				return res;
			} else {
				let posting = res.data;
				if (posting.is_match == true) {
					posting.is_match = null;
				} else {
					posting.is_match = true;
				}

				return this.api.updatePosting(posting).then((res) => {
					if (res.isSuccessful) {
						let postingIndex = this.postings.findIndex((p) => p.id == id);
						if (postingIndex != -1) {
							this.postings[postingIndex] = res.data;
						}
						this.store();
					}

					return res;
				});
			}
		});
	}

	public dislikePosting(id: number) {
		return this.postingById(id).then((res) => {
			if (!res.isSuccessful) {
				return res;
			} else {
				let posting = res.data;
				if (posting.is_match == false) {
					posting.is_match = null;
				} else {
					posting.is_match = false;
				}

				return this.api.updatePosting(posting).then((res) => {
					if (res.isSuccessful) {
						let postingIndex = this.postings.findIndex((p) => p.id == id);
						if (postingIndex != -1) {
							this.postings[postingIndex] = res.data;
						}
						this.store();
					}

					return res;
				});
			}
		});
	}

	public getReadPostingsOfSource(sourceId: number) {
		return this.api.getReadPostingsOfSource(sourceId).then((res) => {
			return res;
		});
	}
}

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
