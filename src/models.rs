use super::chrono;
use super::schema::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize, AsChangeset, Insertable)]
#[primary_key(id)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub profile_pic: Option<String>,
    pub email: String,
    pub verified: Option<bool>,
    pub created_at: chrono::NaiveDateTime,
    pub admin: bool,
    pub scopes: Vec<String>
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct CreateUser {
    // id: Uuid,
    pub username: String,
    pub password: String,
    pub profile_pic: Option<String>,
    pub email: String,
    pub scopes: Vec<String>
    // verified: Option<bool>,
    // created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Serialize, Queryable)]
pub struct PublicUser {
    pub id: Uuid,
    pub username: String,
    pub profile_pic: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub admin: bool,
}

#[derive(Deserialize, Serialize, Queryable, Debug, Clone)]
pub struct PrivateUser {
    pub id: Uuid,
    pub username: String,
    pub profile_pic: Option<String>,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub admin: bool,
    pub scopes: Vec<String>
}

#[derive(
    Debug, Clone, Queryable, Serialize, Deserialize, AsChangeset, Insertable, Associations,
)]
#[belongs_to(foreign_key = id)]
#[primary_key(id)]
#[table_name = "apps"]
pub struct App {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub token_lifetime: i32,
    pub domains: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateAppInput {
    pub name: String,
    pub description: Option<String>,
    pub token_lifetime: i32,
    pub domains: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Associations, Queryable)]
#[belongs_to(foreign_key = id)]
#[table_name = "apps"]
pub struct CreateApp {
    pub name: String,
    pub description: Option<String>,
    pub token_lifetime: i32,
    pub owner: Uuid,
    pub domains: Vec<String>,
}
