import { writable } from "svelte/store";
import { setContext } from 'svelte';

export const sources = writable([]);
setContext('sources', sources);