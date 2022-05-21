use crate::models::{User, CreateUser, PrivateUser};
use diesel::prelude::*;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use uuid::Uuid;
use crate::actions::DbError;
use crate::schema;

pub fn get_user_by_email_and_password(email_in: String, password_in: String, conn: &PgConnection) -> Result<Option<User>, DbError> {
    use schema::users::dsl::{email, users};
    let user = users.filter(email.eq(email_in)).first::<User>(conn);
    match user {
        Ok(user) => {
            let hash = PasswordHash::new(&user.password).unwrap();
            match Argon2::default().verify_password(password_in.as_bytes(), &hash) {
                Ok(_) => Ok(Some(user)),
                Err(_) => Ok(None)
            }
        }
        Err(_) => Ok(None),
    }
}

pub fn get_user_by_username_and_password(username_in: String, password_in: String, conn: &PgConnection) -> Result<Option<User>, DbError> {
    use schema::users::dsl::{username, users};
    let user = users.filter(username.eq(username_in)).first::<User>(conn);
    match user {
        Ok(user) => {
            let hash = PasswordHash::new(&user.password).unwrap();
            match Argon2::default().verify_password(password_in.as_bytes(), &hash) {
                Ok(_) => Ok(Some(user)),
                Err(_) => Ok(None)
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
    };
    let res = diesel::insert_into(table).values(&new_user).get_result::<User>(conn)?;

    Ok(PrivateUser {
        id: res.id,
        username: res.username,
        profile_pic: res.profile_pic,
        email: res.email,
        created_at: res.created_at,
        admin: res.admin
    })
}