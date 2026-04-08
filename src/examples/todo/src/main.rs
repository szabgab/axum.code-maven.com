use askama::Template;
use axum::{
    Router,
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};
use serde::Deserialize;

struct Item {
    id: String,
    title: String,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "main.html")]
struct MainTemplate {
    title: String,
}

#[derive(Template)]
#[template(path = "edit.html")]
struct EditTemplate {
    title: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    text: String,
}

async fn main_page() -> impl IntoResponse {
    let template = MainTemplate {
        title: String::from("TODO"),
    };
    HtmlTemplate(template)
}

// Query(params): Query<Params>
async fn edit() -> impl IntoResponse {
    let template = EditTemplate {
        title: String::from("Add new item"),
    };
    HtmlTemplate(template)
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(main_page))
        .route("/edit", get(edit))
}
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, create_router()).await.unwrap();
}

#[cfg(test)]
mod tests;
