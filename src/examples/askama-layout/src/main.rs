use askama::Template;
use axum::{
    Router,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};

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
struct MainTemplate {}

#[derive(Template)]
#[template(path = "content.html")]
struct ContentTemplate {}

#[derive(Template)]
#[template(path = "content_with_title.html")]
struct ContentWithTitleTemplate {}


async fn main_page() -> impl IntoResponse {
    let template = MainTemplate {};
    HtmlTemplate(template)
}

async fn content_default_title() -> impl IntoResponse {
    let template = ContentTemplate {};
    HtmlTemplate(template)
}

async fn content_with_title() -> impl IntoResponse {
    let template = ContentWithTitleTemplate {};
    HtmlTemplate(template)
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(main_page))
        .route("/content", get(content_default_title))
        .route("/content-with-title", get(content_with_title))
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
