use axum::{Router, response::{Html, IntoResponse}, routing::get,
http::header::{self, HeaderMap}};

async fn handle_main_page() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html>
  <head>
    <link rel="stylesheet" href="/static/css/style.css">
    <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
    <title>Embedded static file</title>
  </head>
  <body>
    <h1>Hello, World!</h1>
  </body>
</html>
"#)
}

async fn send_style_css() -> impl IntoResponse {
    let css = include_str!("static/style.css");
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
    (headers, css)
}

fn create_router() -> Router {
    Router::new().route("/", get(handle_main_page))
        .route("/static/css/style.css", get(send_style_css))
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
