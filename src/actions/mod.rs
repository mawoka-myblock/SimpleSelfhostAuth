pub mod user;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;
