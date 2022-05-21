use actix_web::{web, get, HttpResponse, Error};
use crate::templates::LoginTemplate;
use askama::Template;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LoginQuery {
    pub return_to: String
}

#[get("/login")]
pub async fn login(query: web::Query<LoginQuery>) -> Result<HttpResponse, Error>{
    let return_to = query.return_to.to_string();
    let hello = LoginTemplate { redir_url: &*return_to }; // instantiate your struct
    Ok(HttpResponse::Ok().content_type("text/html").body(hello.render().unwrap()))
}
