<script lang="ts">
import { cx } from '$lib/utils/format';

type Props = {
	value?: string;
	placeholder?: string;
	type?: string;
	disabled?: boolean;
	oninput?: (value: string) => void;
};

let {
	value = $bindable(''),
	placeholder,
	type = 'text',
	disabled = false,
	oninput
}: Props = $props();
</script>

<input
	class={cx('control')}
	{type}
	{placeholder}
	{disabled}
	bind:value
	oninput={() => oninput?.(value)}
/>
