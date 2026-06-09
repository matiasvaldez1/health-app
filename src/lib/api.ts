import type {
	WeightEntry,
	MeditationSession,
	FeelingEntry,
	AiTip,
	CreateWeightRequest,
	CreateMeditationRequest,
	CreateFeelingRequest,
	AppSettings
} from './types';

function isTauri(): boolean {
	return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

const API_BASE = 'http://localhost:3333';

async function tauriInvoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
	const { invoke } = await import('@tauri-apps/api/core');
	return invoke<T>(command, args);
}

async function httpGet<T>(path: string): Promise<T> {
	const resp = await fetch(`${API_BASE}${path}`);
	if (!resp.ok) throw new Error(await resp.text());
	return resp.json();
}

async function httpPost<T>(path: string, body?: unknown): Promise<T> {
	const resp = await fetch(`${API_BASE}${path}`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: body ? JSON.stringify(body) : undefined
	});
	if (!resp.ok) throw new Error(await resp.text());
	const text = await resp.text();
	return text ? JSON.parse(text) : undefined;
}

async function httpDelete(path: string): Promise<void> {
	const resp = await fetch(`${API_BASE}${path}`, { method: 'DELETE' });
	if (!resp.ok) throw new Error(await resp.text());
}

// Weight
export async function createWeight(req: CreateWeightRequest): Promise<WeightEntry> {
	if (isTauri()) return tauriInvoke('cmd_create_weight', { req });
	return httpPost('/api/weight', req);
}

export async function listWeights(from?: string, to?: string): Promise<WeightEntry[]> {
	if (isTauri()) return tauriInvoke('cmd_list_weights', { from: from ?? null, to: to ?? null });
	const params = new URLSearchParams();
	if (from) params.set('from', from);
	if (to) params.set('to', to);
	const qs = params.toString();
	return httpGet(`/api/weight${qs ? '?' + qs : ''}`);
}

export async function deleteWeight(id: number): Promise<void> {
	if (isTauri()) return tauriInvoke('cmd_delete_weight', { id });
	return httpDelete(`/api/weight/${id}`);
}

// Meditation
export async function createMeditation(req: CreateMeditationRequest): Promise<MeditationSession> {
	if (isTauri()) return tauriInvoke('cmd_create_meditation', { req });
	return httpPost('/api/meditation', req);
}

export async function listMeditations(from?: string, to?: string): Promise<MeditationSession[]> {
	if (isTauri()) return tauriInvoke('cmd_list_meditations', { from: from ?? null, to: to ?? null });
	const params = new URLSearchParams();
	if (from) params.set('from', from);
	if (to) params.set('to', to);
	const qs = params.toString();
	return httpGet(`/api/meditation${qs ? '?' + qs : ''}`);
}

export async function deleteMeditation(id: number): Promise<void> {
	if (isTauri()) return tauriInvoke('cmd_delete_meditation', { id });
	return httpDelete(`/api/meditation/${id}`);
}

// Feelings
export async function createFeeling(req: CreateFeelingRequest): Promise<FeelingEntry> {
	if (isTauri()) return tauriInvoke('cmd_create_feeling', { req });
	return httpPost('/api/feelings', req);
}

export async function listFeelings(from?: string, to?: string): Promise<FeelingEntry[]> {
	if (isTauri()) return tauriInvoke('cmd_list_feelings', { from: from ?? null, to: to ?? null });
	const params = new URLSearchParams();
	if (from) params.set('from', from);
	if (to) params.set('to', to);
	const qs = params.toString();
	return httpGet(`/api/feelings${qs ? '?' + qs : ''}`);
}

export async function deleteFeeling(id: number): Promise<void> {
	if (isTauri()) return tauriInvoke('cmd_delete_feeling', { id });
	return httpDelete(`/api/feelings/${id}`);
}

// AI Tips
export async function getLatestTip(): Promise<AiTip | null> {
	if (isTauri()) return tauriInvoke('cmd_get_latest_tip');
	return httpGet('/api/tips/latest');
}

export async function generateTips(): Promise<AiTip> {
	if (isTauri()) return tauriInvoke('cmd_generate_tips');
	return httpPost('/api/tips/generate');
}

export async function checkTipsStale(): Promise<boolean> {
	if (isTauri()) return tauriInvoke('cmd_check_tips_stale');
	return httpGet('/api/tips/stale');
}

// Settings
export async function getSettings(): Promise<AppSettings> {
	if (isTauri()) return tauriInvoke('cmd_get_settings');
	return httpGet('/api/settings');
}

export async function saveSettings(settings: AppSettings): Promise<void> {
	if (isTauri()) return tauriInvoke('cmd_save_settings', { settings });
	await httpPost('/api/settings', settings);
}

export async function hasApiKey(): Promise<boolean> {
	if (isTauri()) return tauriInvoke('cmd_has_api_key');
	const s = await getSettings();
	return !!s.api_key;
}
