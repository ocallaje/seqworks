// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use seqworks;
use seqworks::app_state::AppState;
use tauri::{AppHandle, State, Manager};

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

#[tauri::command]
async fn init_bulk(app_params: seqworks::pipelines::AppParams, state: State<'_, AppState>, app_handle: AppHandle) -> Result<String, String> {
    let rnaseq_cmd: String = match seqworks::pipelines::parse_bulk_params(app_params, state) {
      Ok(rnaseq_cmd ) => {
        println!("RNAseq command: {}", rnaseq_cmd);

        // Run the command via SSH
        //match seqworks::ssh::run_ssh_command("localhost:2222".to_string(), "root".to_string(), "password".to_string(), rnaseq_cmd.clone()).await {
        match seqworks::ssh::run_ssh_command_via_jump("localhost", 2222, "root", "password", "172.17.0.2", 22, "student", "student1", &rnaseq_cmd).await {
            //gen153055.gen.tcd.ie  naga 
            Ok(output) => {
                println!("SSH command output: {}", output);
                app_handle.emit("init_result", "Initiated Pipeline, please wait for completion email").unwrap();
                rnaseq_cmd
            },
            Err(e) => {
                eprintln!("Failed to run SSH command: {}", e);
                app_handle.emit("init_result", "Failed to connect to Reaper").unwrap();
                format!("Failed to initialise pipeline: {}", e)
            }
        }
      }
      Err(e) => {
        eprintln!("Failed to parse parameters: {}", e);
        app_handle.emit("init_result", "Initialisation Failed").unwrap();
        format!("Failed to initialise pipeline: {}", e)
      }
    };
    Ok(rnaseq_cmd)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_websocket::init())
        .manage(AppState::new())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, login_with_ssh, ws_listen, get_project_list, init_bulk])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
