import { UsageApi } from '../api/usage';

export class Usages {
	usages: Usage[] = [];
	api: UsageApi;

	constructor() {
		this.usages = [];
		this.api = new UsageApi();
	}

	public getUsageCost(days: number | null) {
		return this.api.getUsageCost(days).then((res) => {
			return res;
		});
	}
}

export class Usage {
	source_name: string = '';
	source_id: number | null = null;
	cost: number = 0.0;
}
