import { notifications } from '../store';
import { get, writable } from 'svelte/store';

export enum NotificationType {
	Error = 'error',
	Warning = 'warning',
	Success = 'success'
}

export interface Notification {
	type: NotificationType;
	message: string;
	details: string | null;
}

export class NotificationHandler {
	notifications: Notification[] = [];

	constructor() {
		this.notifications = get(notifications);

		notifications.subscribe((_) => {
			this.notifications = get(notifications);
		});
	}

	public store(): void {
		notifications.set(this.notifications);
	}

	public subscribe(callback: (value: any) => void): void {
		notifications.subscribe(callback);
	}

	public add(type: NotificationType, message: string, details: string | null = null) {
		let notification: Notification = {
			type: type,
			message: message,
			details: details
		};
		this.notifications.push(notification);
		this.store();
	}

	public addError(message: string, details: string | null = null) {
		this.add(NotificationType.Error, message, details);
	}

	public remove(notification: Notification) {
		this.notifications = this.notifications.filter((n) => n != notification);
		this.store();
	}
}
