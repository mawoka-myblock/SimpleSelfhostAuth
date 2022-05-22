pub mod app;
pub mod user;
use crate::models::{PrivateUser};
use actix_identity::Identity;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn parse_identity(id: Identity) -> Option<PrivateUser> {
    match id.identity() {
        Some(t) => {
            let user: PrivateUser = serde_json::from_str(&t).unwrap();
            Some(user)
        }
        None => None,
    }
}
