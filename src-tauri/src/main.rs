// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use seqworks;
use seqworks::app_state::AppState;
use tauri::{AppHandle, State, Manager};
use serde::Deserialize;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn login_with_ssh(user: String, pass: String, state: State<'_, AppState>) -> bool {
    if user == "user" && pass == "123" {
        println!("SSH login result for {}: success", user);
        let mut username = state.username.lock().unwrap(); // Update the shared state with the WebSocket URL
        *username = Some(user.clone());
        match seqworks::socket::register(state.clone()) {
            Ok(ws_url) => {
                println!("WebSocket URL: {}", ws_url);
            }
            Err(e) => {
                eprintln!("Failed to register: {}", e);
            }
        }
        true // Authentication successful
    } else {
        match seqworks::ssh::ssh_authenticate(user.clone(), pass, "localhost:2222".to_string()) {
            Ok(pass) => {
                println!("SSH login result for {}: {}", user, pass);
                true
            }
            Err(e) => {
                eprintln!("Login Failed: {}", e);
                false
            }
        }
    }
}

#[tauri::command]
async fn ws_listen(app_handle: AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    seqworks::socket::ws_connect(app_handle, state).await
}

#[tauri::command]
async fn get_project_list(pipe_type: &str) -> Result<Vec<String>, Vec<String>> {
    let project_list: Vec<String> = match seqworks::ftp_cmds::get_dirs(pipe_type) {
        Ok(project_list ) => {
            project_list
        }
        _ => {
            Vec::new()
        }
    };
    Ok(project_list)
}

#[derive(Deserialize)]
struct AppParamsWrapper {
    params: AppParamsEnum,
}

#[derive(Deserialize)]
enum AppParamsEnum {
    AppParams(seqworks::pipelines::AppParams),
    AppSCParams(seqworks::pipelines::AppSCParams),
}


#[tauri::command]
async fn init_pipe(wrapper: AppParamsWrapper, state: State<'_, AppState>, app_handle: AppHandle) -> Result<String, String> {
    let rnaseq_cmd: String = match wrapper.params {
        AppParamsEnum::AppParams(params) => {
            match seqworks::pipelines::parse_bulk_params(params, state) {
                Ok(rnaseq_cmd) => format!("Bulk RNAseq command: {}", rnaseq_cmd),
                Err(e) => {
                    eprintln!("Failed to get Bulk RNAseq command: {}", e);
                    app_handle.emit("init_result", "Failed to parse parameters, check inputs").unwrap();
                    return Err(format!("Failed to initialise pipeline: {}", e));
                }
            }
        }
        AppParamsEnum::AppSCParams(params) => {
            match seqworks::pipelines::parse_sc_params(params, state) {
                Ok(rnaseq_cmd ) => format!("Single cell RNAseq command: {}", rnaseq_cmd),
                Err(e) => {
                    eprintln!("Failed to get Single Cell RNAseq command: {}", e);
                    app_handle.emit("init_result", "Failed to parse parameters, check inputs").unwrap();
                    return Err(format!("Failed to initialise pipeline: {}", e));
                }
            }    
        }
    };
    
    // Run the command via SSH
    match seqworks::ssh::run_ssh_command_via_jump("localhost", 2222, "root", "password", "172.17.0.2", 22, "student", "student1", &rnaseq_cmd).await {
        //gen153055.gen.tcd.ie  naga 
        Ok(output) => {
            println!("SSH command output: {}", output);
            app_handle.emit("init_result", "Initiated Pipeline, please wait for completion email").unwrap();
            Ok(rnaseq_cmd)
        },
        Err(e) => {
            eprintln!("Failed to run SSH command: {}", e);
            app_handle.emit("init_result", "Failed to connect to Reaper").unwrap();
            Err(format!("Failed to initialise pipeline: {}", e))
        }
    }
}

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
        .invoke_handler(tauri::generate_handler![greet, login_with_ssh, ws_listen, get_project_list, init_pipe])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
