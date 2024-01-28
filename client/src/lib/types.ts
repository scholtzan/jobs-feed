export class Source {
    name: string = "";
    url: string = "";
    id: number | null = null;
    pagination: string | null = null;
    selector: string | null = null;
}

export class Filter {
    id: number | null = null;
    name: string = "";
    value: string = "";
}