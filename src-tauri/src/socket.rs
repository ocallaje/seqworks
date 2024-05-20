use crate::app_state::AppState;
//use futures::SinkExt;
use futures::StreamExt;
use tauri::{AppHandle, Manager, State};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

// Private Structs
#[derive(serde::Serialize)]
struct RegisterRequest {
    user_id: u32,
    topic: String,
}

#[derive(serde::Deserialize)]
struct RegisterResponse {
    url: String,
}

#[derive(Clone, serde::Serialize)]
pub struct WebSocketMessage {
    message: String,
}

// Public Functions
pub fn register(state: State<'_, AppState>) -> Result<String, String> {
    let register_body = RegisterRequest {
        user_id: 1,
        topic: "nextflow".to_string(),
    };
    let register_body_json = serde_json::to_string(&register_body)
        .map_err(|e| format!("Failed to serialize request body to JSON: {}", e))?;

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("http://localhost:7777/register")
        .header("Content-Type", "application/json")
        .body(register_body_json)
        .send()
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    // Parse response to get websocket URL
    if response.status().is_success() {
        let response_body = response
            .text()
            .map_err(|e| format!("Failed to read response from http register request: {}", e))?;
        let register_response: RegisterResponse = serde_json::from_str(&response_body)
            .map_err(|e| format!("Failed to parse response JSON: {}", e))?;
        let url = register_response.url;
        let mut ws_url = state.ws_url.lock().unwrap(); // Update the shared state with the WebSocket URL
        *ws_url = Some(url.clone());
        Ok(url)
    } else {
        Err(format!(
            "Request failed with status code: {}",
            response.status()
        ))
    }
}

pub async fn ws_connect(
    app_handle: AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    // Get websocket URL from shared state
    let ws_url = {
        let ws_url = state.ws_url.lock().unwrap();
        ws_url.clone().ok_or("WebSocket URL not set")?
    };

    //let url = Url::parse(ws_url).expect("Cannot parse URL");
    let (ws_stream, _) = connect_async(ws_url)
        .await
        .map_err(|e| format!("Failed to connect: {}", e))?;

    let (_ws_write, mut ws_read) = ws_stream.split(); // Split stream into sender and receiver

    // Send a message to the WebSocket server
    //ws_write.send(Message::Text("Hello, WebSocket!".into())).await.expect("Failed to send message");

    // Receive a message from the WebSocket server
    while let Some(Ok(message)) = ws_read.next().await {
        match message {
            Message::Text(text) => {
                println!("Received: {}", text);
                let ws_message = WebSocketMessage {
                    message: text.clone(),
                };
                app_handle.emit("websocket-message", ws_message).unwrap();
            }
            Message::Binary(bin) => {
                println!("Received binary data: {:?}", bin);
                //app_handle.emit_all("websocket-message", format!("{:?}", bin)).unwrap(); // Emit the binary data
            }
            _ => (),
        }
    }

    Ok("WebSocket connection closed".to_string())
}
