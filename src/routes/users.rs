use crate::db::DbPool;
use crate::models::CreateUser;
use crate::{actions, models};
use actix_identity::Identity;
use actix_web::web::{self};
use actix_web::{get, post, Error, HttpResponse};
use serde::Deserialize;
use crate::actions::user::check_totp_token;

#[derive(Deserialize, Clone)]
pub struct LoginInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
    pub stay_logged_in: bool,
    pub totp: Option<String>,
}

#[post("/login")]
pub async fn login(
    id: Identity,
    pool: web::Data<DbPool>,
    data: web::Json<LoginInput>,
) -> Result<HttpResponse, Error> {
    let stay_logged_in = data.stay_logged_in;
    let totp_token = data.0.clone().totp;
    let totp_token_is_some = data.totp.is_some();
    let user: HttpResponse = match &data.email {
        Some(_) => {
            let res = web::block(move || {
                let conn = pool.get()?;
                actions::user::get_user_by_email_and_password(
                    data.email.as_ref().unwrap().to_string(),
                    data.password.to_string(),
                    &conn,
                )
            })
                .await?
                .map_err(actix_web::error::ErrorInternalServerError)?;
            match res {
                Some(user) => {
                    let u = models::PrivateUser {
                        id: user.id,
                        username: user.username,
                        profile_pic: user.profile_pic,
                        email: user.email,
                        created_at: user.created_at,
                        admin: user.admin,
                        scopes: user.scopes,
                    };
                    if totp_token_is_some && user.totp_token.is_some() {
                        if check_totp_token(totp_token.as_ref().unwrap().to_string(), user.totp_token.unwrap()) {} else {
                            return Ok(HttpResponse::Unauthorized().body("TOTP invalid"));
                        }
                    }
                    id.remember(match actions::create_jwt(u.clone(), stay_logged_in) {
                        Some(t) => t,
                        None => return Ok(HttpResponse::InternalServerError().finish())
                    });
                    HttpResponse::Ok().json(u)
                }
                None => HttpResponse::NotFound().finish(),
            }
        }
        None => match &data.username {
            Some(_) => {
                let res = web::block(move || {
                    let conn = pool.get()?;
                    actions::user::get_user_by_username_and_password(
                        data.username.as_ref().unwrap().to_string(),
                        data.password.to_string(),
                        &conn,
                    )
                })
                    .await?
                    .map_err(actix_web::error::ErrorInternalServerError)?;
                match res {
                    Some(user) => {
                        let u = models::PrivateUser {
                            id: user.id,
                            username: user.username,
                            profile_pic: user.profile_pic,
                            email: user.email,
                            created_at: user.created_at,
                            admin: user.admin,
                            scopes: user.scopes,
                        };
                        id.remember(match actions::create_jwt(u.clone(), stay_logged_in) {
                            Some(t) => t,
                            None => return Ok(HttpResponse::InternalServerError().finish())
                        });
                        HttpResponse::Ok().json(u)
                    }
                    None => HttpResponse::NotFound().finish(),
                }
            }
            None => HttpResponse::BadRequest().finish(),
        },
    };
    Ok(user)
}

#[post("/create")]
pub async fn create_user(
    _id: Identity,
    pool: web::Data<DbPool>,
    data: web::Json<CreateUser>,
) -> Result<HttpResponse, Error> {
    let _ = web::block(move || {
        let conn = pool.get()?;
        actions::user::create_user(data.into_inner(), &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError);

    Ok(HttpResponse::Created().finish())
}

#[get("/logout")]
pub async fn logout(id: Identity) -> Result<HttpResponse, Error> {
    id.forget();
    Ok(HttpResponse::Ok().finish())
}

#[post("/setup_totp")]
pub async fn setup_totp(id: Identity, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let user = match actions::parse_identity(id) {
        Some(u) => u,
        None => return Ok(HttpResponse::Unauthorized().finish())
    };
    let res = web::block(move || {
        let conn = pool.get()?;
        actions::user::setup_totp_auth(user.id, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(res))
}

#[get("/me")]
pub async fn get_login_status(id: Identity) -> Result<HttpResponse, Error> {
    match actions::parse_identity(id) {
        Some(u) => Ok(HttpResponse::Ok().json(u)),
        None => Ok(HttpResponse::Unauthorized().finish())
    }
}