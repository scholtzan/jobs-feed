export const ssr = false;
export const csr = true;
export const trailingSlash = 'always'; 


import "../app.postcss";
import { Settings } from "../lib/types";

import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
  return {
    sources: (await fetch("/sources").then((data) => data.json())) as Source[],
    filters: (await fetch("/filters").then((data) => data.json())) as Filter[],
    postings: (await fetch("/postings/unread").then((data) => data.json())) as Posting[],
    settings: (await fetch("/settings").then((data) => {
      return data.json().then((json) => {
        if (json == null) {
          return new Settings();
        } else {
          return json;
        }
      })
    })) as Settings,
  };
};
