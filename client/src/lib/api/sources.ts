import { error, success } from ".";
import { RequestResponse } from "../types/index";
import { Source } from "../types/sources";

export class SourcesApi {
    constructor() {}

    public createSource = async (source: Source) => {
        return fetch('/sources', {
            method: 'POST',
            body: JSON.stringify(source)
        }).then((response) => {
            if (response.status == 200) {
                return response.json().then((json) => {
                    let storedSource: Source = Object.assign(new Source(), json); 
                    return success(storedSource);
                });
            } else {
                return error(`Cannot add source. ${response}`);
            }
        });
    }

    public updateSource = async (source: Source) => {
        return fetch('/sources/' + source.id, {
            method: 'PUT',
            body: JSON.stringify(source)
        }).then((response) => {
            if (response.status == 200) {
                return response.json().then((json) => {
                    let storedSource: Source = Object.assign(new Source(), json);
                    return success(storedSource);
                })
            } else {
                return error(`Could not update source: ${response}`);
            }
        });
    }

    public deleteSource = async (sourceId: number) => {
        return fetch('/postings/' + sourceId, {
            method: 'DELETE'
        }).then((response) => {
            if (response.status == 200) {
                return success({});
            } else {
                return error("Could not delete source");
            }
        });
    }
}
