use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::path::PathBuf;

pub type DbPool = Pool<SqliteConnectionManager>;

pub fn get_db_path() -> PathBuf {
    let mut path = dirs_next().unwrap_or_else(|| PathBuf::from("."));
    path.push("health-tracker.db");
    path
}

fn dirs_next() -> Option<PathBuf> {
    if let Some(data_dir) = dirs::data_local_dir() {
        let app_dir = data_dir.join("com.matiasvaldez.health-tracker");
        std::fs::create_dir_all(&app_dir).ok();
        Some(app_dir)
    } else {
        None
    }
}

pub fn init_pool() -> DbPool {
    let db_path = get_db_path();
    let manager = SqliteConnectionManager::file(&db_path);
    let pool = Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("failed to create db pool");

    let conn = pool.get().expect("failed to get db connection");
    conn.execute_batch("PRAGMA journal_mode=WAL;")
        .expect("failed to set WAL mode");
    conn.execute_batch("PRAGMA foreign_keys=ON;")
        .expect("failed to enable foreign keys");

    pool
}

pub fn run_migrations(pool: &DbPool) {
    let conn = pool.get().expect("failed to get db connection for migrations");
    let sql = include_str!("../migrations/001_init.sql");
    conn.execute_batch(sql)
        .expect("failed to run migrations");
}
