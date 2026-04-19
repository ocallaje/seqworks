use axum::{
    routing::{get, post},
    Router,
};
use serde::{Serialize, Deserialize};
use crate::{
    handler::{
        health_checker_handler, pipeline_handler, dirs_handler, login_handler, 
        metrics_handler
    },
    auth::{
        OidcAppState, OidcStateStore, oidc_login_handler, oidc_callback_handler, 
        oidc_logout_handler, validate_handler
    },
    //model,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use crate::oidc;
use tower_http::services::ServeDir;



pub async fn create_router() -> Router {
    // Build OIDC client async, then move state into the router
    let oidc_client = Arc::new(oidc::build_oidc_client().await);
    let state_store: OidcStateStore = Arc::new(Mutex::new(HashMap::new()));
    let oidc_state = OidcAppState { client: oidc_client, state_store };
    //let db = model::todo_db();

    Router::new()
        .route("/api/health", get(health_checker_handler))
        .route("/api/init_pipe", post(pipeline_handler))
        .route("/api/get_project_dirs", post(dirs_handler))
        .route("/api/auth/login", post(login_handler))
        // --- new OIDC routes ---
        .route("/api/auth/oidc/login", get(oidc_login_handler))
        .route("/api/auth/oidc/callback", get(oidc_callback_handler))
        .route("/api/auth/validate", get(validate_handler))
        .route("/api/auth/logout", post(oidc_logout_handler))

        // dashboard routes
        .route("/api/dashboard/metrics", get(metrics_handler))

        .nest_service("/", ServeDir::new("../src"))   // <-- serves ./static/ dir
        .with_state(oidc_state)
    }