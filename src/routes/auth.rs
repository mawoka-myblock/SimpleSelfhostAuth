use actix_web::{post, get, Error, HttpResponse, http::header, HttpRequest};
use actix_web::web::{self};
use actix_identity::Identity;
use crate::db::DbPool;
use crate::{actions, models};
use serde::{Deserialize};
use crate::models::{CreateUser};
use crate::schema::users::dsl::users;


#[get("/auth")]
pub async fn proxy_auth(id: Identity, pool: web::Data<DbPool>, req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("Req came in!");
    // req.headers().get(
    let user = match id.identity() {
        Some(u) => u,
        // None => return Ok(HttpResponse::Unauthorized().finish())
        None => return Ok(HttpResponse::Unauthorized().finish())
    };
    let request_uri = match req.headers().get(header::HeaderName::from_lowercase(b"x-original-uri").unwrap()) {
        Some(val) => val,
        None => return Ok(HttpResponse::BadRequest().body("x-original-uri header missing"))
    };
    let remote_addr = match req.headers().get(header::HeaderName::from_lowercase(b"x-original-remote-addr").unwrap()) {
        Some(val) => val,
        None => return Ok(HttpResponse::BadRequest().body("x-original-remote-addr header missing"))
    };
    let host = match req.headers().get(header::HeaderName::from_lowercase(b"x-original-host").unwrap()) {
        Some(val) => val,
        None => return Ok(HttpResponse::BadRequest().body("x-original-host header missing"))
    };

    println!("{:?}, {:?}, {:?}", request_uri, remote_addr, host);


    Ok(HttpResponse::Ok().finish())
}