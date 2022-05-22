use actix_identity::Identity;
use actix_web::web::{self};
use actix_web::{get, patch, Error, HttpResponse};
// use deadpool_redis::{Pool, redis::{cmd}};
use crate::actions::{parse_identity, self};
use crate::db::DbPool;
use crate::models;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GetUsersQuery {
    offset: i64,
}

#[get("/users")]
pub async fn get_users(id: Identity, query: web::Query<GetUsersQuery>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let offset = query.offset;
    match parse_identity(id) {
        Some(u) => u,
        None => return Ok(HttpResponse::Unauthorized().finish())
    };

    let res = web::block(move || {
        let conn = pool.get()?;
        actions::user::get_all_users(offset, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Serialize, Deserialize)]
pub struct GetUserQuery {
    id: uuid::Uuid,
}

#[get("/user")]
pub async fn get_user(id: Identity, query: web::Query<GetUserQuery>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let user_id = query.id;
    match parse_identity(id) {
        Some(u) => u,
        None => return Ok(HttpResponse::Unauthorized().finish())
    };

    let res = web::block(move || {
        let conn = pool.get()?;
        actions::user::get_single_private_user(user_id, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorNotFound)?;
    Ok(HttpResponse::Ok().json(res))
}

#[patch("/user")]
pub async fn patch_user(data: web::Json<models::PatchUser>, id: Identity, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    match parse_identity(id) {
        Some(u) => u,
        None => return Ok(HttpResponse::Unauthorized().finish())
    };
    let res = web::block(move || {
        let conn = pool.get()?;
        actions::user::patch_user(data.into_inner(), &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorNotFound)?;
    Ok(HttpResponse::Ok().json(res))
}