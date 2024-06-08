<!--
  @component
  ## To make this work:
  You MUST SET `peer` tailwind class, to the previous HTML element manually

  ### Notes:
  you may need to style your parent element by adding `flex flex-row-reverse`, add some `gap` and so on... because this component is just a tooltip, not a container
-->

<script lang="ts">
	import { NotificationHandler, NotificationType, type Notification } from '../types/notifications';
	import SvelteMarkdown from 'svelte-markdown';

	let notificationHandler = new NotificationHandler();

	let notifications = notificationHandler.notifications;

	notificationHandler.subscribe((_) => {
		notifications = notificationHandler.notifications;
	});

	/**
	 * Delete a specific notification
	 * @param notification notification to delete
	 */
	function removeNotification(notification: Notification) {
		notificationHandler.remove(notification);
	}
</script>

<div class="absolute top-6 right-6 gap-y-4 grid grid-cols-1 max-w-[35em] w-[88%]">
	{#each notifications as notification}
		<!-- Show different notification types in different colors -->
		<div
			role="alert"
			class="alert {notification.type == NotificationType.Error
				? 'alert-error'
				: notification.type == NotificationType.Warning
					? 'alert-warning'
					: ''} shadow-lg w-full z-40"
		>
			{#if notification.type == NotificationType.Error}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke-width="1.5"
					stroke="currentColor"
					class="w-6 h-6"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M12 9v3.75m9-.75a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 3.75h.008v.008H12v-.008Z"
					/>
				</svg>
			{:else if notification.type == NotificationType.Warning}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke-width="1.5"
					stroke="currentColor"
					class="w-6 h-6"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126ZM12 15.75h.007v.008H12v-.008Z"
					/>
				</svg>
			{/if}

			<!-- Collapsible details -->
			<div>
				<details class="collapse">
					<summary class="collapse-title font-bold">
						<SvelteMarkdown source={notification.message} />
					</summary>
					{#if notification.details}
						<div class="collapse-content font-small">
							<SvelteMarkdown source={notification.details} />
						</div>
					{/if}
				</details>
			</div>

			<!-- Close button -->
			<button
				title="Close"
				on:click={() => removeNotification(notification)}
				class="btn btn-ghost btn-sm btn-circle"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-6 w-6"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M6 18L18 6M6 6l12 12"
					/></svg
				>
			</button>
		</div>
	{/each}
</div>
