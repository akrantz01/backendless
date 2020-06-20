use super::utils;
use crate::{
    errors::ApiError,
    models::{Project, ProjectMessage},
};
use actix_session::Session;
use actix_web::{delete, get, post, put, web, HttpResponse};
use regex::Regex;
use uuid::Uuid;

lazy_static! {
    static ref PROJECT_NAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
}

#[get("/projects")]
async fn list(session: Session) -> Result<HttpResponse, ApiError> {
    let id = utils::is_authenticated(&session)?;

    let projects = Project::find_all(id)?;
    Ok(utils::success_with_data(json!(projects)))
}

#[post("/projects")]
async fn create(
    project: web::Json<ProjectMessage>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let id = utils::is_authenticated(&session)?;
    let project = project.into_inner();

    if !PROJECT_NAME_REGEX.is_match(&project.name) {
        return Err(ApiError::new(
            400,
            "field 'name' must match '^[a-zA-Z0-9_-]+$'".to_string(),
        ));
    }

    Project::create(project, id)?;
    Ok(utils::success())
}

#[get("/projects/{id}")]
async fn read(id: web::Path<Uuid>, session: Session) -> Result<HttpResponse, ApiError> {
    let user_id = utils::is_authenticated(&session)?;

    let project = Project::find(id.into_inner())?;
    if project.user_id == user_id {
        Ok(utils::success_with_data(json!(project)))
    } else {
        Err(ApiError::new(
            403,
            "user lacks permission to access resource".to_string(),
        ))
    }
}

#[put("/projects/{id}")]
async fn update(
    id: web::Path<Uuid>,
    project: web::Json<ProjectMessage>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let user_id = utils::is_authenticated(&session)?;

    let p = Project::find(id.clone())?;
    if p.user_id == user_id {
        Project::update(id.into_inner(), project.into_inner())?;
        Ok(utils::success())
    } else {
        Err(ApiError::new(
            403,
            "user lacks permission to access resource".to_string(),
        ))
    }
}

#[delete("/projects/{id}")]
async fn delete(id: web::Path<Uuid>, session: Session) -> Result<HttpResponse, ApiError> {
    let user_id = utils::is_authenticated(&session)?;

    let project = Project::find(id.clone())?;
    if project.user_id == user_id {
        Project::delete(id.into_inner())?;
        Ok(utils::success())
    } else {
        Err(ApiError::new(
            403,
            "user lacks permission to access resource".to_string(),
        ))
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
    cfg.service(create);
    cfg.service(read);
    cfg.service(update);
    cfg.service(delete);
}
