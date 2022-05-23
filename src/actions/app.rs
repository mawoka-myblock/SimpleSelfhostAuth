use crate::actions::DbError;
use crate::models::{App, CreateApp, PrivateUser, User};
use crate::{models, schema};
use diesel::prelude::*;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

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

pub fn create_app(
    data: models::CreateApp,
    user: Input,
    conn: &PgConnection,
) -> Result<App, DbError> {
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
        })
        .execute(conn)?;
    let app = apps.filter(name.eq(data.name)).first::<App>(conn)?;
    Ok(app)
}

pub fn list_apps(conn: &PgConnection, offset: i64) -> Result<Vec<App>, DbError> {
    use schema::apps::dsl::{apps, updated_at};
    let res = apps
        .order_by(updated_at.desc())
        .limit(10)
        .offset(offset)
        .load::<App>(conn)?;
    Ok(res)
}
