<script lang="ts">
type Props = {
	label?: string;
};

let { label = '加载中' }: Props = $props();
</script>

<div class="spinner-wrap">
	<span class="spinner"></span>
	<span>{label}</span>
</div>
