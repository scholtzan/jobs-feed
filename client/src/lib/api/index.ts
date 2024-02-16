import { RequestResponse } from '../types/index';

export const error = (error: string, data?: any): RequestResponse => {
	return {
		status: 'error',
		isSuccessful: false,
		message: error,
		data
	};
};

export const success = (data: any): RequestResponse => {
	return {
		status: 'success',
		isSuccessful: true,
		message: '',
		data
	};
};
