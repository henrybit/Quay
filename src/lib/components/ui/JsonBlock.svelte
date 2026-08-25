<script lang="ts">
import { prettyJson } from '$lib/utils/format';

type Props = {
	value: unknown;
};

let { value }: Props = $props();
</script>

<pre class="json-block">{prettyJson(value)}</pre>
