use super::utils;
use crate::{
    errors::ApiError,
    models::{User, UserMessage},
};
use actix_session::Session;
use actix_web::{delete, get, put, web, HttpResponse};

#[get("/user")]
async fn read(session: Session) -> Result<HttpResponse, ApiError> {
    let id = utils::is_authenticated(&session)?;
    let user = User::find(id)?;
    Ok(utils::success_with_data(json!(user)))
}

#[put("/user")]
async fn update(user: web::Json<UserMessage>, session: Session) -> Result<HttpResponse, ApiError> {
    let id = utils::is_authenticated(&session)?;
    User::update(id, user.into_inner())?;
    Ok(utils::success())
}

#[delete("/user")]
async fn delete(session: Session) -> Result<HttpResponse, ApiError> {
    let id = utils::is_authenticated(&session)?;
    User::delete(id)?;
    session.purge();
    Ok(utils::success())
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(read);
    cfg.service(update);
    cfg.service(delete);
}
