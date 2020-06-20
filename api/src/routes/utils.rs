use crate::errors::ApiError;
use actix_session::Session;
use actix_web::HttpResponse;
use uuid::Uuid;

/// Get the authenticated user's ID
pub fn is_authenticated(session: &Session) -> Result<Uuid, ApiError> {
    let id: Option<Uuid> = session.get("user_id")?;

    match id {
        Some(id) => Ok(id),
        None => Err(ApiError::new(401, "unauthorized".to_string())),
    }
}

/// Generic success message
pub fn success() -> HttpResponse {
    HttpResponse::Ok().json(json!({ "success": true }))
}

/// Success message with data
pub fn success_with_data(data: serde_json::Value) -> HttpResponse {
    HttpResponse::Ok().json(json!({ "success": true, "data": data }))
}
