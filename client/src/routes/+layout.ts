export const ssr = false;
export const csr = true;
export const trailingSlash = 'always'; 

import "../app.postcss";

import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
  return {
    sources: (await fetch("/sources").then((data) => data.json())) as Source[]
  };
};
