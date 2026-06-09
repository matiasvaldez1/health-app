export interface WeightEntry {
	id?: number;
	weight_kg: number;
	date: string;
	notes?: string;
	created_at?: string;
	updated_at?: string;
}

export interface MeditationSession {
	id?: number;
	date: string;
	time: string;
	duration_min: number;
	notes?: string;
	created_at?: string;
	updated_at?: string;
}

export interface FeelingEntry {
	id?: number;
	date: string;
	content: string;
	mood_score?: number;
	created_at?: string;
	updated_at?: string;
}

export interface AiTip {
	id?: number;
	data_hash: string;
	response: string;
	entries_count: number;
	created_at?: string;
}

export interface CreateWeightRequest {
	weight_kg: number;
	date: string;
	notes?: string;
}

export interface CreateMeditationRequest {
	date: string;
	time: string;
	duration_min: number;
	notes?: string;
}

export interface CreateFeelingRequest {
	date: string;
	content: string;
	mood_score?: number;
}

export interface UpdateWeightRequest {
	weight_kg?: number;
	date?: string;
	notes?: string;
}

export interface UpdateMeditationRequest {
	date?: string;
	time?: string;
	duration_min?: number;
	notes?: string;
}

export interface UpdateFeelingRequest {
	date?: string;
	content?: string;
	mood_score?: number;
}

export interface ExportData {
	weights: WeightEntry[];
	meditations: MeditationSession[];
	feelings: FeelingEntry[];
}

export interface AppSettings {
	api_key?: string;
	weight_goal?: number;
}
