use crate::actions::totp_type_string_to_totp_enum;
use crate::actions::user::{
    check_totp_token, get_single_private_user, get_user_by_email, get_user_by_email_and_password,
    get_user_by_username,
};
use crate::db::DbPool;
use crate::models::{CreateUser, TotpType};
use crate::{actions, models};
use actix_identity::Identity;
use actix_web::web::{self};
use actix_web::{delete, get, post, Error, HttpResponse};
use deadpool_redis::{redis::cmd, Pool};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct LoginInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
    pub stay_logged_in: bool,
    pub totp: Option<i32>,
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
                        totp_type: match user.two_factor {
                            Some(t) => Some(totp_type_string_to_totp_enum(&t)),
                            None => None,
                        },
                    };
                    if totp_token_is_some && user.totp_data.is_some() {
                        if check_totp_token(
                            totp_token.as_ref().unwrap().to_string(),
                            user.totp_token.unwrap(),
                        ) {} else {
                            return Ok(HttpResponse::Unauthorized().body("TOTP invalid"));
                        }
                    } else if user.totp_token.is_some() {
                        return Ok(HttpResponse::BadRequest().body("TOTP not provided"));
                    }
                    id.remember(match actions::create_jwt(u.clone(), stay_logged_in) {
                        Some(t) => t,
                        None => return Ok(HttpResponse::InternalServerError().finish()),
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
                            totp_type: match user.two_factor {
                                Some(t) => Some(totp_type_string_to_totp_enum(&t)),
                                None => None,
                            },
                        };
                        if totp_token_is_some && user.totp_token.is_some() {
                            if check_totp_token(
                                totp_token.as_ref().unwrap().to_string(),
                                user.totp_token.unwrap(),
                            ) {} else {
                                return Ok(HttpResponse::Unauthorized().body("TOTP invalid"));
                            }
                        } else if user.totp_token.is_some() {
                            return Ok(HttpResponse::BadRequest().body("TOTP not provided"));
                        }
                        id.remember(match actions::create_jwt(u.clone(), stay_logged_in) {
                            Some(t) => t,
                            None => return Ok(HttpResponse::InternalServerError().finish()),
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
    id: Identity,
    pool: web::Data<DbPool>,
    data: web::Json<CreateUser>,
) -> Result<HttpResponse, Error> {
    match actions::parse_identity(id) {
        Some(u) => u,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };
    let res = web::block(move || {
        let conn = pool.get()?;
        actions::user::create_user(data.into_inner(), &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(res))
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
        None => return Ok(HttpResponse::Unauthorized().finish()),
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
        None => Ok(HttpResponse::Unauthorized().finish()),
    }
}

#[derive(Deserialize, Clone)]
pub struct DeactivateTOTPInput {
    pub totp: i32,
}

#[delete("/totp")]
pub async fn deactivate_totp(
    id: Identity,
    query: web::Query<DeactivateTOTPInput>,
    pool: web::Data<DbPool>,
    pool2: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let totp = query.totp;
    let user = match actions::parse_identity(id.clone()) {
        Some(u) => u,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };
    if user.totp_type.is_none() {
        return Ok(HttpResponse::BadRequest().finish());
    }
    let user_all = web::block(move || {
        let conn = pool.get()?;
        actions::user::get_single_user(user.id, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
    if !check_totp_token(totp.to_string(), user_all.totp_data.unwrap()) {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    web::block(move || {
        let conn = pool2.get()?;
        actions::user::deactivate_totp(user.id, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    id.forget();
    Ok(HttpResponse::Ok().finish())
}

#[derive(Serialize, Deserialize)]
pub struct RequestTokenInput {
    pub username: Option<String>,
    pub email: Option<String>,
}

#[post("/request_token")]
pub async fn request_token(
    id: Identity,
    data: web::Json<RequestTokenInput>,
    pool: web::Data<DbPool>,
    pool2: web::Data<DbPool>,
    redis_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user;
    // START getting user
    if data.email.is_some() {
        user = web::block(move || {
            let conn = pool.get()?;
            get_user_by_email(data.0.email.unwrap(), &conn)
        })
            .await?
            .map_err(actix_web::error::ErrorUnauthorized)?;
    } else if data.username.is_some() {
        user = web::block(move || {
            let conn = pool.get()?;
            get_user_by_username(data.0.username.unwrap(), &conn)
        })
            .await?
            .map_err(actix_web::error::ErrorUnauthorized)?;
    } else {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    if user.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let user = user.unwrap();
    // END getting user
    // START getting TOTP-Type
    let totp_type = match user.two_factor {
        None => return Ok(HttpResponse::Unauthorized().finish()),
        Some(s) => totp_type_string_to_totp_enum(&s)
    };
    if totp_type == TotpType::Totp {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    // END getting TOTP-Type

    // START generating token
    let mut rng = rand::thread_rng();
    let token: String = (0..8)
        .map(|_| {
            let idx: i32 = rng.gen_range(0..10);
            idx.to_string()
        })
        .collect();
    // END generating token
    let client = reqwest::Client::new();
    if totp_type == TotpType::Ntfy {
        let base_url = match user.totp_data {
            Some(t) => t,
            None => return Ok(HttpResponse::InternalServerError().finish())
        };

        let res = client.post(base_url)
            .body(format!("Your login-token is {} and it's valid for 10 minutes.", token))
            .header("Priority", "5")
            .header("Title", "Login-Token for SSA")
            .header("Tags", "security,ssa,login")
            .header("Email", user.email)
            .send()
            .await;
        let _ = match res {
            Ok(a) => a,
            Err(_) => return Ok(HttpResponse::InternalServerError().finish())
        };
    }


    Ok(HttpResponse::Ok().finish())
}
