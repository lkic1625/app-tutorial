use axum::{routing::{get, put}, Router, extract::State, Json};
use axum::response::IntoResponse;
use axum::http::StatusCode;
use std::sync::Arc;
use std::collections::HashMap;
use tower_http::services::ServeDir;
use axum::routing::get_service;

mod config;
use config::Config;

#[derive(serde::Deserialize)]
struct FunctionRequest {
    method: String,
    #[serde(default)]
    params: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    context: Option<serde_json::Value>,
}

#[derive(serde::Serialize)]
struct FunctionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ErrorMessage>,
}

#[derive(serde::Serialize)]
struct ErrorMessage {
    message: String,
    #[serde(rename = "type")]
    typ: String,
}

#[tokio::main]
async fn main() {
    let config = Config::load().expect("load config");
    let addr = format!("0.0.0.0:{}", config.api.public.http.port);
    let shared = Arc::new(config);

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/functions", put(function_handler))
        .nest_service(
            "/resource/wam/tutorial",
            get_service(ServeDir::new("resources/wam/tutorial")).handle_error(|_|
                async { (StatusCode::INTERNAL_SERVER_ERROR, "failed to open file") })
        )
        .with_state(shared.clone());

    println!("running on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn ping() -> &'static str {
    "pong"
}

async fn function_handler(
    State(config): State<Arc<Config>>,
    Json(req): Json<FunctionRequest>,
) -> impl IntoResponse {
    match req.method.as_str() {
        "tutorial" => {
            let mut wam_args = HashMap::new();
            if let Some(params) = req.params.as_ref() {
                if let Some(trigger) = params.get("trigger") {
                    if let Some(attrs) = trigger.get("attributes") {
                        if let Some(map) = attrs.as_object() {
                            for (k,v) in map.iter() {
                                if let Some(s) = v.as_str() {
                                    wam_args.insert(k.clone(), s.to_string());
                                }
                            }
                        }
                    }
                }
            }
            if let Some(ctx) = req.context.as_ref() {
                if let Some(caller) = ctx.get("caller") {
                    if let Some(id) = caller.get("id").and_then(|v| v.as_str()) {
                        wam_args.insert("managerId".into(), id.to_string());
                    }
                }
            }
            wam_args.insert("message".into(), "This is a test message sent by a manager.".into());
            let res = serde_json::json!({
                "type": "wam",
                "attributes": {
                    "appId": config.app_id,
                    "name": "tutorial",
                    "wamArgs": wam_args,
                }
            });
            (StatusCode::OK, Json(FunctionResponse{ result: Some(res), error: None }))
        }
        "sendAsBot" => {
            // TODO: send message via app store API
            let res = serde_json::json!({
                "type": "string",
                "attributes": {}
            });
            (StatusCode::OK, Json(FunctionResponse{ result: Some(res), error: None }))
        }
        _ => {
            let err = ErrorMessage { message: format!("invalid method, {}", req.method), typ: String::new() };
            (StatusCode::OK, Json(FunctionResponse{ result: None, error: Some(err) }))
        }
    }
}
