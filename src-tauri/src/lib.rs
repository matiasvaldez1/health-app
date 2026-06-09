mod claude;
mod commands;
mod db;
mod handlers;
mod models;
mod server;
mod services;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let pool = db::init_pool();
    db::run_migrations(&pool);

    let axum_pool = pool.clone();

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
        rt.block_on(server::start_server(axum_pool, 3333));
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(pool)
        .invoke_handler(tauri::generate_handler![
            cmd_create_weight,
            cmd_list_weights,
            cmd_delete_weight,
            cmd_create_meditation,
            cmd_list_meditations,
            cmd_delete_meditation,
            cmd_create_feeling,
            cmd_list_feelings,
            cmd_delete_feeling,
            cmd_get_latest_tip,
            cmd_generate_tips,
            cmd_check_tips_stale,
            cmd_update_weight,
            cmd_update_meditation,
            cmd_update_feeling,
            cmd_export_data,
            cmd_get_settings,
            cmd_save_settings,
            cmd_has_api_key,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
