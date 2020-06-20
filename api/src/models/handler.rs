use crate::{
    database, errors::ApiError, models::Deployment, project_format::Handler as HandlerFormat,
    schema::handlers,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Associations, Identifiable)]
#[belongs_to(Deployment)]
#[table_name = "handlers"]
pub struct Handler {
    pub id: Uuid,
    pub deployment_id: Uuid,
    pub name: String,
    pub query_parameters: Option<Vec<String>>,
    pub headers: Option<Vec<String>>,
    pub path_parameters: Option<Vec<String>>,
    pub body: Option<serde_json::Value>,
    pub logic: serde_json::Value,
}

impl Handler {
    /// Retrieve all handlers for a deployment
    pub fn find_all(deployment_id: Uuid) -> Result<Vec<Self>, ApiError> {
        let conn = database::connection()?;

        let results = handlers::table
            .filter(handlers::deployment_id.eq(deployment_id))
            .load::<Handler>(&conn)?;
        Ok(results)
    }

    /// Create a handler
    pub fn create(handler: HandlerFormat, deployment_id: Uuid) -> Result<Self, ApiError> {
        let conn = database::connection()?;

        // Set unused values to null rather than empty array/object
        let query_parameters = array_is_empty(handler.query_parameters);
        let headers = array_is_empty(handler.headers);
        let path_parameters = array_is_empty(handler.path_parameters);
        let body = if let Some(b) = handler.body {
            if b.is_object() {
                Some(b)
            } else {
                None
            }
        } else {
            None
        };

        let handler = diesel::insert_into(handlers::table)
            .values(Handler {
                id: Uuid::new_v4(),
                deployment_id,
                name: handler.name,
                query_parameters,
                headers,
                path_parameters,
                body,
                logic: handler.logic,
            })
            .get_result(&conn)?;
        Ok(handler)
    }

    /// Delete a handler
    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let conn = database::connection()?;

        let res = diesel::delete(handlers::table)
            .filter(handlers::id.eq(id))
            .execute(&conn)?;
        Ok(res)
    }
}

fn array_is_empty<T>(arr: Option<Vec<T>>) -> Option<Vec<T>> {
    if let Some(arr) = arr {
        if arr.len() != 0 {
            return Some(arr);
        }
    }
    None
}
