<script lang="ts">
import type { Snippet } from 'svelte';

type Props = {
	open: boolean;
	title: string;
	wide?: boolean;
	onclose: () => void;
	children: Snippet;
	footer?: Snippet;
};

let { open, title, wide = false, onclose, children, footer }: Props = $props();
</script>

{#if open}
	<div
		class="modal-backdrop"
		onclick={onclose}
		onkeydown={(event) => {
			if (event.key === 'Escape') onclose();
		}}
		role="presentation"
	>
		<div
			class="modal"
			class:wide
			tabindex="-1"
			onclick={(event) => event.stopPropagation()}
			onkeydown={(event) => event.stopPropagation()}
			role="dialog"
			aria-modal="true"
		>
			<header class="modal-head">
				<h2>{title}</h2>
				<button class="icon-btn" type="button" onclick={onclose} aria-label="关闭">✕</button>
			</header>
			<div class="modal-body">
				{@render children()}
			</div>
			{#if footer}
				<footer class="modal-foot">
					{@render footer()}
				</footer>
			{/if}
		</div>
	</div>
{/if}
