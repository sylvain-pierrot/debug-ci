use axum::{extract::{State, Path}, Json, response::IntoResponse, http::StatusCode};
use axum_valid::Valid;
use serde_json::Value;
use tracing::event;

use crate::{common::{state::app_state::AppState, errors::database_errors::map_to_http_status_code}, domains::questions::service::{create_question, fetch_random_questions, self}};

use super::schema::{CreateQuestionDto, IncrementAnswerCountDto};

pub async fn create_question_handler(
    State(data): State<AppState>,
    Valid(Json(create_counter_dto)): Valid<Json<CreateQuestionDto>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    event!(
        tracing::Level::INFO,
        "create_question_handler called with {:?}",
        create_counter_dto
    );

    let counter = create_question(create_counter_dto, &data.db)
        .await
        .map_err(|e| {
            event!(tracing::Level::ERROR, "error creating question: {}", e);
            map_to_http_status_code(&e)
        })?;

    Ok((StatusCode::CREATED, Json(counter)))
}

pub async fn get_random_question_handler(
    State(data): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    event!(
        tracing::Level::INFO,
        "get_random_question called"
    );

    let question = fetch_random_questions(&data.db)
        .await
        .map_err(|e| {
            event!(tracing::Level::ERROR, "error creating counter: {}", e);
            map_to_http_status_code(&e)
        })?;

    Ok((StatusCode::OK, Json(question)))
}

pub async fn increment_answer_count_handler(
    Path(question_id): Path<String>,
    State(data): State<AppState>,
    Valid(Json(create_counter_dto)): Valid<Json<IncrementAnswerCountDto>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    event!(
        tracing::Level::INFO,
        "increment_answer_count_handler/{:?} called with {:?}",
        question_id,
        create_counter_dto
    );

    service::increment_answer_count(question_id, create_counter_dto, &data.db)
        .await
        .map_err(|e| {
            event!(tracing::Level::ERROR, "error creating question: {}", e);
            map_to_http_status_code(&e)
        })?;

    Ok((StatusCode::NO_CONTENT, ()))
}