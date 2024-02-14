import { postings } from "../store";
import { get, writable } from 'svelte/store';
import { PostingsApi } from "../api/postings";

export class Postings {
    postings: Posting[] = [];
    api: PostingsApi

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
        return this.api.getBookmarkedPostings()
    }

    public postingById(id: number) {
        return this.postings.find((p) => p.id == id)
    }

    public getTodaysPostings(): Posting[] {
        let currentDate = new Date();
        return this.postings.filter((p) => {
            let postingDate = new Date(p.created_at);
            return postingDate.getFullYear() === currentDate.getFullYear() &&
            postingDate.getMonth() === currentDate.getMonth() &&
            postingDate.getDate() === currentDate.getDate()
        });
    }

    public postingsBySource(): Posting[] {
        return Object.groupBy(this.postings, ( p: Posting ) => p.source_id);
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
        let posting = this.postingById(id);

        if (posting === undefined) {
            return;
        }

        posting.bookmarked = !posting.bookmarked;

        return this.api.updatePosting(posting).then((res) => {
            if (!res.isSuccessful) {
                posting.bookmarked = !posting.bookmarked;
            } else {
                this.store();
            }

            return res;
        })
    }
}

export class Posting {
    id: number | null = null;
    title: string = "";
    description: string = "";
    url: string = "";
    seen: boolean = false;
    source_id: number | null = null;
    created_at: Date = new Date();
    bookmarked: boolean = false;
}