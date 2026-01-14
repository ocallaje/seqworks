use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::{json, Map, Value};

use crate::{
    //state::ApiState,
    auth::AuthUser
};
use pipeline_core::{states, pipelines};


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
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    // test user, remove and uncomment authuser from function signature
    let user = AuthUser {
        username: "testuser".to_string(),
    };

    println!("User {} started pipeline", user.username);

    let mut pipeline_repo_url = String::new();
    let pipe_result= match wrapper.params {
        states::AppParamsEnum::AppParams(params) => {
            println!("Bulk params detected");
            pipeline_repo_url = "https://github.com/ocallaje/kubeflow".to_string();
            pipelines::parse_bulk_params(params, &user.username)    
        }
        states::AppParamsEnum::AppSCParams(params) => {
            println!("SC params detected");
            pipeline_repo_url = "https://github.com/ocallaje/kubeflow".to_string();
            pipelines::parse_sc_params(params, &user.username)
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
            return Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    };

    //let rnaseq_cmd = pipe_result.rnaseq_cmd;
    let params_map: Map<String, Value> = pipe_result.params_map;

    // Prepare Argo workflow body
    let args = map_to_nextflow_args(&params_map);
    let body = json!({
        "resourceKind": "WorkflowTemplate",
        "resourceName": "nextflow-pipeline-template",
        "namespace": "nextflow",
        "submitOptions": {
            "parameters": [
                format!("pipeline_url={}", pipeline_repo_url),
                format!("nextflow run pipeline.nf {}", args),
            ],
            "labels": "triggered-by=api"
        }
    });

    // Submit to Argo
    //let client = Client::new();
    //let argo_response = client
    //    .post("http://192.168.1.39:31500/api/v1/workflows/nextflow/submit")
    //    .json(&body)
    //    .send()
    //    .await;

    //        match argo_response {
    //            Ok(resp) if resp.status().is_success() => {
     //               (
    //                    StatusCode::OK,
    //                    Json(json!({ "command": cmd, "argo_status": "submitted" })),
     //               )
    //                    .into_response()
    //            }
    //            Ok(resp) => {
     //               let status = resp.status();
     //               let text = resp.text().await.unwrap_or_default();
     //               (
     //                   StatusCode::INTERNAL_SERVER_ERROR,
    //                    Json(json!({ "error": format!("Argo submission failed: {} - {}", status, text) })),
     //               )
    //                    .into_response()
     //           }
    //            Err(e) => (
    //                StatusCode::INTERNAL_SERVER_ERROR,
    //                Json(json!({ "error": format!("Failed to contact Argo API: {}", e) })),
    ////            )
    //                .into_response(),
    //        }
    //    }
    //    Err(e) => (
    //        StatusCode::BAD_REQUEST,
    //        Json(json!({ "error": format!("Failed to initialise pipeline: {}", e) })),
    //    )
     //       .into_response(),
    //}

    let json_response = serde_json::json!({
        "status": "success",
        "params": body
    });

    return Ok((StatusCode::OK, Json(json_response)));
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