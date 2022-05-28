use crate::templates::LoginTemplate;
use actix_web::{get, web, Error, HttpResponse, Responder};
use askama::Template;
use mime_guess::from_path;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/admin-dashboard/dist/"]
struct Asset;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/templates/"]
#[include = "*.css"]
struct TailwindCSS;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}
fn handle_css(path: &str) -> Option<String> {
    match TailwindCSS::get(path) {
        Some(t) => Some(std::str::from_utf8(t.data.as_ref()).unwrap().to_string()),
        None => None,
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginQuery {
    pub return_to: String,
}

#[get("/login")]
pub async fn login(query: web::Query<LoginQuery>) -> Result<HttpResponse, Error> {
    let return_to = query.return_to.to_string();
    let hello = LoginTemplate {
        redir_url: &*return_to,
        tailwind: &handle_css("tailwind.css").unwrap(),
    }; // instantiate your struct
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(hello.render().unwrap()))
}

#[get("/")]
pub async fn index() -> impl Responder {
    handle_embedded_file("index.html")
}

#[get("/{_:.*}")]
pub async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(path.as_str())
}

#[get("/css/tailwind.css")]
pub async fn css() -> impl Responder {
    match handle_css("tailwind.css") {
        Some(content) => HttpResponse::Ok().content_type("text/css").body(content),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
    // HttpResponse::Ok().body("HALLO!!!")
}
