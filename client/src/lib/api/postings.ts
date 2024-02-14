import { error, success } from ".";
import { RequestResponse } from "../types/index";
import { Posting } from "../types/postings";

export class PostingsApi {
    constructor() {}

    public getPostings = async () => {

    }

    public getBookmarkedPostings = async () => {
        return fetch("/postings/bookmarked", {
            method: "GET"
        }).then((response) => {
            if (response.status == 200) {
                return response.json().then((json) => {
                    return success(json as Posting[]);
                });
            } else {
                return error("Could not get bookmarked postings");
            }
        });
    }

    public markPostingsAsRead = async (ids: number[]) => {
        return fetch('/postings/mark_read', {
            method: 'PUT',
            body: JSON.stringify(ids)
        }).then((response) => {
            if (response.status == 200) {
                return success({})
            } else {
                return error("Cannot mark postings as read");
            }
        });
    }

    public updatePosting = async (posting: Posting) => {
        return fetch('/postings/' + posting.id, {
            method: 'PUT',
            body: JSON.stringify(posting)
        }).then((response) => {
            if (response.status == 200) {
                return response.json().then((json) => {
                    return success(json as Posting);
                });
            } else {
                return error("Could not bookmark posting");
            }
        });
    }
}
