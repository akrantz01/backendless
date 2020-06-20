use super::utils;
use crate::{
    errors::ApiError,
    models::{User, UserMessage},
};
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct LoginMessage {
    pub email: String,
    pub password: String,
}

#[post("/authentication/register")]
async fn register(user: web::Json<UserMessage>) -> Result<HttpResponse, ApiError> {
    let user = user.into_inner();

    if user.username.len() > 64 {
        Err(ApiError::new(
            400,
            "field 'username' must be less than 64 characters".to_string(),
        ))
    } else if user.email.len() < 5 || user.email.len() > 254 {
        Err(ApiError::new(
            400,
            "field 'email' must be between 5 and 254 characters".to_string(),
        ))
    } else {
        User::create(user)?;
        Ok(utils::success())
    }
}

#[post("/authentication/login")]
async fn login(
    credentials: web::Json<LoginMessage>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let credentials = credentials.into_inner();

    // Find user by email
    let user = User::find_by_email(credentials.email).map_err(|e| match e.status_code {
        404 => ApiError::new(401, "invalid email or password".to_string()),
        _ => e,
    })?;

    // Check password
    let is_valid = user.verify_password(credentials.password.as_bytes())?;

    if is_valid {
        // Set session
        session.set("user_id", user.id)?;
        session.renew();

        Ok(utils::success())
    } else {
        Err(ApiError::new(401, "invalid email or password".to_string()))
    }
}

#[get("/authentication/logout")]
async fn logout(session: Session) -> Result<HttpResponse, ApiError> {
    utils::is_authenticated(&session)?;
    session.purge();
    Ok(utils::success())
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
    cfg.service(logout);
}
