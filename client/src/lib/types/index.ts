export interface RequestResponse {
	status: 'success' | 'error';
	isSuccessful: boolean;
	message: string;
	data: unknown;
}
