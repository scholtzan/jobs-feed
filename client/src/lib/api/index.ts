import { type RequestResponse } from '../types/index';

// API request response if an error occurred
export const error = (error: string, data?: any): RequestResponse => {
	return {
		status: 'error',
		isSuccessful: false,
		message: error,
		data
	};
};

// API request response if successfull
export const success = (data: any): RequestResponse => {
	return {
		status: 'success',
		isSuccessful: true,
		message: '',
		data
	};
};
