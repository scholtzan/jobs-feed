import { writable, type Writable } from 'svelte/store';
import type { Source } from './types/sources';
import type { Filter } from './types/filters';
import type { Posting } from './types/postings';
import type { Notification } from './types/notifications';
import { Settings } from './types/settings';

export const sources: Writable<Source[]> = writable([]);
export const filters: Writable<Filter[]> = writable([]);
export const postings: Writable<Posting[]> = writable([]);
export const settings: Writable<Settings> = writable({});
export const selectedSource: Writable<string | number | null> = writable('all');
export const notifications: Writable<Notification[]> = writable([]);
export const showSidebar = writable(false);
