<script lang="ts">
	type Props = {
		value?: string;
		placeholder?: string;
		rows?: number;
		disabled?: boolean;
		oninput?: (value: string) => void;
	};

	let {
		value = $bindable(''),
		placeholder,
		rows = 4,
		disabled = false,
		oninput
	}: Props = $props();
</script>

<textarea
	class="control control-area"
	{placeholder}
	{rows}
	{disabled}
	bind:value
	oninput={() => oninput?.(value)}
></textarea>
