use super::chrono;
use super::schema::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize, AsChangeset, Insertable)]
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
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct CreateUser {
    // id: Uuid,
    pub username: String,
    pub password: String,
    pub profile_pic: Option<String>,
    pub email: String,
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

#[derive(Deserialize, Serialize, Queryable)]
pub struct PrivateUser {
    pub id: Uuid,
    pub username: String,
    pub profile_pic: Option<String>,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub admin: bool,
}