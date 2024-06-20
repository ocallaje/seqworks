use seqworks::app_state::AppState;
use seqworks::commands;

#[cfg(test)]
mod tests {
    use super::*;
    use tauri::{Manager, State, generate_context, Builder};
    use tauri::test::{mock_builder, mock_context, noop_assets};
    include!(concat!("../env_vars.rs"));


    #[tauri::command]
      fn ping() -> &'static str {
          "pong"
      }

    fn create_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::App<R> {
        builder
            .invoke_handler(tauri::generate_handler![ping])
            // remove the string argument to use your app's config file
            .build(tauri::generate_context!())
            .expect("failed to build app")
    }


    fn test_test() {
        let app = create_app(mock_builder());
        let webview = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
            .build()
            .unwrap();

        // run the `ping` command and assert it returns `pong`
        let res = tauri::test::get_ipc_response(
            &webview,
            tauri::webview::InvokeRequest {
                cmd: "ping".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                url: "http://tauri.localhost".parse().unwrap(),
                body: tauri::ipc::InvokeBody::default(),
                headers: Default::default(),
                //invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        ).map(|b| b.deserialize::<String>().unwrap());
    }

    
}
