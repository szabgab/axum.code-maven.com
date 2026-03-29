use askama::Template;
use axum::{
    Router,
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    text: String,
}

#[derive(Template)]
#[template(path = "main.html")]
struct MainTemplate {}

#[derive(Template)]
#[template(path = "echo.html")]
struct EchoTemplate {
    text: String,
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

async fn main_page() -> impl IntoResponse {
    let template = MainTemplate {};
    HtmlTemplate(template)
}

async fn echo(Query(params): Query<Params>) -> impl IntoResponse {
    let template = EchoTemplate { text: params.text };
    HtmlTemplate(template)
}

fn create_route() -> Router {
    Router::new()
        .route("/", get(main_page))
        .route("/echo", get(echo))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, create_route()).await.unwrap();
}

#[cfg(test)]
mod tests;
