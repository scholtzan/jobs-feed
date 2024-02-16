export const ssr = false;
export const csr = true;
export const trailingSlash = 'always'; 

import "../app.postcss";
import { SettingsHandler } from "../lib/types/settings";
import { Sources } from "../lib/types/sources";
import { Postings } from "../lib/types/postings";
import { Filters } from "../lib/types/filters";

import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
  let settingsHandler = new SettingsHandler();
  let sourcesHandler = new Sources();
  let postingsHandler = new Postings();
  let filtersHandler = new Filters();

  settingsHandler.refresh().then((res) => {
    if (!res.isSuccessful) {
      console.log("Cannot get settings"); // todo
    }
  });

  sourcesHandler.refresh().then((res) => {
    if (!res.isSuccessful) {
      console.log("Cannot get sources"); // todo
    }
  });

  postingsHandler.refresh(true).then((res) => {
    if (!res.isSuccessful) {
      console.log("Cannot get postings"); // todo
    }
  });

  filtersHandler.refresh().then((res) => {
    if (!res.isSuccessful) {
      console.log("Cannot get filters"); // todo
    }
  });

  return {};
};
