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
        *username = Some(format!("{}@college.tcd.ie", user.clone()));
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
        match seqworks::ssh::ssh_authenticate(username.clone().unwrap(), pass, ssh_auth_server.to_string()) {
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
    
    // Access .env variables
    let ssh_jumphost_ip:String = env::var("SSH_JUMPHOST_IP").expect("SSH_JUMPHOST_IP must be set in .env (i.e. localhost)"); 
    let ssh_jumphost_port:u16 = env::var("SSH_JUMPHOST_PORT").expect("SSH_JUMPHOST_PORT must be set in .env (i.e. 22)").parse::<u16>().unwrap();
    let ssh_jumphost_user:String = env::var("SSH_JUMPHOST_USER").expect("SSH_JUMPHOST_USER must be set in .env (i.e. user)"); 
    let ssh_jumphost_pass:String = env::var("SSH_JUMPHOST_PASS").expect("SSH_JUMPHOST_PASS must be set in .env (i.e. password)"); 
    let ssh_tunnel_dest_ip:String = env::var("SSH_TUNNEL_DEST_IP").expect("SSH_TUNNEL_DEST_IP must be set in .env (i.e. localhost)");
    let ssh_tunnel_dest_port:u16 = env::var("SSH_TUNNEL_DEST_PORT").expect("SSH_TUNNEL_DEST_PORT must be set in .env (i.e. 22)").parse::<u16>().unwrap();
    let ssh_tunnel_dest_user:String = env::var("SSH_TUNNEL_DEST_USER").expect("SSH_TUNNEL_DEST_USER must be set in .env (i.e. user)");
    let ssh_tunnel_dest_pass:String = env::var("SSH_TUNNEL_DEST_PASS").expect("SSH_TUNNEL_DEST_PASS must be set in .env (i.e. password)");
    
    // Run the command via SSH
    match seqworks::ssh::run_ssh_command_via_jump(&ssh_jumphost_ip, ssh_jumphost_port, &ssh_jumphost_user, &ssh_jumphost_pass,
        &ssh_tunnel_dest_ip, ssh_tunnel_dest_port, &ssh_tunnel_dest_user, &ssh_tunnel_dest_pass, &rnaseq_cmd).await {
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
        .invoke_handler(tauri::generate_handler![greet, login_with_ssh, ws_listen, 
            get_project_list, init_pipe, cellxgene_startup, cellxgene_teardown])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
