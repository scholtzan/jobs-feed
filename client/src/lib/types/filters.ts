import { filters } from "../store";
import { get, writable } from 'svelte/store';
import { FiltersApi } from "../api/filters";


export class Filters {
    filters: Filter[] = [];
    api: FiltersApi;

    constructor() {
        this.filters = get(filters);
        this.api = new FiltersApi();
    }

    public store(): void {
        filters.set(this.filters);
    }

    public subscribe(callback: (value: any) => void): void {
        filters.subscribe(callback);
    }

    public updateFilters(filters: Filter[]) {
        return this.api.updateFilters(filters).then((res) => {
            if (res.isSuccessful) {
                this.filters = res.data;                    
                this.store();
            }

            return res;
        });
    } 
}


export class Filter {
    id: number | null = null;
    name: string = "";
    value: string = "";
}
