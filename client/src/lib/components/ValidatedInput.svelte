<!--
  @component
  Text input that will show an error tooltip if the validation has failed for the input.
-->
<script lang="ts">
	export let label: string = '';
	export let placeholder: string = '';
	export let validation: null | string = null;
	export let value: string = '';
	export let masked: boolean = false; // whether or not to show the input text
</script>

<label class="form-control w-full max-w">
	<!-- Input label -->
	<div class="label">
		<span class="label-text">{label}</span>
	</div>

	<!-- Input -->
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
