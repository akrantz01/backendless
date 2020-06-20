use super::{Handler, Route};
use crate::{
    database,
    errors::ApiError,
    models::Project,
    project_format::{Handler as HandlerFormat, Route as RouteFormat},
    schema::deployments,
};
use chrono::{NaiveDateTime, Utc};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Clone, Debug, Serialize, Deserialize, Queryable, Insertable, Associations, Identifiable,
)]
#[belongs_to(Project)]
#[table_name = "deployments"]
pub struct Deployment {
    pub id: Uuid,
    pub project_id: Uuid,
    pub version: String,
    pub hash: String,
    pub has_static: bool,
    pub published_at: NaiveDateTime,
}

impl Deployment {
    /// Retrieve all deployments for a project
    pub fn find_all(project_id: Uuid) -> Result<Vec<Self>, ApiError> {
        let conn = database::connection()?;

        let results = deployments::table
            .filter(deployments::project_id.eq(project_id))
            .load::<Deployment>(&conn)?;
        Ok(results)
    }

    /// Get a deployment by id
    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = database::connection()?;

        let deployment = deployments::table
            .filter(deployments::id.eq(id))
            .first(&conn)?;
        Ok(deployment)
    }

    /// Get a deployment by its hash
    pub fn find_by_hash(hash: &String, project_id: Uuid) -> Result<Option<Self>, ApiError> {
        let conn = database::connection()?;

        let result = deployments::table
            .filter(deployments::project_id.eq(project_id))
            .filter(deployments::hash.eq(hash))
            .first(&conn);
        match result {
            Ok(r) => Ok(Some(r)),
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(ApiError::from(e)),
        }
    }

    /// Create a deployment
    pub fn create(version: String, hash: String, project_id: Uuid) -> Result<Self, ApiError> {
        let conn = database::connection()?;

        let deployment = diesel::insert_into(deployments::table)
            .values(Deployment {
                id: Uuid::new_v4(),
                project_id,
                version,
                hash,
                has_static: false,
                published_at: Utc::now().naive_utc(),
            })
            .get_result(&conn)?;
        Ok(deployment)
    }

    /// Delete a project
    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let conn = database::connection()?;

        let res = diesel::delete(deployments::table)
            .filter(deployments::id.eq(id))
            .execute(&conn)?;
        Ok(res)
    }

    /// Mark deployment as having static files
    pub fn mark_has_static(&self) -> Result<usize, ApiError> {
        let conn = database::connection()?;

        let deployment = diesel::update(self).set(deployments::has_static.eq(true));
        println!("{}", diesel::debug_query::<Pg, _>(&deployment));

        Ok(deployment.execute(&conn)?)
    }

    /// Add a handler to the deployment
    pub fn add_handler(&self, handler: HandlerFormat) -> Result<Handler, ApiError> {
        Ok(Handler::create(handler, self.id)?)
    }

    /// Add a route to the deployment
    pub fn add_route(&self, route: RouteFormat) -> Result<Route, ApiError> {
        Ok(Route::create(route, self.id)?)
    }
}
