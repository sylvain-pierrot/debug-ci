use axum::{response::IntoResponse, Json};

/// GET `/health` handler
pub async fn health_check() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status": "ok"
    });

    Json(json_response)
}
