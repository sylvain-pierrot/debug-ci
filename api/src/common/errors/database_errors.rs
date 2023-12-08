use axum::{http, Json};
use regex::Regex;
use serde_json::Value;

const DB_UNIQUE_CONTRAINT_VIOLATION: i32 = 2067;
const DB_PRIMARY_KEY_VIOLATION: i32 = 1555;

const APP_UNIQUE_CONTRAINT_VIOLATION: &str = "COUNTER-DB-302";
const APP_INTERNAL_SERVER_ERROR: &str = "COUNTER-DB-500";
const APP_RESOURCE_NOT_FOUND: &str = "COUNTER-DB-404";

pub struct ErrorMeta {
    pub reason: String,
    pub column: Option<String>,
}

/// SQLlite return error code in the following format:
///   error returned from database: (code: <code>) <reason>: <table>.<column>
/// Where <code> is a number, <reason> is a string, <table> is a string and <column> is a string.
pub fn map_to_http_status_code(error: &sqlx::Error) -> (http::StatusCode, Json<Value>) {
    let result = match error {
        sqlx::Error::Database(_) => {
            let error_code = extract_error_code(error);

            match error_code {
                Some(DB_PRIMARY_KEY_VIOLATION) => {
                    let error_field = extract_error_field(error).unwrap_or_default();

                    (
                        http::StatusCode::UNPROCESSABLE_ENTITY,
                        APP_UNIQUE_CONTRAINT_VIOLATION,
                        "Unique constraint violation".to_string(),
                        Some(error_field),
                    )
                }
                Some(DB_UNIQUE_CONTRAINT_VIOLATION) => {
                    let error_field = extract_error_field(error).unwrap_or_default();

                    (
                        http::StatusCode::UNPROCESSABLE_ENTITY,
                        APP_UNIQUE_CONTRAINT_VIOLATION,
                        "Unique constraint violation".to_string(),
                        Some(error_field),
                    )
                }
                _ => (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    APP_INTERNAL_SERVER_ERROR,
                    "Unknown error".to_string(),
                    None,
                ),
            }
        }
        sqlx::Error::RowNotFound => (
            http::StatusCode::NOT_FOUND,
            APP_RESOURCE_NOT_FOUND,
            "Row not found".to_string(),
            None,
        ),
        _ => (
            http::StatusCode::INTERNAL_SERVER_ERROR,
            APP_INTERNAL_SERVER_ERROR,
            "Unknown error".to_string(),
            None,
        ),
    };

    let json_response = serde_json::json!({
        "status_code": result.0.as_u16(),
        "error_code": result.1,
        "message": result.2,
        "meta": result.3,
    });

    (result.0, Json(json_response))
}

/// Extract <code> from error message
pub fn extract_error_code(error: &sqlx::Error) -> Option<i32> {
    let re = Regex::new(r"\(code: (\d+)\)").unwrap();

    let error_message = error.to_string();

    let error_code = re
        .captures(&error_message)?
        .get(1)?
        .as_str()
        .parse::<i32>()
        .ok()?;

    Some(error_code)
}

/// Extract <table>.<column> from error message
pub fn extract_error_field(error: &sqlx::Error) -> Option<String> {
    let re = Regex::new(r": \w+\.(\w+)$").unwrap();

    let error_message = error.to_string();

    let error_field = re.captures(&error_message)?.get(1)?.as_str();

    Some(error_field.to_string())
}
