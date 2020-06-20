use actix_threadpool::BlockingError;
use actix_web::error::{Error as ActixError, ParseError};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use serde::Deserialize;
use std::{fmt, io::Error as IoError};

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub status_code: u16,
    pub message: String,
}

impl ApiError {
    pub fn new(status_code: u16, message: String) -> ApiError {
        ApiError {
            status_code,
            message,
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<ActixError> for ApiError {
    fn from(error: ActixError) -> ApiError {
        ApiError::new(500, error.to_string())
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> ApiError {
        match error {
            DieselError::DatabaseError(kind, err) => match kind {
                DatabaseErrorKind::UniqueViolation => ApiError::new(409, err.message().to_string()),
                DatabaseErrorKind::ForeignKeyViolation => ApiError::new(
                    500,
                    format!("Foreign key constraint violated: {}", err.message()),
                ),
                DatabaseErrorKind::UnableToSendCommand => {
                    ApiError::new(500, format!("Unable to send command: {}", err.message()))
                }
                DatabaseErrorKind::SerializationFailure => ApiError::new(
                    500,
                    format!(
                        "Serializable transaction failed to commit: {}",
                        err.message()
                    ),
                ),
                _ => ApiError::new(500, format!("Unexpected database error: {}", err.message())),
            },
            DieselError::NotFound => ApiError::new(404, "record not found".to_string()),
            err => ApiError::new(500, format!("Diesel error: {}", err)),
        }
    }
}

impl From<ParseError> for ApiError {
    fn from(error: ParseError) -> ApiError {
        match error {
            ParseError::Incomplete => ApiError::new(400, "incomplete request".to_string()),
            ParseError::TooLarge => ApiError::new(413, "payload is too large".to_string()),
            ParseError::Io(err) => ApiError::new(500, format!("an IO error occurred: {}", err)),
            err => unreachable!(format!("should be handled by actix: {}", err)),
        }
    }
}

impl From<BlockingError<IoError>> for ApiError {
    fn from(error: BlockingError<IoError>) -> ApiError {
        match error {
            BlockingError::Error(e) => {
                ApiError::new(500, format!("failed to write to disk: {}", e))
            }
            BlockingError::Canceled => {
                ApiError::new(500, "writing operation cancelled".to_string())
            }
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = if status_code.as_u16() < 500 {
            self.message.clone()
        } else {
            error!("{}", self.message);
            "internal server error".to_string()
        };

        HttpResponse::build(status_code).json(json!({ "success": false, "reason": message }))
    }
}
