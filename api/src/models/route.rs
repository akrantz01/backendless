use crate::{
    database, errors::ApiError, models::Deployment, project_format::Route as RouteFormat,
    schema::routes,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Associations, Identifiable)]
#[belongs_to(Deployment)]
#[table_name = "routes"]
pub struct Route {
    pub id: Uuid,
    pub deployment_id: Uuid,
    pub path: String,
    pub methods: Vec<String>,
    pub handler: String,
}

impl Route {
    /// Retrieve all routes for a deployment
    pub fn find_all(deployment_id: Uuid) -> Result<Vec<Self>, ApiError> {
        let conn = database::connection()?;

        let results = routes::table
            .filter(routes::deployment_id.eq(deployment_id))
            .load::<Route>(&conn)?;
        Ok(results)
    }

    /// Create a route
    pub fn create(route: RouteFormat, deployment_id: Uuid) -> Result<Self, ApiError> {
        let conn = database::connection()?;

        let route = diesel::insert_into(routes::table)
            .values(Route {
                id: Uuid::new_v4(),
                path: route.path,
                methods: route.methods,
                handler: route.handler,
                deployment_id,
            })
            .get_result(&conn)?;
        Ok(route)
    }

    /// Delete a route
    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let conn = database::connection()?;

        let res = diesel::delete(routes::table)
            .filter(routes::id.eq(id))
            .execute(&conn)?;
        Ok(res)
    }
}
