import { error, success } from ".";
import { RequestResponse } from "../types/index";
import { Settings } from "../types/settings";

export class SettingsApi {
    constructor() {}

    public updateSettings = async (settings: Settings) => {
        return fetch('/settings', {
            method: 'PUT',
            body: JSON.stringify(settings)
        }).then((response) => {
            if (response.status == 200) {
                return response.json().then((json) => {
                    let storedSettings: Settings = Object.assign(new Settings(), json);
                    return success(storedSettings);
                })
            } else {
                return error(`Could not update settings: ${response}`);
            }
        });
    }

    public getSettings = async () => {
        return fetch('/settings', {
            method: 'GET'
        }).then((response) => {
            if (response.status == 200) {
                return response.json().then((json) => {
                    let storedSettings: Settings;
                    if (json == null) { 
                        storedSettings = new Settings();
                    } else {
                        storedSettings = Object.assign(new Settings(), json);
                    }
                    return success(storedSettings);
                });
            } else {
                return error(`Could not get settings: ${response}`);
            }
        });
    }
}
