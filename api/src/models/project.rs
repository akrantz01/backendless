use crate::{database, errors::ApiError, models::User, schema::projects};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "projects"]
pub struct ProjectMessage {
    pub name: String,
    pub description: String,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Associations, Identifiable)]
#[belongs_to(User)]
#[table_name = "projects"]
pub struct Project {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Project {
    /// Retrieve all projects for a user
    pub fn find_all(user_id: Uuid) -> Result<Vec<Self>, ApiError> {
        let conn = database::connection()?;

        let results = projects::table
            .filter(projects::user_id.eq(user_id))
            .load::<Project>(&conn)?;
        Ok(results)
    }

    /// Find a project by ID
    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = database::connection()?;

        let project = projects::table.filter(projects::id.eq(id)).first(&conn)?;
        Ok(project)
    }

    /// Create a project
    pub fn create(project: ProjectMessage, uid: Uuid) -> Result<Self, ApiError> {
        let conn = database::connection()?;

        let project = diesel::insert_into(projects::table)
            .values(Project {
                id: Uuid::new_v4(),
                user_id: uid,
                name: project.name,
                description: project.description,
                created_at: Utc::now().naive_utc(),
                updated_at: None,
            })
            .get_result(&conn)?;
        Ok(project)
    }

    /// Update a project
    pub fn update(id: Uuid, mut project: ProjectMessage) -> Result<Self, ApiError> {
        let conn = database::connection()?;

        project.updated_at = Some(Utc::now().naive_utc());

        let project = diesel::update(projects::table)
            .filter(projects::id.eq(id))
            .set(project)
            .get_result(&conn)?;

        Ok(project)
    }

    /// Delete a project
    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let conn = database::connection()?;

        let res = diesel::delete(projects::table)
            .filter(projects::id.eq(id))
            .execute(&conn)?;
        Ok(res)
    }
}
