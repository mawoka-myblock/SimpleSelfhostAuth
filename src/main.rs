pub mod actions;
pub mod db;
pub mod models;
pub mod routes;
pub mod schema;
pub mod templates;

use std::env;

extern crate chrono;

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate lazy_static;

use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, App, HttpServer};
use deadpool_redis::Config;

use diesel_migrations::embed_migrations;
// use futures::future::{join_all, Future};

embed_migrations!();
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let pool = db::get_pool();

    let conn = pool.get().unwrap();
    let redis_url = env::var("REDIS_URL").expect("no DB URL");
    env::var("SECRET_KEY").expect("not SECRET_KEY");
    let cfg = Config::from_url(redis_url);
    let pool_redis = cfg.create_pool(None).unwrap();
    embedded_migrations::run(&conn).unwrap();

    HttpServer::new(move || {
        let policy = CookieIdentityPolicy::new(&[0; 32])
            .name("auth-cookie")
            .secure(false);
        let cors = Cors::default().allow_any_origin();

        App::new()
            .wrap(IdentityService::new(policy))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(pool_redis.clone()))
            .wrap(cors)
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/users")
                            .service(routes::users::login) // POST /login
                            .service(routes::users::create_user) // POST /create
                            .service(routes::users::setup_totp) // POST /setup_totp
                            .service(routes::users::get_login_status), // GET /check
                    )
                    .service(
                        web::scope("/admin")
                            .service(routes::admin::get_users) // GET /users?offset=Int
                            .service(routes::admin::get_user) // GET /user?id=UUID
                            .service(routes::admin::patch_user) // PATCH /user
                            .service(routes::admin::get_apps) // GET /apps
                            .service(routes::admin::create_app) // POST /app
                            .service(routes::admin::get_app) // GET /app?id=UUID
                            .service(routes::admin::patch_app) // PATCH /app
                            .service(routes::admin::delete_user) // DELETE /user?id=UUID
                            .service(routes::admin::delete_app) // DELETE /app?id=UUID
                    ),
            )
            .service(routes::auth::proxy_auth) // GET /auth
            .service(
                web::scope("/account")
                    .service(routes::frontend::login) // GET /login
                    .service(routes::users::login), // POST /login
            )
            .service(routes::frontend::index)
            .service(routes::frontend::dist)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
