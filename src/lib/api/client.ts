import { invoke as tauriInvoke } from '@tauri-apps/api/core';

export class ApiError extends Error {
	constructor(message: string) {
		super(message);
		this.name = 'ApiError';
	}
}

export async function invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
	try {
		return await tauriInvoke<T>(command, args);
	} catch (error) {
		const message =
			typeof error === 'string'
				? error
				: error instanceof Error
					? error.message
					: '调用本地服务失败';
		throw new ApiError(message);
	}
}
