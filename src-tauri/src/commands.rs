use crate::claude;
use crate::db::DbPool;
use crate::models::*;
use crate::services;
use std::path::PathBuf;

fn config_path() -> PathBuf {
    let mut path = crate::db::get_db_path();
    path.pop();
    path.push("config.json");
    path
}

fn read_settings() -> AppSettings {
    let path = config_path();
    if path.exists() {
        let data = std::fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or(AppSettings { api_key: None, weight_goal: None })
    } else {
        AppSettings { api_key: None, weight_goal: None }
    }
}

fn write_settings(settings: &AppSettings) -> Result<(), String> {
    let path = config_path();
    let data = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    std::fs::write(&path, data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn cmd_create_weight(pool: tauri::State<'_, DbPool>, req: CreateWeightRequest) -> Result<WeightEntry, String> {
    services::create_weight(&pool, req)
}

#[tauri::command]
pub fn cmd_list_weights(pool: tauri::State<'_, DbPool>, from: Option<String>, to: Option<String>) -> Result<Vec<WeightEntry>, String> {
    services::list_weights(&pool, from, to)
}

#[tauri::command]
pub fn cmd_delete_weight(pool: tauri::State<'_, DbPool>, id: i64) -> Result<(), String> {
    services::delete_weight(&pool, id)
}

#[tauri::command]
pub fn cmd_create_meditation(pool: tauri::State<'_, DbPool>, req: CreateMeditationRequest) -> Result<MeditationSession, String> {
    services::create_meditation(&pool, req)
}

#[tauri::command]
pub fn cmd_list_meditations(pool: tauri::State<'_, DbPool>, from: Option<String>, to: Option<String>) -> Result<Vec<MeditationSession>, String> {
    services::list_meditations(&pool, from, to)
}

#[tauri::command]
pub fn cmd_delete_meditation(pool: tauri::State<'_, DbPool>, id: i64) -> Result<(), String> {
    services::delete_meditation(&pool, id)
}

#[tauri::command]
pub fn cmd_create_feeling(pool: tauri::State<'_, DbPool>, req: CreateFeelingRequest) -> Result<FeelingEntry, String> {
    services::create_feeling(&pool, req)
}

#[tauri::command]
pub fn cmd_list_feelings(pool: tauri::State<'_, DbPool>, from: Option<String>, to: Option<String>) -> Result<Vec<FeelingEntry>, String> {
    services::list_feelings(&pool, from, to)
}

#[tauri::command]
pub fn cmd_delete_feeling(pool: tauri::State<'_, DbPool>, id: i64) -> Result<(), String> {
    services::delete_feeling(&pool, id)
}

#[tauri::command]
pub fn cmd_get_latest_tip(pool: tauri::State<'_, DbPool>) -> Result<Option<AiTip>, String> {
    services::get_latest_tip(&pool)
}

#[tauri::command]
pub async fn cmd_generate_tips(pool: tauri::State<'_, DbPool>) -> Result<AiTip, String> {
    let settings = read_settings();
    let api_key = settings.api_key.ok_or("No API key configured. Go to Settings to add your Claude API key.")?;

    let total = services::get_total_entries_count(&pool)?;
    if total == 0 {
        return Err("No health data tracked yet. Start logging your weight, meditation, or feelings first!".to_string());
    }

    let response = claude::generate_tips(&api_key, &pool).await?;
    let data_hash = services::compute_data_hash(&pool)?;
    services::save_tip(&pool, &data_hash, &response, total)
}

#[tauri::command]
pub fn cmd_check_tips_stale(pool: tauri::State<'_, DbPool>) -> Result<bool, String> {
    let latest = services::get_latest_tip(&pool)?;
    match latest {
        None => Ok(true),
        Some(tip) => {
            if let Some(ref created) = tip.created_at {
                let created_dt = chrono::NaiveDateTime::parse_from_str(created, "%Y-%m-%d %H:%M:%S")
                    .map_err(|e| e.to_string())?;
                let now = chrono::Local::now().naive_local();
                let days_old = (now - created_dt).num_days();
                if days_old >= 3 {
                    return Ok(true);
                }
            }

            let current_count = services::get_total_entries_count(&pool)?;
            let diff = current_count - tip.entries_count;
            Ok(diff >= 3)
        }
    }
}

#[tauri::command]
pub fn cmd_get_settings() -> Result<AppSettings, String> {
    let mut settings = read_settings();
    if let Some(ref key) = settings.api_key {
        if key.len() > 8 {
            settings.api_key = Some(format!("{}...{}", &key[..4], &key[key.len() - 4..]));
        }
    }
    Ok(settings)
}

#[tauri::command]
pub fn cmd_save_settings(settings: AppSettings) -> Result<(), String> {
    let mut current = read_settings();
    if let Some(key) = settings.api_key {
        current.api_key = Some(key);
    }
    if settings.weight_goal.is_some() {
        current.weight_goal = settings.weight_goal;
    }
    write_settings(&current)
}

#[tauri::command]
pub fn cmd_has_api_key() -> Result<bool, String> {
    let settings = read_settings();
    Ok(settings.api_key.is_some())
}

#[tauri::command]
pub fn cmd_update_weight(pool: tauri::State<'_, DbPool>, id: i64, req: UpdateWeightRequest) -> Result<WeightEntry, String> {
    services::update_weight(&pool, id, req)
}

#[tauri::command]
pub fn cmd_update_meditation(pool: tauri::State<'_, DbPool>, id: i64, req: UpdateMeditationRequest) -> Result<MeditationSession, String> {
    services::update_meditation(&pool, id, req)
}

#[tauri::command]
pub fn cmd_update_feeling(pool: tauri::State<'_, DbPool>, id: i64, req: UpdateFeelingRequest) -> Result<FeelingEntry, String> {
    services::update_feeling(&pool, id, req)
}

#[tauri::command]
pub fn cmd_export_data(pool: tauri::State<'_, DbPool>) -> Result<ExportData, String> {
    services::export_all_data(&pool)
}
