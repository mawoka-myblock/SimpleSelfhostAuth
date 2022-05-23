use actix_identity::Identity;
use actix_web::web::{self};
use actix_web::{post, Error, HttpResponse};
// use deadpool_redis::{Pool, redis::{cmd}};
use crate::actions::{app, parse_identity};
use crate::db::DbPool;
use crate::models;
// use crate::models::App;
/*
#[get("/create")]
pub async fn create_app(redis: web::Data<Pool>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let mut conn = redis.get().await.unwrap();
    let redis_res: Option<String> = cmd("GET")
        .arg(&["test"])
        .query_async(&mut conn)
        .await
        .unwrap();

    let app: App = match redis_res {
        Some(s) => {
            let a: App = serde_json::from_str(&*s).unwrap();
            a
        }
        None => {
            web::block(move || {
                let db_conn = pool.get()?;
                get_app_from_domain("test.com", &db_conn)
            }).await?
                .map_err(actix_web::error::ErrorInternalServerError)?
        }
    };


    /*    cmd("SET")
            .arg(&["test", "hi"])
            .query_async::<_, ()>(&mut conn1)
            .await.unwrap();*/
    Ok(HttpResponse::Ok().finish())
}
*/
