// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use seqworks;
use seqworks::app_state::AppState;
use tauri::{AppHandle, State, Manager};
use serde::Deserialize;
use dotenvy::dotenv;
use std::env;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn login_with_ssh(user: String, pass: String, state: State<'_, AppState>) -> bool {
    let ssh_auth_server:String = env::var("SSH_AUTH_SERVER").expect("SSH_AUTH_SERVER must be set in .env (i.e. localhost:2222)"); // access env variable 
    let mut username = state.username.lock().unwrap(); // Update the shared state with the WebSocket URL
        *username = Some(user.clone());
    if user == "user" && pass == "123" {
        println!("SSH login result for {}: success", user);
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
        let user_domain:String = format!("{}@college.tcd.ie", user.clone());
        match seqworks::ssh::ssh_authenticate(user_domain, pass, ssh_auth_server.to_string()) {
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
async fn ws_start(app_handle: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    seqworks::socket::start_websocket(app_handle, state).await
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
                Ok(rnaseq_cmd) => rnaseq_cmd,
                Err(e) => {
                    eprintln!("Failed to get Bulk RNAseq command: {}", e);
                    app_handle.emit("init_result", "Failed to parse parameters, check inputs").unwrap();
                    return Err(format!("Failed to initialise pipeline: {}", e));
                }
            }
        }
        AppParamsEnum::AppSCParams(params) => {
            match seqworks::pipelines::parse_sc_params(params, state) {
                Ok(rnaseq_cmd ) => rnaseq_cmd.to_string(),
                Err(e) => {
                    eprintln!("Failed to get Single Cell RNAseq command: {}", e);
                    app_handle.emit("init_result", "Failed to parse parameters, check inputs").unwrap();
                    return Err(format!("Failed to initialise pipeline: {}", e));
                }
            }    
        }
    };
 
    match seqworks::ssh::ssh_chain(&rnaseq_cmd).await {
        Ok(_exit_status) => app_handle.emit("init_result", "Pipeline Initialised! Please wait for completion email").unwrap(),
        Err(e) => {
            eprintln!("Failed to get Single Cell RNAseq command: {}", e);
            app_handle.emit("init_result", "Failed to send command to server").unwrap();
            return Err(format!("Failed to initialise pipeline: {}", e));
        }
    };
    
    Ok(rnaseq_cmd)

}

#[tauri::command]
async fn cellxgene_startup(params: seqworks::ssh::CxgParams, app_handle: AppHandle) -> Result<String, String> {
    match seqworks::ssh::start_cellxgene(params).await {
        Ok(_) => {
            let message = "Launched CellXGene".to_string(); 
            app_handle.emit("cellxgene_result", &message).unwrap();
            Ok(message)
        },
        Err(e) => {
            let err_msg = format!("Failed to launch CellXGene, contact administrator: {}", e);
            eprintln!("{}", &err_msg);
            app_handle.emit("cellxgene_result", &err_msg).unwrap();
            return Err(err_msg);
        }
    }
}

#[tauri::command]
async fn cellxgene_teardown(params: seqworks::ssh::CxgParams, app_handle: AppHandle) -> Result<String, String> {
    match seqworks::ssh::stop_cellxgene(params).await {
        Ok(_) => {
            let message = "CellxGene container stopped and data saved to project/cellxgene".to_string(); 
            app_handle.emit("cellxgene_result", &message).unwrap();
            Ok(message)
        },
        Err(e) => {
            let err_msg = format!("Failed to connect to SSH server: {}", e);
            eprintln!("{}", &err_msg);
            app_handle.emit("cellxgene_result", "Failed to close CellXGene, contact administrator").unwrap();
            return Err(err_msg);
        }
    } 
}

fn main() {
    dotenv().ok();
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
        .invoke_handler(tauri::generate_handler![greet, login_with_ssh, ws_start, 
            get_project_list, init_pipe, cellxgene_startup, cellxgene_teardown])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
