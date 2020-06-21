use super::utils;
use crate::{
    config::CFG,
    errors::ApiError,
    models::{Deployment, Handler, Project, Route},
    project_format::ProjectFormat,
};
use actix_multipart::{Field, Multipart};
use actix_session::Session;
use actix_web::{delete, get, post, put, web, HttpResponse};
use cloud_storage::Object;
use futures::{StreamExt, TryStreamExt};
use redis::Commands;
use ring::digest;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read};
use std::path::PathBuf;
use uuid::Uuid;
use zip::ZipArchive;

#[get("/projects/{id}/deployments")]
async fn list(id: web::Path<Uuid>, session: Session) -> Result<HttpResponse, ApiError> {
    let user_id = utils::is_authenticated(&session)?;

    let project = Project::find(id.into_inner())?;
    if project.user_id != user_id {
        return Err(ApiError::new(
            403,
            "user lacks permission for resource".to_string(),
        ));
    }

    let deployments = Deployment::find_all(project.id)?;
    Ok(utils::success_with_data(json!(deployments)))
}

#[post("/projects/{id}/deployments")]
async fn create(
    format: web::Json<ProjectFormat>,
    id: web::Path<Uuid>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let user_id = utils::is_authenticated(&session)?;

    let project = Project::find(id.into_inner())?;
    if project.user_id != user_id {
        return Err(ApiError::new(
            403,
            "user lacks permission for resource".to_string(),
        ));
    }

    let format = format.into_inner();
    let hash = generate_hash(&format)?;

    // Prevent duplicates
    if let Some(d) = Deployment::find_by_hash(&hash, project.id)? {
        return Ok(utils::success_with_data(json!({ "id": d.id })));
    }

    let deployment = Deployment::create(format.version, hash, project.id)?;

    for handler in format.handlers {
        deployment.add_handler(handler)?;
    }

    for route in format.routes {
        deployment.add_route(route)?;
    }

    Ok(utils::success_with_data(json!({ "id": deployment.id })))
}

#[put("/projects/{project_id}/deployments/{deployment_id}")]
async fn add_static(
    mut payload: Multipart,
    ids: web::Path<(Uuid, Uuid)>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let user_id = utils::is_authenticated(&session)?;

    let project = Project::find(ids.0)?;
    if project.user_id != user_id {
        return Err(ApiError::new(
            403,
            "user lacks permission for resource".to_string(),
        ));
    }

    let deployment = Deployment::find(ids.1)?;
    if deployment.project_id != project.id {
        return Err(ApiError::new(
            404,
            "specified deployment does not exist".to_string(),
        ));
    } else if deployment.has_static {
        return Err(ApiError::new(
            403,
            "static files already registered for deployment".to_string(),
        ));
    }

    let mut static_files: Option<Field> = None;

    while let Ok(Some(field)) = payload.try_next().await {
        let content_disposition = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let name = content_disposition
            .get_name()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;

        if name == "static" {
            static_files = Some(field);
            break;
        }
    }

    let mut static_files = match static_files {
        Some(s) => s,
        None => {
            return Err(ApiError::new(
                400,
                "form field 'static' is required".to_string(),
            ))
        }
    };

    let content_type = static_files.content_type();
    if content_type.type_() != "application" && content_type.subtype() != "zip" {
        return Err(ApiError::new(
            400,
            "form field 'static' must be a zip file".to_string(),
        ));
    }

    let mut raw: Vec<u8> = Vec::new();
    while let Some(chunk) = static_files.next().await {
        let data = chunk.unwrap();
        raw.extend_from_slice(data.as_ref());
    }

    let buffer = Cursor::new(raw);
    let mut archive = ZipArchive::new(buffer).map_err(|_| ApiError::new(400, "invalid zip archive format".to_string()))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let name = file.sanitized_name().as_path().display().to_string();

        let mut path = PathBuf::new();
        path.push(project.id.to_string());
        path.push(&name);

        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        let guess = mime_guess::from_path(&name);
        let mime = guess.first_or(mime::APPLICATION_OCTET_STREAM);

        web::block(move || Object::create(&CFG.gcp.bucket, data.as_ref(), path.to_str().unwrap(), &mime.to_string())).await?;
    }

    deployment.mark_has_static()?;

    let mut connection = crate::redis::connection()?;
    let _n: i32 = connection.publish("publish", project.id.to_string()).unwrap();

    Ok(utils::success())
}

#[get("/projects/{project_id}/deployments/{deployment_id}")]
async fn read(ids: web::Path<(Uuid, Uuid)>, session: Session) -> Result<HttpResponse, ApiError> {
    let user_id = utils::is_authenticated(&session)?;

    let project = Project::find(ids.0)?;
    if project.user_id != user_id {
        return Err(ApiError::new(
            403,
            "user lacks permission for resource".to_string(),
        ));
    }

    let deployment = Deployment::find(ids.1)?;
    if deployment.project_id != project.id {
        return Err(ApiError::new(
            404,
            "specified deployment does not exist".to_string(),
        ));
    } else if !deployment.has_static {
        return Err(ApiError::new(
            406,
            "deployment is not yet complete".to_string(),
        ));
    }

    let mut response = ReadResponse::from(deployment.clone());
    response.routes = Route::find_all(deployment.id)?;
    response.handlers = Handler::find_all(deployment.id)?;

    Ok(utils::success_with_data(json!(response)))
}

#[delete("/projects/{project_id}/deployments/{deployment_id}")]
async fn delete(ids: web::Path<(Uuid, Uuid)>, session: Session) -> Result<HttpResponse, ApiError> {
    let user_id = utils::is_authenticated(&session)?;

    let project = Project::find(ids.0)?;
    if project.user_id != user_id {
        return Err(ApiError::new(
            403,
            "user lacks permission for resource".to_string(),
        ));
    }

    let deployment = Deployment::find(ids.1)?;
    if deployment.project_id != project.id {
        return Err(ApiError::new(
            404,
            "specified deployment does not exist".to_string(),
        ));
    }

    for handler in Handler::find_all(deployment.id)? {
        Handler::delete(handler.id)?;
    }

    for route in Route::find_all(deployment.id)? {
        Route::delete(route.id)?;
    }

    Deployment::delete(deployment.id)?;

    let mut connection = crate::redis::connection()?;
    let _n: i32 = connection.publish("delete", project.id.to_string()).unwrap();

    Ok(utils::success())
}

/// Serialize a project to JSON
fn generate_hash(format: &ProjectFormat) -> Result<String, ApiError> {
    let json = serde_json::to_string(format).map_err(|e| {
        ApiError::new(
            500,
            format!(
                "failed to re-encode project configuration for hashing: {}",
                e
            ),
        )
    })?;
    let hash = digest::digest(&digest::SHA256, json.as_bytes());
    Ok(hex::encode(hash))
}

/// Custom response type to combine the standard deployment
/// information, handlers, and routes in a flat object.
#[derive(Deserialize, Serialize)]
struct ReadResponse {
    #[serde(flatten)]
    pub deployment: Deployment,
    pub routes: Vec<Route>,
    pub handlers: Vec<Handler>,
}

impl From<Deployment> for ReadResponse {
    fn from(d: Deployment) -> ReadResponse {
        ReadResponse {
            deployment: d,
            routes: vec![],
            handlers: vec![],
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
    cfg.service(create);
    cfg.service(add_static);
    cfg.service(read);
    cfg.service(delete);
}
