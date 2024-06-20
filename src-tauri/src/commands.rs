use crate::{ssh, pipelines, socket, app_state, ftp_cmds};
use tauri::{AppHandle, State, Manager};
include!(concat!("../env_vars.rs"));


#[tauri::command]
pub fn login_with_ssh(user: String, pass: String, state: State<'_, app_state::AppState>) -> bool {
    let mut username = state.username.lock().unwrap(); // Update the shared state with the WebSocket URL
        *username = Some(user.clone());
    if user == "user" && pass == "123" {
        println!("SSH login result for {}: success", user);
        match socket::register(state.clone()) {
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
        match ssh::ssh_authenticate(user_domain, pass, SSH_JUMPHOST.to_string()) {
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
pub async fn ws_start(app_handle: AppHandle, state: State<'_, app_state::AppState>) -> Result<(), String> {
    socket::start_websocket(app_handle, state).await
}

#[tauri::command]
pub async fn get_project_list(pipe_type: &str) -> Result<Vec<String>, Vec<String>> {
    let project_list: Vec<String> = match ftp_cmds::get_dirs(pipe_type) {
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
pub async fn init_pipe(wrapper: app_state::AppParamsWrapper, state: State<'_, app_state::AppState>, app_handle: AppHandle) -> Result<String, String> {
    let rnaseq_cmd: String = match wrapper.params {
        app_state::AppParamsEnum::AppParams(params) => {
            match pipelines::parse_bulk_params(params, state) {
                Ok(rnaseq_cmd) => rnaseq_cmd,
                Err(e) => {
                    eprintln!("Failed to get Bulk RNAseq command: {}", e);
                    app_handle.emit("init_result", "Failed to parse parameters, check inputs").unwrap();
                    return Err(format!("Failed to initialise pipeline: {}", e));
                }
            }
        }
        app_state::AppParamsEnum::AppSCParams(params) => {
            match pipelines::parse_sc_params(params, state) {
                Ok(rnaseq_cmd ) => rnaseq_cmd.to_string(),
                Err(e) => {
                    eprintln!("Failed to get Single Cell RNAseq command: {}", e);
                    app_handle.emit("init_result", "Failed to parse parameters, check inputs").unwrap();
                    return Err(format!("Failed to initialise pipeline: {}", e));
                }
            }    
        }
    };
 
    match ssh::ssh_chain(&rnaseq_cmd).await {
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
pub async fn cellxgene_startup(params: ssh::CxgParams, app_handle: AppHandle) -> Result<String, String> {
    match ssh::start_cellxgene(params).await {
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
pub async fn cellxgene_teardown(params: ssh::CxgParams, app_handle: AppHandle) -> Result<String, String> {
    match ssh::stop_cellxgene(params).await {
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