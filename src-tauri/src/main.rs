// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use seqworks::commands;
use seqworks::app_state::AppState;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            app.handle().plugin(tauri_plugin_updater::Builder::new().build())?;
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_websocket::init())
        .manage(AppState::new())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![commands::login_with_ssh, commands::ws_start, 
            commands::get_project_list, commands::init_pipe, commands::cellxgene_startup, commands::cellxgene_teardown])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
