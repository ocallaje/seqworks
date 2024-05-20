use std::sync::Mutex;

pub struct AppState {
    pub ws_url: Mutex<Option<String>>,
    pub username: Mutex<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            ws_url: Mutex::new(None),
            username: Mutex::new(None),
        }
    }
}
