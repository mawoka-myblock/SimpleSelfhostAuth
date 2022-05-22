use crate::db::DbPool;
use actix_identity::Identity;
use actix_web::web::{self};
use actix_web::{get, http::header, Error, HttpRequest, HttpResponse};
use deadpool_redis::{Pool, redis::{cmd}};
use crate::actions::{parse_identity};
use crate::actions::app as actions;
use crate::models::App;

#[get("/auth")]
pub async fn proxy_auth(redis: web::Data<Pool>,
                        id: Identity,
                        pool: web::Data<DbPool>,
                        req: HttpRequest,
) -> Result<HttpResponse, Error> {
    println!("Req came in!");
    let user = match parse_identity(id) {
        Some(u) => u,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };
    let request_uri = match req
        .headers()
        .get(header::HeaderName::from_lowercase(b"x-original-uri").unwrap())
    {
        Some(val) => val,
        None => return Ok(HttpResponse::BadRequest().body("x-original-uri header missing")),
    };
    let remote_addr = match req
        .headers()
        .get(header::HeaderName::from_lowercase(b"x-original-remote-addr").unwrap())
    {
        Some(val) => val,
        None => return Ok(HttpResponse::BadRequest().body("x-original-remote-addr header missing")),
    };
    let host = match req
        .headers()
        .get(header::HeaderName::from_lowercase(b"x-original-host").unwrap())
    {
        Some(val) => val,
        None => return Ok(HttpResponse::BadRequest().body("x-original-host header missing")),
    };
    let mut conn = redis.get().await.unwrap();
    let redis_res: Option<String> = cmd("GET")
        .arg(&[format!("apps:domain:{}", host.to_str().unwrap())]) // TODO not the smartest option
        .query_async(&mut conn)
        .await
        .unwrap();
    let host2 = host.clone();
    println!("here");
    let app = match redis_res {
        Some(t) => {
            let a: App = serde_json::from_str(&*t).unwrap();
            a
        }
        None => {
            let app = web::block(move || {
                let conn = pool.get()?;
                actions::get_app_from_domain(host2.to_str().unwrap(), &conn)
            }).await?.map_err(actix_web::error::ErrorNotFound)?;
            app
        }
    };
    println!("{:?}, {:?}, {:?}", request_uri, remote_addr, host);
    return if user.admin || user.scopes.contains(&format!("app:{}", &app.name)) {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Unauthorized().body("You don't have the rights to access this app."))
    }
}




