import { notifications } from '../store';
import { get } from 'svelte/store';

/**
 * Types of notifications, based on severity.
 */
export enum NotificationType {
	Error = 'error',
	Warning = 'warning',
	Success = 'success'
}

/**
 * Data container for a single notification.
 */
export interface Notification {
	type: NotificationType;
	message: string;
	details: string | null;
}

/**
 * Handler for creating notifications.
 */
export class NotificationHandler {
	// active notifications
	notifications: Notification[] = [];

	/**
	 * Create a new notification handler instance.
	 */
	constructor() {
		this.notifications = get(notifications);

		notifications.subscribe((_) => {
			this.notifications = get(notifications);
		});
	}

	/**
	 * Write active notifications to the Svelte store.
	 */
	public store(): void {
		notifications.set(this.notifications);
	}

	/**
	 * Execute callback if notification data changes.
	 * @param callback function to run when data changes
	 */
	public subscribe(callback: (value: any) => void): void {
		notifications.subscribe(callback);
	}

	/**
	 * Create a new notification of a specific type.
	 * @param type notification type
	 * @param message notification message
	 * @param details extra data shown as part of notification
	 */
	public add(type: NotificationType, message: string, details: string | null = null): void {
		let notification: Notification = {
			type: type,
			message: message,
			details: details
		};
		this.notifications.push(notification);
		this.store();
	}

	/**
	 * Create an error notification.
	 * @param message notification message
	 * @param details extra notification data
	 */
	public addError(message: string, details: string | null = null): void {
		this.add(NotificationType.Error, message, details);
	}

	/**
	 * Create a success notification.
	 * @param message notification message
	 * @param details extra notification data
	 */
	public addMessage(message: string, details: string | null = null) {
		this.add(NotificationType.Success, message, details);
	}

	/**
	 * Delete a specific notification.
	 * @param notification notification to delete
	 */
	public remove(notification: Notification) {
		this.notifications = this.notifications.filter((n) => n != notification);
		this.store();
	}
}
