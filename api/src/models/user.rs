use crate::{database, errors::ApiError, schema::users};
use argon2::Config;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UserMessage {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Associations, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: NaiveDateTime,
}

impl User {
    /// Find a user by their ID
    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = database::connection()?;

        let user = users::table.filter(users::id.eq(id)).first(&conn)?;
        Ok(user)
    }

    /// Find a user by their email
    pub fn find_by_email(email: String) -> Result<Self, ApiError> {
        let conn = database::connection()?;

        let user = users::table.filter(users::email.eq(email)).first(&conn)?;
        Ok(user)
    }

    /// Register a user
    pub fn create(user: UserMessage) -> Result<Self, ApiError> {
        let conn = database::connection()?;

        let mut user = User::from(user);
        user.hash_password()?;

        // Handle conflict errors
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&conn);
        match user {
            Ok(user) => Ok(user),
            Err(DieselError::DatabaseError(kind, err)) => match kind {
                DatabaseErrorKind::UniqueViolation => {
                    let field = match err.constraint_name().unwrap() {
                        "users_email_key" => "email",
                        "users_username_key" => "username",
                        _ => unreachable!("unexpected constraint on field"),
                    };
                    Err(ApiError::new(
                        409,
                        format!("user with specified {} already exists", field),
                    ))
                }
                _ => Err(ApiError::new(
                    500,
                    format!("Unexpected database error: {}", err.message()),
                )),
            },
            Err(err) => Err(ApiError::new(500, format!("Diesel error: {}", err))),
        }
    }

    /// Update a user's information
    pub fn update(id: Uuid, mut user: UserMessage) -> Result<Self, ApiError> {
        let conn = database::connection()?;

        user.username = String::from("");

        let user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(user)
            .get_result(&conn)?;

        Ok(user)
    }

    /// Delete a user from the database
    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let conn = database::connection()?;

        let res = diesel::delete(users::table.filter(users::id.eq(id))).execute(&conn)?;
        Ok(res)
    }

    /// Hash a user's password
    pub fn hash_password(&mut self) -> Result<(), ApiError> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config {
            variant: argon2::Variant::Argon2id,
            ..Config::default()
        };

        self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
            .map_err(|e| ApiError::new(500, format!("Failed to hash password: {}", e)))?;

        Ok(())
    }

    /// Verify a user's password
    pub fn verify_password(&self, password: &[u8]) -> Result<bool, ApiError> {
        argon2::verify_encoded(&self.password, password)
            .map_err(|e| ApiError::new(500, format!("Failed to verify password: {}", e)))
    }
}

impl From<UserMessage> for User {
    fn from(user: UserMessage) -> Self {
        User {
            id: Uuid::new_v4(),
            email: user.email,
            username: user.username,
            password: user.password,
            created_at: Utc::now().naive_utc(),
        }
    }
}
