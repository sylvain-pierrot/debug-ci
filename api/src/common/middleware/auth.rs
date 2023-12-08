use axum::{
    http::{StatusCode, Request},
    middleware::Next,
    response::IntoResponse, extract::State,
};
use base64::{engine::general_purpose, Engine};
use tracing_log::log::error;
use crate::common::state::app_state::AppState;

pub async fn basic_auth_middleware<B>(
    State(state): State<AppState>,
    req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let auth_header = req.headers().get("Authorization");

    if let Some(auth_header) = auth_header {
        let auth_header = auth_header.to_str().unwrap();
        let auth_header = auth_header.replace("Basic ", ""); 
        let auth_header = general_purpose::STANDARD.decode(auth_header).map_err(|e| {
            error!("error decoding auth header: {}", e);
            (StatusCode::UNAUTHORIZED, "Unauthorized".to_string())
        })?;
        let auth_header = String::from_utf8(auth_header).map_err(|e| {
            error!("error decoding auth header: {}", e);
            (StatusCode::UNAUTHORIZED, "Unauthorized".to_string())
        })?;
        let auth_header: Vec<&str> = auth_header.split(':').collect();

        if auth_header[0] == state.env.admin_username && auth_header[1] == state.env.admin_password {
            let res = next.run(req).await;

            return Ok(res);
        }
    }

    Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))
}