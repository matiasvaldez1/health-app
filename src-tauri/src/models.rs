use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeightEntry {
    pub id: Option<i64>,
    pub weight_kg: f64,
    pub date: String,
    pub notes: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeditationSession {
    pub id: Option<i64>,
    pub date: String,
    pub time: String,
    pub duration_min: i32,
    pub notes: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeelingEntry {
    pub id: Option<i64>,
    pub date: String,
    pub content: String,
    pub mood_score: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiTip {
    pub id: Option<i64>,
    pub data_hash: String,
    pub response: String,
    pub entries_count: i32,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWeightRequest {
    pub weight_kg: f64,
    pub date: String,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMeditationRequest {
    pub date: String,
    pub time: String,
    pub duration_min: i32,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFeelingRequest {
    pub date: String,
    pub content: String,
    pub mood_score: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWeightRequest {
    pub weight_kg: Option<f64>,
    pub date: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMeditationRequest {
    pub date: Option<String>,
    pub time: Option<String>,
    pub duration_min: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFeelingRequest {
    pub date: Option<String>,
    pub content: Option<String>,
    pub mood_score: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportData {
    pub weights: Vec<WeightEntry>,
    pub meditations: Vec<MeditationSession>,
    pub feelings: Vec<FeelingEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub api_key: Option<String>,
    pub weight_goal: Option<f64>,
}
