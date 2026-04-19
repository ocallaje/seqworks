use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Serialize, Deserialize};
use serde_json::{json, Map, Value};

use crate::{
    //state::ApiState,
    auth::AuthUser,
    config,
};
use pipeline_core::{states, pipelines, utils};


pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Seqworks API is up!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn pipeline_handler(
    //user: AuthUser,
    Json(wrapper): Json<states::AppParamsWrapper>,
) -> Result<axum::Json<serde_json::Value>, (StatusCode, String)>  {

    // test user, remove and uncomment authuser from function signature
    let user = AuthUser {
        username: "testuser".to_string(),
    };

    println!("User {} started pipeline", user.username);

    let pipeline_repo_url;
    let root_dir = config::data_root().unwrap();

    let pipe_result= match wrapper.params {
        states::AppParamsEnum::AppParams(params) => {
            println!("Bulk params detected");
            pipeline_repo_url = "https://github.com/ocallaje/kubeflow".to_string();
            pipelines::parse_bulk_params(root_dir, params, &user.username)    
        }
        states::AppParamsEnum::AppSCParams(params) => {
            println!("SC params detected");
            pipeline_repo_url = "https://github.com/ocallaje/kubeflow".to_string();
            pipelines::parse_sc_params(root_dir, params, &user.username)
        }
    };

    let pipe_result: pipelines::PipelineResult = match pipe_result {
        Ok(result) => result,                       // Success: extract PipelineResult
        Err(e) => {
            eprintln!("Failed to parse params: {}", e);
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Failed to parse params: {}", e)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response).to_string()))
        }
    };

    //let rnaseq_cmd = pipe_result.rnaseq_cmd;
    let params_map: Map<String, Value> = pipe_result.params_map;

    // Prepare Argo workflow body
    let args = map_to_nextflow_args(&params_map);
    println!("{}", args);

    let body = json!({
        "resourceKind": "WorkflowTemplate",
        "resourceName": "nextflow-pipeline-template",
        "namespace": "nextflow",
        "submitOptions": {
            "parameters": [
                format!("pipeline_url={}", pipeline_repo_url),
                format!("nextflow_params={}", args),
            ],
            "labels": "triggered-by=api"
        }
    });

    // Submit to Argo
    let client = reqwest::Client::new();
    let argo_url = "http://192.168.1.39:31500/api/v1/workflows/nextflow/submit";
    let resp = client.post(argo_url)
        //.bearer_auth(argo_token)
        .json(&body)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to reach Argo server: {}", e)))?;

    if resp.status().is_success() {
        let workflow_resp: serde_json::Value = resp.json().await.unwrap_or(json!({}));
        Ok(axum::Json(json!({
            "status": "Pipeline Initialised! Please wait for completion email",
            "workflow": workflow_resp
        })))
    } else {
        let status = StatusCode::from_u16(resp.status().as_u16())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let text = resp.text().await.unwrap_or_default();

        Err((status, format!("Argo error: {}", text)))
    }

}

#[derive(Deserialize)]
pub struct DirsRequest {
    pipe_type: String,
}
pub async fn dirs_handler(
    //user: AuthUser,
    Json(payload): Json<DirsRequest>
) -> impl IntoResponse {

    let root_dir = config::data_root().unwrap();
    
    let dirs: Vec<String> = match utils::get_dirs(&root_dir, &payload.pipe_type) {
        Ok(dirs) => dirs,
        Err(err) => {
            tracing::error!("get_dirs failed: {}", err);
            Vec::new()
        }
    };

    Json(serde_json::json!({
        "dirs": dirs
    }))

    
}


#[derive(Deserialize)]
pub struct LoginRequest {
    pub user: String,
    pub pass: String,
}

pub async fn login_handler(
    //user: AuthUser,
    Json(payload): Json<LoginRequest>
) -> impl IntoResponse {
    
    if payload.user == "user" && payload.pass == "123" {
        (
            StatusCode::OK,
            Json(serde_json::json!({
                "authenticated": true
            }))
        )
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "authenticated": false,
                "message": "Invalid credentials"
            }))
        )
    }
    
}


fn map_to_nextflow_args(params_map: &Map<String, Value>) -> String {
    params_map
        .iter()
        .map(|(k, v)| {
            // Convert value to string safely
            let v_str = match v {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => serde_json::to_string(v).unwrap_or_default(),
            };
            format!("--{} {}", k, v_str)
        })
        .collect::<Vec<String>>()
        .join(" ")
}



// CellXGene commands
#[derive(Deserialize)]
pub struct CxgParams {
    pub project: String,
    pub h5_file: String,
}

pub async fn start_cellxgene_via_argo(
    Json(params): Json<CxgParams>
) -> Result<axum::Json<serde_json::Value>, (StatusCode, String)> {
    let argo_url = "http://192.168.1.39:31500/api/v1/workflows/seqworks/submit";
    //let argo_token = std::env::var("ARGO_TOKEN").unwrap_or_default(); // use service account or token

    let client = reqwest::Client::new();

    let body = json!({
        "resourceKind": "WorkflowTemplate",
        "resourceName": "cellxgene-template",
        "namespace": "seqworks",
        "workflowTemplateRef": { "name": "cellxgene-template" },
        "submitOptions": {
            "parameters": [
                format!("project={}", params.project),
                format!("h5_file={}", params.h5_file)
            ],
            "labels": "triggered-by=seqworks_api"
        }
    });

    let resp = client.post(argo_url)
        //.bearer_auth(argo_token)
        .json(&body)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to reach Argo server: {}", e)))?;

    if resp.status().is_success() {
        let workflow_resp: serde_json::Value = resp.json().await.unwrap_or(json!({}));
        Ok(axum::Json(json!({
            "status": "success",
            "workflow": workflow_resp
        })))
    } else {
        let status = StatusCode::from_u16(resp.status().as_u16())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let text = resp.text().await.unwrap_or_default();

        Err((status, format!("Argo error: {}", text)))
    }
}



#[derive(Serialize)]
pub struct DashboardMetrics {
    pub active_workflows: u32,
    pub queued_workflows: u32,
    pub samples_today: u32,
    pub samples_completed: u32,
    pub avg_runtime_mins: u32,
    pub runtime_trend: String,
    pub failed_runs_24h: u32,
}

pub async fn metrics_handler(
    _auth: AuthUser,
) -> impl IntoResponse {
    match fetch_workflows().await {
        Ok(workflows) => {
            let metrics = compute_metrics(&workflows);
            Json(metrics)
        }
        Err(err) => {
            eprintln!("Failed to fetch Argo workflows: {:?}", err);

            Json(DashboardMetrics {
                active_workflows: 0,
                queued_workflows: 0,
                samples_today: 0,
                samples_completed: 0,
                avg_runtime_mins: 0,
                runtime_trend: "error".into(),
                failed_runs_24h: 0,
            })
        }
    }
}

use chrono::{DateTime, Utc, Duration};

fn compute_metrics(workflows: &[ArgoWorkflow]) -> DashboardMetrics {
    let now = Utc::now();
    let today_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
    let today_start = DateTime::<Utc>::from_naive_utc_and_offset(today_start, Utc);

    let week_ago = now - Duration::days(7);
    let two_weeks_ago = now - Duration::days(14);

    let mut active = 0;
    let mut queued = 0;
    let mut completed = 0;

    let mut failed_24h = 0;
    let mut samples_today = 0;

    let mut runtimes_this_week = vec![];
    let mut runtimes_last_week = vec![];

    for wf in workflows {
        let status = match &wf.status {
            Some(s) => s,
            None => continue,
        };

        let phase = status.phase.as_deref().unwrap_or("");

        match phase {
            "Running" => active += 1,
            "Pending" => queued += 1,
            "Succeeded" => completed += 1,
            _ => {}
        }

        // Parse timestamps
        let start = status.startedAt.as_ref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        let end = status.finishedAt.as_ref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        // Samples today
        if let Some(s) = start {
            if s >= today_start {
                samples_today += 1;
            }
        }

        // Failed in last 24h
        if phase == "Failed" {
            if let Some(e) = end {
                if e >= now - Duration::hours(24) {
                    failed_24h += 1;
                }
            }
        }

        // Runtime calculations
        if let (Some(s), Some(e)) = (start, end) {
            let runtime_mins = (e - s).num_minutes();

            if runtime_mins > 0 {
                if s >= week_ago {
                    runtimes_this_week.push(runtime_mins);
                } else if s >= two_weeks_ago {
                    runtimes_last_week.push(runtime_mins);
                }
            }
        }
    }

    // Average runtime (this week)
    let avg_runtime = if !runtimes_this_week.is_empty() {
        runtimes_this_week.iter().sum::<i64>() / runtimes_this_week.len() as i64
    } else {
        0
    };

    // Last week avg
    let last_week_avg = if !runtimes_last_week.is_empty() {
        runtimes_last_week.iter().sum::<i64>() / runtimes_last_week.len() as i64
    } else {
        0
    };

    // Trend calculation
    let runtime_trend = if last_week_avg > 0 {
        let change = ((avg_runtime - last_week_avg) as f64 / last_week_avg as f64) * 100.0;

        if change.abs() < 1.0 {
            "→ ~0% vs last week".to_string()
        } else if change > 0.0 {
            format!("↑ {:.0}% vs last week", change)
        } else {
            format!("↓ {:.0}% vs last week", change.abs())
        }
    } else {
        "N/A".to_string()
    };

    DashboardMetrics {
        active_workflows: active,
        queued_workflows: queued,
        samples_today,
        samples_completed: completed,
        avg_runtime_mins: avg_runtime as u32,
        runtime_trend,
        failed_runs_24h: failed_24h,
    }
}

#[derive(Deserialize)]
struct ArgoWorkflowList {
    items: Vec<ArgoWorkflow>,
}

#[derive(Deserialize)]
struct ArgoWorkflow {
    status: Option<WorkflowStatus>,
}

#[derive(Deserialize)]
struct WorkflowStatus {
    phase: Option<String>, // Running, Pending, Succeeded, Failed, etc.
    startedAt: Option<String>,
    finishedAt: Option<String>,
}


async fn fetch_workflows() -> Result<Vec<ArgoWorkflow>, reqwest::Error> {
    let url = std::env::var("ARGO_PIPE_URL")
        .expect("ARGO_PIPE_URL must be set");

    let client = reqwest::Client::new();

    let res = client
        .get(&url)
        .send()
        .await?
        .json::<ArgoWorkflowList>()
        .await?;

    Ok(res.items)
}