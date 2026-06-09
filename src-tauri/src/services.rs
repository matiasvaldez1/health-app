use crate::db::DbPool;
use crate::models::*;
use rusqlite::params;
use sha2::{Digest, Sha256};

pub fn create_weight(pool: &DbPool, req: CreateWeightRequest) -> Result<WeightEntry, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO weight_entries (weight_kg, date, notes) VALUES (?1, ?2, ?3)",
        params![req.weight_kg, req.date, req.notes],
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    get_weight_by_id(pool, id)
}

fn get_weight_by_id(pool: &DbPool, id: i64) -> Result<WeightEntry, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    conn.query_row(
        "SELECT id, weight_kg, date, notes, created_at, updated_at FROM weight_entries WHERE id = ?1",
        params![id],
        |row| {
            Ok(WeightEntry {
                id: Some(row.get(0)?),
                weight_kg: row.get(1)?,
                date: row.get(2)?,
                notes: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

pub fn list_weights(pool: &DbPool, from: Option<String>, to: Option<String>) -> Result<Vec<WeightEntry>, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    let mut sql = "SELECT id, weight_kg, date, notes, created_at, updated_at FROM weight_entries".to_string();
    let mut conditions = Vec::new();

    if from.is_some() {
        conditions.push("date >= ?1");
    }
    if to.is_some() {
        conditions.push(if from.is_some() { "date <= ?2" } else { "date <= ?1" });
    }

    if !conditions.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&conditions.join(" AND "));
    }
    sql.push_str(" ORDER BY date DESC");

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(ref f) = from {
        param_values.push(Box::new(f.clone()));
    }
    if let Some(ref t) = to {
        param_values.push(Box::new(t.clone()));
    }

    let params_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

    let rows = stmt
        .query_map(params_refs.as_slice(), |row| {
            Ok(WeightEntry {
                id: Some(row.get(0)?),
                weight_kg: row.get(1)?,
                date: row.get(2)?,
                notes: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;

    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

pub fn delete_weight(pool: &DbPool, id: i64) -> Result<(), String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM weight_entries WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn create_meditation(pool: &DbPool, req: CreateMeditationRequest) -> Result<MeditationSession, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO meditation_sessions (date, time, duration_min, notes) VALUES (?1, ?2, ?3, ?4)",
        params![req.date, req.time, req.duration_min, req.notes],
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    get_meditation_by_id(pool, id)
}

fn get_meditation_by_id(pool: &DbPool, id: i64) -> Result<MeditationSession, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    conn.query_row(
        "SELECT id, date, time, duration_min, notes, created_at, updated_at FROM meditation_sessions WHERE id = ?1",
        params![id],
        |row| {
            Ok(MeditationSession {
                id: Some(row.get(0)?),
                date: row.get(1)?,
                time: row.get(2)?,
                duration_min: row.get(3)?,
                notes: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

pub fn list_meditations(pool: &DbPool, from: Option<String>, to: Option<String>) -> Result<Vec<MeditationSession>, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    let mut sql = "SELECT id, date, time, duration_min, notes, created_at, updated_at FROM meditation_sessions".to_string();
    let mut conditions = Vec::new();

    if from.is_some() {
        conditions.push("date >= ?1");
    }
    if to.is_some() {
        conditions.push(if from.is_some() { "date <= ?2" } else { "date <= ?1" });
    }

    if !conditions.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&conditions.join(" AND "));
    }
    sql.push_str(" ORDER BY date DESC, time DESC");

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(ref f) = from {
        param_values.push(Box::new(f.clone()));
    }
    if let Some(ref t) = to {
        param_values.push(Box::new(t.clone()));
    }

    let params_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

    let rows = stmt
        .query_map(params_refs.as_slice(), |row| {
            Ok(MeditationSession {
                id: Some(row.get(0)?),
                date: row.get(1)?,
                time: row.get(2)?,
                duration_min: row.get(3)?,
                notes: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;

    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

pub fn delete_meditation(pool: &DbPool, id: i64) -> Result<(), String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM meditation_sessions WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn create_feeling(pool: &DbPool, req: CreateFeelingRequest) -> Result<FeelingEntry, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO feeling_entries (date, content, mood_score) VALUES (?1, ?2, ?3)",
        params![req.date, req.content, req.mood_score],
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    get_feeling_by_id(pool, id)
}

fn get_feeling_by_id(pool: &DbPool, id: i64) -> Result<FeelingEntry, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    conn.query_row(
        "SELECT id, date, content, mood_score, created_at, updated_at FROM feeling_entries WHERE id = ?1",
        params![id],
        |row| {
            Ok(FeelingEntry {
                id: Some(row.get(0)?),
                date: row.get(1)?,
                content: row.get(2)?,
                mood_score: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

pub fn list_feelings(pool: &DbPool, from: Option<String>, to: Option<String>) -> Result<Vec<FeelingEntry>, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    let mut sql = "SELECT id, date, content, mood_score, created_at, updated_at FROM feeling_entries".to_string();
    let mut conditions = Vec::new();

    if from.is_some() {
        conditions.push("date >= ?1");
    }
    if to.is_some() {
        conditions.push(if from.is_some() { "date <= ?2" } else { "date <= ?1" });
    }

    if !conditions.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&conditions.join(" AND "));
    }
    sql.push_str(" ORDER BY date DESC");

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(ref f) = from {
        param_values.push(Box::new(f.clone()));
    }
    if let Some(ref t) = to {
        param_values.push(Box::new(t.clone()));
    }

    let params_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

    let rows = stmt
        .query_map(params_refs.as_slice(), |row| {
            Ok(FeelingEntry {
                id: Some(row.get(0)?),
                date: row.get(1)?,
                content: row.get(2)?,
                mood_score: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;

    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

pub fn delete_feeling(pool: &DbPool, id: i64) -> Result<(), String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM feeling_entries WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_latest_tip(pool: &DbPool) -> Result<Option<AiTip>, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    let result = conn.query_row(
        "SELECT id, data_hash, response, entries_count, created_at FROM ai_tips ORDER BY created_at DESC LIMIT 1",
        [],
        |row| {
            Ok(AiTip {
                id: Some(row.get(0)?),
                data_hash: row.get(1)?,
                response: row.get(2)?,
                entries_count: row.get(3)?,
                created_at: row.get(4)?,
            })
        },
    );

    match result {
        Ok(tip) => Ok(Some(tip)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

pub fn save_tip(pool: &DbPool, data_hash: &str, response: &str, entries_count: i32) -> Result<AiTip, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO ai_tips (data_hash, response, entries_count) VALUES (?1, ?2, ?3)",
        params![data_hash, response, entries_count],
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    conn.query_row(
        "SELECT id, data_hash, response, entries_count, created_at FROM ai_tips WHERE id = ?1",
        params![id],
        |row| {
            Ok(AiTip {
                id: Some(row.get(0)?),
                data_hash: row.get(1)?,
                response: row.get(2)?,
                entries_count: row.get(3)?,
                created_at: row.get(4)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

pub fn get_total_entries_count(pool: &DbPool) -> Result<i32, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    let count: i64 = conn
        .query_row(
            "SELECT (SELECT COUNT(*) FROM weight_entries) + (SELECT COUNT(*) FROM meditation_sessions) + (SELECT COUNT(*) FROM feeling_entries)",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    Ok(count as i32)
}

pub fn update_weight(pool: &DbPool, id: i64, req: UpdateWeightRequest) -> Result<WeightEntry, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    let mut sets = Vec::new();
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    let mut idx = 1;

    if let Some(ref v) = req.weight_kg { sets.push(format!("weight_kg = ?{}", idx)); param_values.push(Box::new(*v)); idx += 1; }
    if let Some(ref v) = req.date { sets.push(format!("date = ?{}", idx)); param_values.push(Box::new(v.clone())); idx += 1; }
    if let Some(ref v) = req.notes { sets.push(format!("notes = ?{}", idx)); param_values.push(Box::new(v.clone())); idx += 1; }

    if sets.is_empty() { return get_weight_by_id(pool, id); }

    sets.push(format!("updated_at = datetime('now')"));
    let sql = format!("UPDATE weight_entries SET {} WHERE id = ?{}", sets.join(", "), idx);
    param_values.push(Box::new(id));

    let params_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
    conn.execute(&sql, params_refs.as_slice()).map_err(|e| e.to_string())?;
    get_weight_by_id(pool, id)
}

pub fn update_meditation(pool: &DbPool, id: i64, req: UpdateMeditationRequest) -> Result<MeditationSession, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    let mut sets = Vec::new();
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    let mut idx = 1;

    if let Some(ref v) = req.date { sets.push(format!("date = ?{}", idx)); param_values.push(Box::new(v.clone())); idx += 1; }
    if let Some(ref v) = req.time { sets.push(format!("time = ?{}", idx)); param_values.push(Box::new(v.clone())); idx += 1; }
    if let Some(ref v) = req.duration_min { sets.push(format!("duration_min = ?{}", idx)); param_values.push(Box::new(*v)); idx += 1; }
    if let Some(ref v) = req.notes { sets.push(format!("notes = ?{}", idx)); param_values.push(Box::new(v.clone())); idx += 1; }

    if sets.is_empty() { return get_meditation_by_id(pool, id); }

    sets.push(format!("updated_at = datetime('now')"));
    let sql = format!("UPDATE meditation_sessions SET {} WHERE id = ?{}", sets.join(", "), idx);
    param_values.push(Box::new(id));

    let params_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
    conn.execute(&sql, params_refs.as_slice()).map_err(|e| e.to_string())?;
    get_meditation_by_id(pool, id)
}

pub fn update_feeling(pool: &DbPool, id: i64, req: UpdateFeelingRequest) -> Result<FeelingEntry, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    let mut sets = Vec::new();
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    let mut idx = 1;

    if let Some(ref v) = req.date { sets.push(format!("date = ?{}", idx)); param_values.push(Box::new(v.clone())); idx += 1; }
    if let Some(ref v) = req.content { sets.push(format!("content = ?{}", idx)); param_values.push(Box::new(v.clone())); idx += 1; }
    if let Some(ref v) = req.mood_score { sets.push(format!("mood_score = ?{}", idx)); param_values.push(Box::new(*v)); idx += 1; }

    if sets.is_empty() { return get_feeling_by_id(pool, id); }

    sets.push(format!("updated_at = datetime('now')"));
    let sql = format!("UPDATE feeling_entries SET {} WHERE id = ?{}", sets.join(", "), idx);
    param_values.push(Box::new(id));

    let params_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
    conn.execute(&sql, params_refs.as_slice()).map_err(|e| e.to_string())?;
    get_feeling_by_id(pool, id)
}

pub fn export_all_data(pool: &DbPool) -> Result<ExportData, String> {
    let weights = list_weights(pool, None, None)?;
    let meditations = list_meditations(pool, None, None)?;
    let feelings = list_feelings(pool, None, None)?;
    Ok(ExportData { weights, meditations, feelings })
}

pub fn compute_data_hash(pool: &DbPool) -> Result<String, String> {
    let weights = list_weights(pool, None, None)?;
    let meditations = list_meditations(pool, None, None)?;
    let feelings = list_feelings(pool, None, None)?;

    let mut hasher = Sha256::new();
    hasher.update(serde_json::to_string(&weights).unwrap_or_default().as_bytes());
    hasher.update(serde_json::to_string(&meditations).unwrap_or_default().as_bytes());
    hasher.update(serde_json::to_string(&feelings).unwrap_or_default().as_bytes());

    Ok(hex::encode(hasher.finalize()))
}
