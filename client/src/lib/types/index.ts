/**
 * API response.
 */
export interface RequestResponse {
	// status is based on the status code of the request
	status: 'success' | 'error';
	// whether the the request was successful
	isSuccessful: boolean;
	// response message
	message: string;
	// response body
	data: unknown;
}
