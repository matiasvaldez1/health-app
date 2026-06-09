use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::claude;
use crate::db::DbPool;
use crate::models::*;
use crate::services;

#[derive(Deserialize)]
pub struct DateRange {
    pub from: Option<String>,
    pub to: Option<String>,
}

type ApiResult<T> = Result<Json<T>, (StatusCode, String)>;

fn err(e: String) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e)
}

pub async fn list_weights(
    Extension(pool): Extension<DbPool>,
    Query(range): Query<DateRange>,
) -> ApiResult<Vec<WeightEntry>> {
    services::list_weights(&pool, range.from, range.to)
        .map(Json)
        .map_err(err)
}

pub async fn create_weight(
    Extension(pool): Extension<DbPool>,
    Json(req): Json<CreateWeightRequest>,
) -> ApiResult<WeightEntry> {
    services::create_weight(&pool, req).map(Json).map_err(err)
}

pub async fn delete_weight(
    Extension(pool): Extension<DbPool>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    services::delete_weight(&pool, id).map(|_| StatusCode::NO_CONTENT).map_err(err)
}

pub async fn list_meditations(
    Extension(pool): Extension<DbPool>,
    Query(range): Query<DateRange>,
) -> ApiResult<Vec<MeditationSession>> {
    services::list_meditations(&pool, range.from, range.to)
        .map(Json)
        .map_err(err)
}

pub async fn create_meditation(
    Extension(pool): Extension<DbPool>,
    Json(req): Json<CreateMeditationRequest>,
) -> ApiResult<MeditationSession> {
    services::create_meditation(&pool, req).map(Json).map_err(err)
}

pub async fn delete_meditation(
    Extension(pool): Extension<DbPool>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    services::delete_meditation(&pool, id).map(|_| StatusCode::NO_CONTENT).map_err(err)
}

pub async fn list_feelings(
    Extension(pool): Extension<DbPool>,
    Query(range): Query<DateRange>,
) -> ApiResult<Vec<FeelingEntry>> {
    services::list_feelings(&pool, range.from, range.to)
        .map(Json)
        .map_err(err)
}

pub async fn create_feeling(
    Extension(pool): Extension<DbPool>,
    Json(req): Json<CreateFeelingRequest>,
) -> ApiResult<FeelingEntry> {
    services::create_feeling(&pool, req).map(Json).map_err(err)
}

pub async fn delete_feeling(
    Extension(pool): Extension<DbPool>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    services::delete_feeling(&pool, id).map(|_| StatusCode::NO_CONTENT).map_err(err)
}

pub async fn get_latest_tip(
    Extension(pool): Extension<DbPool>,
) -> ApiResult<Option<AiTip>> {
    services::get_latest_tip(&pool).map(Json).map_err(err)
}

pub async fn generate_tips(
    Extension(pool): Extension<DbPool>,
) -> ApiResult<AiTip> {
    let config_path = {
        let mut p = crate::db::get_db_path();
        p.pop();
        p.push("config.json");
        p
    };

    let settings: AppSettings = if config_path.exists() {
        let data = std::fs::read_to_string(&config_path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or(AppSettings { api_key: None, weight_goal: None })
    } else {
        AppSettings { api_key: None, weight_goal: None }
    };

    let api_key = settings.api_key.ok_or((
        StatusCode::BAD_REQUEST,
        "No API key configured".to_string(),
    ))?;

    let total = services::get_total_entries_count(&pool).map_err(err)?;
    if total == 0 {
        return Err((StatusCode::BAD_REQUEST, "No health data tracked yet".to_string()));
    }

    let response = claude::generate_tips(&api_key, &pool).await.map_err(err)?;
    let data_hash = services::compute_data_hash(&pool).map_err(err)?;
    services::save_tip(&pool, &data_hash, &response, total)
        .map(Json)
        .map_err(err)
}

pub async fn check_tips_stale(
    Extension(pool): Extension<DbPool>,
) -> ApiResult<bool> {
    let latest = services::get_latest_tip(&pool).map_err(err)?;
    match latest {
        None => Ok(Json(true)),
        Some(tip) => {
            if let Some(ref created) = tip.created_at {
                if let Ok(created_dt) = chrono::NaiveDateTime::parse_from_str(created, "%Y-%m-%d %H:%M:%S") {
                    let now = chrono::Local::now().naive_local();
                    if (now - created_dt).num_days() >= 3 {
                        return Ok(Json(true));
                    }
                }
            }
            let current_count = services::get_total_entries_count(&pool).map_err(err)?;
            Ok(Json(current_count - tip.entries_count >= 3))
        }
    }
}

pub async fn get_settings() -> ApiResult<AppSettings> {
    let config_path = {
        let mut p = crate::db::get_db_path();
        p.pop();
        p.push("config.json");
        p
    };

    let mut settings: AppSettings = if config_path.exists() {
        let data = std::fs::read_to_string(&config_path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or(AppSettings { api_key: None, weight_goal: None })
    } else {
        AppSettings { api_key: None, weight_goal: None }
    };

    if let Some(ref key) = settings.api_key {
        if key.len() > 8 {
            settings.api_key = Some(format!("{}...{}", &key[..4], &key[key.len() - 4..]));
        }
    }
    Ok(Json(settings))
}

pub async fn save_settings(
    Json(settings): Json<AppSettings>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let config_path = {
        let mut p = crate::db::get_db_path();
        p.pop();
        p.push("config.json");
        p
    };

    let mut current: AppSettings = if config_path.exists() {
        let existing = std::fs::read_to_string(&config_path).unwrap_or_default();
        serde_json::from_str(&existing).unwrap_or(AppSettings { api_key: None, weight_goal: None })
    } else {
        AppSettings { api_key: None, weight_goal: None }
    };

    if let Some(key) = settings.api_key {
        current.api_key = Some(key);
    }
    if settings.weight_goal.is_some() {
        current.weight_goal = settings.weight_goal;
    }

    let data = serde_json::to_string_pretty(&current).map_err(|e| err(e.to_string()))?;
    std::fs::write(&config_path, data).map_err(|e| err(e.to_string()))?;
    Ok(StatusCode::OK)
}

pub async fn update_weight(
    Extension(pool): Extension<DbPool>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateWeightRequest>,
) -> ApiResult<WeightEntry> {
    services::update_weight(&pool, id, req).map(Json).map_err(err)
}

pub async fn update_meditation(
    Extension(pool): Extension<DbPool>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateMeditationRequest>,
) -> ApiResult<MeditationSession> {
    services::update_meditation(&pool, id, req).map(Json).map_err(err)
}

pub async fn update_feeling(
    Extension(pool): Extension<DbPool>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateFeelingRequest>,
) -> ApiResult<FeelingEntry> {
    services::update_feeling(&pool, id, req).map(Json).map_err(err)
}

pub async fn export_data(
    Extension(pool): Extension<DbPool>,
) -> ApiResult<ExportData> {
    services::export_all_data(&pool).map(Json).map_err(err)
}
