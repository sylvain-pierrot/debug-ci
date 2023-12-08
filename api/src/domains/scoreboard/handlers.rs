use axum::{extract::State, Json, response::IntoResponse, http::StatusCode};
use axum_valid::Valid;
use serde_json::Value;
use tracing::info;

use crate::{common::{state::app_state::AppState, errors::database_errors::map_to_http_status_code}, domains::scoreboard::service};

use super::schema::CreateScoreDto;

pub async fn create_score_handler(
    State(data): State<AppState>,
    Valid(Json(create_score_dto)): Valid<Json<CreateScoreDto>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    info!("create_score_handler called with {:?}", create_score_dto);

    service::create_score( &data.db, create_score_dto)
        .await
        .map_err(|e| {
            info!("error creating score: {}", e);
            map_to_http_status_code(&e)
        })?;

    Ok((StatusCode::CREATED, Json(())))
}

pub async fn get_last_ten_scores_handler(
    State(data): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    info!("get_last_ten_scores called");

    let scores = service::fetch_last_ten_scores(&data.db)
        .await
        .map_err(|e| {
            info!("error fetching last ten scores: {}", e);
            map_to_http_status_code(&e)
        })?;

    Ok((StatusCode::OK, Json(scores)))
}
