use crate::actions::DbError;
use crate::models::{App, AppInput, CreateApp, PatchApp, PrivateUser, User};
use crate::schema;
use diesel::prelude::*;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn get_app_from_domain(domain: &str, conn: &PgConnection) -> Result<App, DbError> {
    use schema::apps::dsl::{apps, domains};
    let app = apps
        .filter(domains.contains(vec![domain]))
        .first::<App>(conn)?;
    Ok(app)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Input {
    User(User),
    PrivateUser(PrivateUser),
}

pub fn create_app(data: AppInput, user: Input, conn: &PgConnection) -> Result<App, DbError> {
    use schema::apps::dsl::{apps, name};
    use schema::apps::table;
    let private_user = match user {
        Input::User(u) => PrivateUser {
            id: u.id,
            username: u.username,
            profile_pic: u.profile_pic,
            email: u.email,
            created_at: u.created_at,
            admin: u.admin,
            scopes: u.scopes,
            totp_enabled: u.totp_token.is_some()
        },
        Input::PrivateUser(u) => u,
    };
    diesel::insert_into(table)
        .values(CreateApp {
            name: data.name.to_string(),
            description: data.description,
            token_lifetime: data.token_lifetime,
            owner: private_user.id,
            domains: data.domains,
            enforce_totp: data.enforce_totp
        })
        .execute(conn)?;
    let app = apps.filter(name.eq(data.name)).first::<App>(conn)?;
    Ok(app)
}

pub fn list_apps(offset: i64, conn: &PgConnection) -> Result<Vec<App>, DbError> {
    use schema::apps::dsl::{apps, updated_at};
    let res = apps
        .order_by(updated_at.desc())
        .limit(10)
        .offset(offset)
        .load::<App>(conn)?;
    Ok(res)
}

pub fn patch_app(app: PatchApp, user: PrivateUser, conn: &PgConnection) -> Result<App, DbError> {
    use schema::apps::dsl::{apps, id, owner};
    let old_app: App = apps
        .filter(id.eq(&app.id))
        .filter(owner.eq(&user.id))
        .first::<App>(conn)?;
    let new_app = App {
        id: old_app.id,
        name: match app.name {
            Some(n) => n,
            None => old_app.name,
        },
        updated_at: chrono::Utc::now().naive_utc(),
        created_at: old_app.created_at,
        owner: user.id,
        domains: match app.domains {
            Some(d) => d,
            None => old_app.domains,
        },

        description: match app.description {
            Some(n) => Some(n),
            None => old_app.description,
        },
        token_lifetime: match app.token_lifetime {
            Some(n) => n,
            None => old_app.token_lifetime,
        },
        enforce_totp: match app.enforce_totp {
            Some(v) => v,
            None => old_app.enforce_totp
        }
    };
    let target = apps.find(app.id);
    diesel::update(target).set(&new_app).execute(conn)?;
    Ok(new_app)
}

pub fn get_app_from_id(id: Uuid, conn: &PgConnection) -> Result<App, DbError> {
    use schema::apps::dsl::apps;
    Ok(apps.find(id).first::<App>(conn)?)
}

pub fn delete_app_from_id(id: Uuid, conn: &PgConnection) -> Result<usize, DbError> {
    use schema::apps::dsl::apps;
    Ok(diesel::delete(apps.find(id)).execute(conn)?)
}
