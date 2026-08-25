<script lang="ts">
type Props = {
	value?: string;
	placeholder?: string;
};

let { value = $bindable(''), placeholder = '搜索' }: Props = $props();
</script>

<label class="search">
	<span aria-hidden="true">⌕</span>
	<input class="control search-input" type="search" {placeholder} bind:value />
</label>
