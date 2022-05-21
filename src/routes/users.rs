use actix_web::{post, Error, HttpResponse, get};
use actix_web::web::{self};
use actix_identity::Identity;
use crate::db::DbPool;
use crate::{actions, models};
use serde::{Deserialize};
use crate::models::{CreateUser};


#[derive(Deserialize)]
pub struct LoginInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

#[post("/login")]
pub async fn login(id: Identity, pool: web::Data<DbPool>, data: web::Json<LoginInput>) -> Result<HttpResponse, Error> {
    let user: HttpResponse = match &data.email {
        Some(_) => {
            let res = web::block(move || {
                let conn = pool.get()?;
                actions::user::get_user_by_email_and_password(data.email.as_ref().unwrap().to_string(), data.password.to_string(), &conn)
            })
                .await?
                .map_err(actix_web::error::ErrorInternalServerError)?;
            match res {
                Some(user) => {
                    id.remember(user.id.to_string());
                    HttpResponse::Ok().json(models::PrivateUser {
                        id: user.id,
                        username: user.username,
                        profile_pic: user.profile_pic,
                        email: user.email,
                        created_at: user.created_at,
                        admin: user.admin,
                    })
                }
                None => HttpResponse::NotFound().finish()
            }
        }
        None => match &data.username {
            Some(_) => {
                let res = web::block(move || {
                    let conn = pool.get()?;
                    actions::user::get_user_by_username_and_password(data.username.as_ref().unwrap().to_string(), data.password.to_string(), &conn)
                })
                    .await?
                    .map_err(actix_web::error::ErrorInternalServerError)?;
                match res {
                    Some(user) => {
                        id.remember(user.id.to_string());
                        HttpResponse::Ok().json(models::PrivateUser {
                            id: user.id,
                            username: user.username,
                            profile_pic: user.profile_pic,
                            email: user.email,
                            created_at: user.created_at,
                            admin: user.admin,
                        })
                    }
                    None => HttpResponse::NotFound().finish()
                }
            }
            None => HttpResponse::BadRequest().finish()
        }
    };
    Ok(user)
}

#[post("/create")]
pub async fn create_user(id: Identity, pool: web::Data<DbPool>, data: web::Json<CreateUser>) -> Result<HttpResponse, Error> {
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