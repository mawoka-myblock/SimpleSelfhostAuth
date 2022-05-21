use diesel::pg::PgConnection;
use diesel::r2d2::{self};
use dotenv::dotenv;
use r2d2::{Pool, ConnectionManager};
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> DbPool {
    // it from the environment within this function
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("no DB URL");
    let migr = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .build(migr)
        .expect("could not build connection pool")
}


