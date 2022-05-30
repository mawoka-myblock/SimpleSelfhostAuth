use crate::actions::{totp_type_string_to_totp_enum, DbError};
use crate::models::{CreateUser, PatchUser, PrivateUser, User};
use crate::schema;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::prelude::*;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use totp_rs::{Algorithm, TOTP};
use uuid::Uuid;

fn user_to_private_user(pu: &User) -> PrivateUser {
    PrivateUser {
        id: pu.id,
        username: pu.username.to_string(),
        profile_pic: pu.profile_pic.as_ref().cloned(),
        email: pu.email.to_string(),
        created_at: pu.created_at,
        admin: pu.admin,
        scopes: pu.scopes.to_vec(),
        totp_type: match pu.clone().two_factor {
            Some(t) => Some(totp_type_string_to_totp_enum(&t)),
            None => None,
        },
    }
}

#[derive(Serialize, Deserialize)]
pub struct SetupTOTPResponse {
    pub url: String,
    pub qr_code: String,
    pub secret: String,
}

pub fn setup_totp_auth(user_id: Uuid, conn: &PgConnection) -> Result<SetupTOTPResponse, DbError> {
    use rand::prelude::*;
    use rand_chacha::ChaCha20Rng;
    use schema::users::dsl::{totp_data, two_factor, users};
    let mut rng = ChaCha20Rng::from_entropy();
    let mut res = [0u8; 32];
    rng.fill(&mut res);
    let res = hex::encode(res);
    let target = users.find(user_id);
    diesel::update(target)
        .set((totp_data.eq(Some(&res)), two_factor.eq("Totp")))
        .execute(conn)?;
    let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, res);
    Ok(SetupTOTPResponse {
        secret: totp.get_secret_base32(),
        url: totp.get_url("SimpleSelfhostAuth", "SimpleSelfhostAuth"),
        qr_code: match totp.get_qr("SimpleSelfhostAuth", "SimpleSelfhostAuth") {
            Ok(t) => t,
            Err(_) => "".to_string(),
        },
    })
}

pub fn check_totp_token(totp_token: String, totp_secret: String) -> bool {
    let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, totp_secret);
    totp.check_current(&*totp_token).unwrap_or(false)
}

pub fn get_user_by_email_and_password(
    email_in: String,
    password_in: String,
    conn: &PgConnection,
) -> Result<Option<User>, DbError> {
    use schema::users::dsl::{email, users};
    let user = users.filter(email.eq(email_in)).first::<User>(conn);
    match user {
        Ok(user) => {
            let hash = PasswordHash::new(&user.password).unwrap();
            match Argon2::default().verify_password(password_in.as_bytes(), &hash) {
                Ok(_) => Ok(Some(user)),
                Err(_) => Ok(None),
            }
        }
        Err(_) => Ok(None),
    }
}

pub fn get_user_by_username_and_password(
    username_in: String,
    password_in: String,
    conn: &PgConnection,
) -> Result<Option<User>, DbError> {
    use schema::users::dsl::{username, users};
    let user = users.filter(username.eq(username_in)).first::<User>(conn);
    match user {
        Ok(user) => {
            let hash = PasswordHash::new(&user.password).unwrap();
            match Argon2::default().verify_password(password_in.as_bytes(), &hash) {
                Ok(_) => Ok(Some(user)),
                Err(_) => Ok(None),
            }
        }
        Err(_) => Ok(None),
    }
}

pub fn create_user(user_data: CreateUser, conn: &PgConnection) -> Result<PrivateUser, DbError> {
    use schema::users::table;
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(user_data.password.as_bytes(), &salt)?;
    let new_user = User {
        id: Uuid::new_v4(),
        username: user_data.username,
        password: password_hash.to_string(),
        profile_pic: None,
        email: user_data.email,
        verified: Some(false),
        created_at: chrono::Utc::now().naive_utc(),
        admin: false,
        scopes: vec![],
        totp_data: None,
        two_factor: None,
    };
    let res = diesel::insert_into(table)
        .values(&new_user)
        .get_result::<User>(conn)?;

    Ok(PrivateUser {
        id: res.id,
        username: res.username,
        profile_pic: res.profile_pic,
        email: res.email,
        created_at: res.created_at,
        admin: res.admin,
        scopes: res.scopes,
        totp_type: match res.clone().two_factor {
            Some(t) => Some(totp_type_string_to_totp_enum(&t)),
            None => None,
        },
    })
}

pub fn get_all_users(offset: i64, conn: &PgConnection) -> Result<Vec<PrivateUser>, DbError> {
    use schema::users::dsl::{created_at, users};

    let res = users
        .order_by(created_at.desc())
        .limit(10)
        .offset(offset)
        .load::<User>(conn)?;

    Ok(res.iter().map(user_to_private_user).collect())
}

pub fn get_single_private_user(
    input_id: uuid::Uuid,
    conn: &PgConnection,
) -> Result<PrivateUser, DbError> {
    use schema::users::dsl::{id, users};
    let res = users.filter(id.eq(input_id)).first::<User>(conn)?;
    Ok(user_to_private_user(&res))
}

pub fn patch_user(data: PatchUser, conn: &PgConnection) -> Result<PrivateUser, DbError> {
    use schema::users::dsl::{id, users};
    let user = users.filter(id.eq(&data.id)).first::<User>(conn)?;

    let updated_user = User {
        id: user.id,
        username: match data.username {
            Some(t) => t,
            None => user.username,
        },
        password: match data.password {
            Some(t) => {
                let argon2 = Argon2::default();
                let salt = SaltString::generate(&mut OsRng);
                let password_hash = argon2.hash_password(t.as_bytes(), &salt)?;
                password_hash.to_string()
            }
            None => user.password,
        },
        profile_pic: match data.profile_pic {
            Some(t) => Some(t),
            None => user.profile_pic,
        },
        email: match data.email {
            Some(t) => t,
            None => user.email,
        },
        verified: match data.verified {
            Some(t) => Some(t),
            None => user.verified,
        },
        created_at: user.created_at,
        admin: match data.admin {
            Some(t) => t,
            None => user.admin,
        },
        scopes: match data.scopes {
            Some(t) => t,
            None => user.scopes,
        },
        totp_data: user.totp_data,
        two_factor: user.two_factor,
    };
    let target = users.find(data.id);
    diesel::update(target).set(&updated_user).execute(conn)?;
    Ok(user_to_private_user(&updated_user))
}

pub fn delete_user(id: Uuid, conn: &PgConnection) -> Result<usize, DbError> {
    use schema::users::dsl::users;
    Ok(diesel::delete(users.find(id)).execute(conn)?)
}

pub fn get_single_user(input_id: uuid::Uuid, conn: &PgConnection) -> Result<User, DbError> {
    use schema::users::dsl::{id, users};
    let res = users.filter(id.eq(input_id)).first::<User>(conn)?;
    Ok(res)
}

pub fn deactivate_totp(input_id: uuid::Uuid, conn: &PgConnection) -> Result<bool, DbError> {
    use schema::users::dsl::{totp_data, two_factor, users};
    let target = users.find(input_id);
    diesel::update(target)
        .set((
            totp_data.eq::<Option<String>>(None),
            two_factor.eq::<Option<String>>(None),
        ))
        .execute(conn)?;
    Ok(true)
}

pub fn get_user_by_email(email_in: String, conn: &PgConnection) -> Result<Option<User>, DbError> {
    use schema::users::dsl::{email, users};
    let user = users.filter(email.eq(email_in)).first::<User>(conn);
    match user {
        Ok(user) => Ok(Some(user)),

        Err(_) => Ok(None),
    }
}

pub fn get_user_by_username(
    username_in: String,
    conn: &PgConnection,
) -> Result<Option<User>, DbError> {
    use schema::users::dsl::{username, users};
    let user = users.filter(username.eq(username_in)).first::<User>(conn);
    match user {
        Ok(user) => Ok(Some(user)),

        Err(_) => Ok(None),
    }
}
