use axum::{ Json, http::StatusCode, response::IntoResponse };
use serde_json::json;

pub type ApiResultJson<T> = Result<(StatusCode, Json<T>), ApiError>;
pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    InvalidInput(String),
    ValidationError(String),
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        ApiError::DatabaseError(error)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_msg): (StatusCode, String) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Seite nicht gefunden!".to_string()),
            ApiError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::ValidationError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            ApiError::DatabaseError(err) => {
                // Intern loggen (in Produktion NIEMALS rohe DB-Fehler an den Client schicken!)
                eprintln!("Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Database Error".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error": error_msg,
        }));

        (status, body).into_response()
    }
}