import { writable } from 'svelte/store';

export const sources = writable([]);
export const filters = writable([]);
export const postings = writable([]);
export const settings = writable({});
export const selectedSource = writable('all');
export const notifications = writable([]);
export const showSidebar = writable(false);
