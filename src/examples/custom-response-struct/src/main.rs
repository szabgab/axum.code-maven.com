use axum::{
    Router,
    http::StatusCode,
    http::header,
    response::{Html, IntoResponse, Response},
    routing::get,
};

struct OurMessage<'a> {
    text: String,
    lang: &'a str,
}

// Tell axum how to convert `OurMessage` into a response.
impl IntoResponse for OurMessage<'_> {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
            format!("Text: {} in language: {}", self.text, self.lang),
        )
            .into_response()
    }
}

async fn main_page() -> Html<&'static str> {
    Html(
        r#"<h1>Main page</h1>
    <a href="/english">English</a><br>
    <a href="/hungarian">Hungarian</a><br>
        "#,
    )
}

async fn english_page() -> impl IntoResponse {
    OurMessage {
        text: String::from("Some text comes here"),
        lang: "en",
    }
}

async fn hungarian_page() -> impl IntoResponse {
    OurMessage {
        text: String::from("Magyarul is lehet"),
        lang: "hu",
    }
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(main_page))
        .route("/english", get(english_page))
        .route("/hungarian", get(hungarian_page))
}

#[tokio::main]
async fn main() {
    let app = create_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests;
