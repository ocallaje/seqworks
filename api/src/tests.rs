#[tokio::test]
async fn test_pipeline_endpoint() {
    let client = reqwest::Client::new();
    let token = "<YOUR_JWT_TOKEN>";

    let payload = serde_json::json!({
        "params": {
            "AppParams": {
                "custom_run_name": "test_run"
            }
        }
    });

    let res = client
        .post("http://localhost:8000/api/init_pipe")
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert!(res.status().is_success());
    let body: serde_json::Value = res.json().await.unwrap();
    println!("{:#?}", body);
}
