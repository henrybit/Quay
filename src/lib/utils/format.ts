export function cx(...parts: Array<string | false | null | undefined>): string {
	return parts.filter(Boolean).join(' ');
}

export function formatNumber(value: number | null | undefined): string {
	if (value == null || Number.isNaN(value)) return '—';
	return new Intl.NumberFormat('zh-CN').format(value);
}

export function formatTime(value: string | number | null | undefined): string {
	if (value == null || value === '') return '—';
	const date = typeof value === 'number' ? new Date(value) : new Date(value);
	if (Number.isNaN(date.getTime())) return String(value);
	return new Intl.DateTimeFormat('zh-CN', {
		month: '2-digit',
		day: '2-digit',
		hour: '2-digit',
		minute: '2-digit',
		second: '2-digit',
		hour12: false
	}).format(date);
}

export function asArray<T>(value: unknown): T[] {
	return Array.isArray(value) ? (value as T[]) : [];
}

export function readPath(value: unknown, path: string): unknown {
	return path.split('.').reduce<unknown>((cursor, key) => {
		if (cursor && typeof cursor === 'object' && key in cursor) {
			return (cursor as Record<string, unknown>)[key];
		}
		return undefined;
	}, value);
}

export function prettyJson(value: unknown): string {
	try {
		return JSON.stringify(value, null, 2);
	} catch {
		return String(value);
	}
}

export function clip(text: string, max = 120): string {
	if (text.length <= max) return text;
	return `${text.slice(0, max)}…`;
}
