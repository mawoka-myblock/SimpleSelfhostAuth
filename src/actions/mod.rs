pub mod app;
pub mod user;

use crate::models::TotpType;
use crate::models::{App, PrivateUser};
use actix_identity::Identity;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use time::{Duration, OffsetDateTime};

pub type DbError = Box<dyn std::error::Error + Send + Sync>;
lazy_static! {
    static ref HEADER: Header = Header::new(Algorithm::EdDSA);
    static ref VALIDATION: Validation = Validation::new(Algorithm::EdDSA);
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user: PrivateUser,
    exp: usize,
}

pub fn parse_identity(id: Identity) -> Option<PrivateUser> {
    let secret = env::var("SECRET_KEY").expect("not SECRET_KEY");
    match id.identity() {
        Some(t) => {
            // let user: PrivateUser = serde_json::from_str(&t).unwrap();
            // Some(user)
            let token = decode::<Claims>(
                &t,
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::default(),
            );
            match token {
                Ok(res) => Some(res.claims.user),
                Err(_) => None,
            }
        }
        None => None,
    }
}

pub fn create_jwt(user: PrivateUser, long: bool) -> Option<String> {
    let time = OffsetDateTime::now_utc();
    let dur = match long {
        true => Duration::days(30),
        false => Duration::hours(1),
    };
    let unix_ts = time + dur;
    let claims = Claims {
        exp: unix_ts.unix_timestamp() as usize,
        user,
    };
    let secret = env::var("SECRET_KEY").expect("no SECRET_KEY");
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    );
    return match token {
        Err(e) => {
            println!("{:?}", e);
            None
        }
        Ok(s) => Some(s),
    };
}

pub fn check_if_user_has_rights_to_access_app(user: PrivateUser, app: App) -> bool {
    if user.admin {
        return true;
    };
    if app.enforce_totp && user.totp_type.is_none() {
        return false;
    };
    if user.scopes.contains(&format!("app:{}", &app.name)) {
        return true;
    };

    for domain in app.domains {
        if user.scopes.contains(&format!("domain:{}", domain)) {
            return true;
        }
    }

    false
}

pub fn totp_type_string_to_totp_enum(data: &str) -> TotpType {
    match data {
        "Totp" => TotpType::Totp,
        "Gotify" => TotpType::Gotify,
        "Ntfy" => TotpType::Ntfy,
        _ => panic!("Wrong type"),
    }
}
