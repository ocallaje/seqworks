#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::response::{Response, IntoResponse};
    use serde_json::Value;
    use crate::handler::{CxgParams, start_cellxgene_via_argo};
    //use tower::ServiceExt; // for `oneshot`


    #[tokio::test]
    async fn test_pipeline_endpoint() {
        let client = reqwest::Client::new();
        //let token = "<YOUR_JWT_TOKEN>";

        let payload = serde_json::json!({
            "params": {
                "AppParams": {
                    "illumina_stranded_kit": "true",
                    "strandedness": "true",
                    "paired_end": "true",
                    "trimadaptors": "true",
                    "verify": "true",
                    "merge_fastqs": "false",
                    "send_email": "true",
                    "cc": "user@example.com",
                    "custom_run_name": "my_run_001",
                    "project": "test_proj1",
                    "genome": "Mouse",
                    "genome_version": "v38",
                    "workflow": "default",
                    "deseq_model": "~Genotype",
                    "deseq_ref_var": "neg"
                }
            }
        });

        let res = client
            .post("http://localhost:8000/api/init_pipe")
            //.bearer_auth(token)
            .json(&payload)
            .send()
            .await
            .unwrap();

        println!("Status: {}", res.status());
        let body = res.text().await.unwrap();
        println!("Body:\n{}", body);

        //assert!(res.status().is_success());
        //let body: serde_json::Value = res.json().await.unwrap();
        //println!("{:#?}", body);
    }

    #[tokio::test]
    async fn test_get_dirs() {
        let client = reqwest::Client::new();
        //let token = "<YOUR_JWT_TOKEN>";

        let payload = serde_json::json!({
            "pipe_type": "bulk"
        });

        let res = client
            .post("http://localhost:8000/api/get_project_dirs")
            //.bearer_auth(token)
            .json(&payload)
            .send()
            .await
            .unwrap();

        assert!(res.status().is_success());
        let body: serde_json::Value = res.json().await.unwrap();
        println!("{:#?}", body);
    }

    #[tokio::test]
    async fn test_start_cellxgene_response() {
        // Create dummy request payload
        let params = CxgParams {
            project: "test_1".to_string(),
            h5_file: "test.h5".to_string(),
        };

        // Call the handler directly
        let response = start_cellxgene_via_argo(axum::extract::Json(params))
            .await;

        // Check HTTP status
        //assert_eq!(response.status(), StatusCode::OK);

        match response {
            Ok(json) => {
                assert_eq!(json.0["status"], "success");
            }
            Err((status, msg)) => panic!("Handler returned error {}: {}", status, msg),
        }
    }
}