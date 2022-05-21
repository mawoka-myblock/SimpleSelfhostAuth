pub mod db;
pub mod models;
pub mod routes;
pub mod schema;
pub mod actions;
pub mod templates;

extern crate chrono;

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate diesel_migrations;

use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, App, HttpServer};
// use diesel::prelude::*;
// use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::embed_migrations;


embed_migrations!();
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let pool = db::get_pool();

    let conn = pool.get().unwrap();
    embedded_migrations::run(&conn).unwrap();

    HttpServer::new(move || {
        let policy = CookieIdentityPolicy::new(&[0; 32])
            .name("auth-cookie")
            .secure(false);
        let cors = Cors::default().allow_any_origin();

        App::new()
            .wrap(IdentityService::new(policy))
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/users")
                            .service(routes::users::login) // POST /login
                            .service(routes::users::create_user) // POST /create
                    )
            )
            .service(routes::auth::proxy_auth)
            .service(web::scope("/account")
                .service(routes::frontend::login)
                .service(routes::users::login))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
