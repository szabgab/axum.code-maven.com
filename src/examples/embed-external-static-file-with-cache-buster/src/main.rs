use xxhash_rust::const_xxh3;
use axum::{Router, response::{Html, IntoResponse}, routing::get,
http::header::{self, HeaderMap}};

const STYLE_CSS: &[u8] = include_bytes!("static/style.css");
const STYLE_CSS_HASH: &str = const_hex::Buffer::<16, false>::new()
    .const_format(&const_xxh3::xxh3_128(STYLE_CSS).to_be_bytes())
    .as_str();


async fn handle_main_page() -> Html<String> {
    Html(format!(r#"
<!DOCTYPE html>
<html>
  <head>
    <link rel="stylesheet" href="/static/css/{STYLE_CSS_HASH}-style.css">
    <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
    <title>Embedded static file</title>
  </head>
  <body>
    <h1>Hello, World!</h1>
  </body>
</html>
"#))
}

async fn send_style_css() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
    (headers, STYLE_CSS)
}

fn create_router() -> Router {
    let css_path = format!("/static/css/{}-style.css", STYLE_CSS_HASH);
    Router::new().route("/", get(handle_main_page))
        .route(&css_path, get(send_style_css))
}

#[tokio::main]
async fn main() {
    let app = create_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests;
