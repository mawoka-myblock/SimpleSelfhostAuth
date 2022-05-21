use askama::Template; // bring trait in scope

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate<'a> {
    pub redir_url: &'a str,
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate<'a> {
    pub error_code: &'a str,
    pub error_title: &'a str,
    pub error_message: &'a str
}