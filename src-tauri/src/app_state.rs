use std::sync::Mutex;
use serde::Deserialize;
use crate::pipelines;

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



#[derive(Deserialize)]
pub struct AppParamsWrapper {
    pub params: AppParamsEnum,
}

#[derive(Deserialize)]
pub enum AppParamsEnum {
    AppParams(pipelines::AppParams),
    AppSCParams(pipelines::AppSCParams),
}