<script lang="ts">
	export let label = '';
	export let placeholder = '';
	export let validation = null;
	export let value = '';
	export let masked = false;
</script>

<label class="form-control w-full max-w">
	<div class="label">
		<span class="label-text">{label}</span>
	</div>

	<div
		class="inline-block {validation ? 'tooltip tooltip-open tooltip-error' : ''}"
		data-tip={validation || null}
	>
		{#if masked}
			<input
				type="password"
				{placeholder}
				class="input input-bordered w-full max-w {validation ? 'input-error' : ''}"
				on:input={() => (validation = null)}
				bind:value
			/>
		{:else}
			<input
				type="text"
				{placeholder}
				class="input input-bordered w-full max-w {validation ? 'input-error' : ''}"
				on:input={() => (validation = null)}
				bind:value
			/>
		{/if}
	</div>
</label>
