export const ssr = false;
export const csr = true;
export const trailingSlash = 'always'; 

import "../app.postcss";

import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
  return {
    sources: (await fetch("/sources").then((data) => data.json())) as Source[],
    filters: (await fetch("/filters").then((data) => data.json())) as Filter[],
    postings: (await fetch("/postings/unread").then((data) => data.json())) as Posting[],
    settings: (await fetch("/settings").then((data) => data.json())) as Settings,
  };
};
