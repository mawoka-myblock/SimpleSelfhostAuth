use actix_identity::Identity;
use actix_web::web::{self};
use actix_web::{delete, get, patch, post, Error, HttpResponse};
// use deadpool_redis::{Pool, redis::{cmd}};
use crate::actions::{self, parse_identity};
use crate::db::DbPool;
use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetUsersQuery {
    offset: i64,
}

#[get("/users")]
pub async fn get_users(
    id: Identity,
    query: web::Query<GetUsersQuery>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let offset = query.offset;
    match parse_identity(id) {
        Some(u) => {
            if !u.admin {
                return Ok(HttpResponse::Unauthorized().finish());
            }
        }
        None => return Ok(HttpResponse::Unauthorized().finish()),
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
pub async fn get_user(
    id: Identity,
    query: web::Query<GetUserQuery>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let user_id = query.id;
    match parse_identity(id) {
        Some(u) => {
            if !u.admin {
                return Ok(HttpResponse::Unauthorized().finish());
            }
        }
        None => return Ok(HttpResponse::Unauthorized().finish()),
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
pub async fn patch_user(
    data: web::Json<models::PatchUser>,
    id: Identity,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let user = match parse_identity(id) {
        Some(u) => u,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };
    if !user.admin || user.id != data.id {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    let res = web::block(move || {
        let conn = pool.get()?;
        actions::user::patch_user(data.into_inner(), &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorNotFound)?;
    Ok(HttpResponse::Ok().json(res))
}

#[get("/apps")]
pub async fn get_apps(
    id: Identity,
    query: web::Query<GetUsersQuery>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let offset = query.offset;
    match parse_identity(id) {
        Some(u) => {
            if !u.admin {
                return Ok(HttpResponse::Unauthorized().finish());
            }
        }
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let res = web::block(move || {
        let conn = pool.get()?;
        actions::app::list_apps(offset, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(res))
}

#[post("/app")]
pub async fn create_app(
    id: Identity,
    pool: web::Data<DbPool>,
    data: web::Json<models::AppInput>,
) -> Result<HttpResponse, Error> {
    let user = match parse_identity(id) {
        Some(u) => {
            if !u.admin {
                return Ok(HttpResponse::Unauthorized().finish());
            }
            u
        }
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };
    let created_app = web::block(move || {
        let conn = pool.get()?;
        actions::app::create_app(
            data.into_inner(),
            actions::app::Input::PrivateUser(user),
            &conn,
        )
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(created_app))
}

#[patch("/app")]
pub async fn patch_app(
    data: web::Json<models::PatchApp>,
    id: Identity,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let user = match parse_identity(id) {
        Some(u) => {
            if !u.admin {
                return Ok(HttpResponse::Unauthorized().finish());
            }
            u
        }
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };
    let res = web::block(move || {
        let conn = pool.get()?;
        actions::app::patch_app(data.into_inner(), user, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorNotFound)?;
    Ok(HttpResponse::Ok().json(res))
}

#[get("/app")]
pub async fn get_app(
    id: Identity,
    query: web::Query<GetUserQuery>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let app_id = query.id;
    match parse_identity(id) {
        Some(u) => {
            if !u.admin {
                return Ok(HttpResponse::Unauthorized().finish());
            }
        }
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let res = web::block(move || {
        let conn = pool.get()?;
        actions::app::get_app_from_id(app_id, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorNotFound)?;
    Ok(HttpResponse::Ok().json(res))
}

#[delete("/user")]
pub async fn delete_user(
    id: Identity,
    query: web::Query<GetUserQuery>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let user_id = query.id;
    match parse_identity(id) {
        Some(u) => {
            if !u.admin || u.id != user_id {
                return Ok(HttpResponse::Unauthorized().finish());
            }
        }
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    web::block(move || {
        let conn = pool.get()?;
        actions::user::delete_user(user_id, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorNotFound)?;
    Ok(HttpResponse::Ok().finish())
}

#[delete("/app")]
pub async fn delete_app(
    id: Identity,
    query: web::Query<GetUserQuery>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let app_id = query.id;
    match parse_identity(id) {
        Some(u) => {
            if !u.admin {
                return Ok(HttpResponse::Unauthorized().finish());
            }
        }
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    web::block(move || {
        let conn = pool.get()?;
        actions::app::delete_app_from_id(app_id, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorNotFound)?;
    Ok(HttpResponse::Ok().finish())
}
