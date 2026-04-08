use axum::{
    Json, Router,
    http::{
        StatusCode,
        header::{self, HeaderMap},
    },
    response::{Html, IntoResponse},
    routing::get,
};
use std::time::{SystemTime, UNIX_EPOCH};

async fn handle_main_page() -> Html<&'static str> {
    Html(
        r#"
    <h1>Set Content-Type</h1>
    Main page is <b>static text/html</b><br>
    <a href="/static-plain-text">static <b>text/plain<b/></a><br>
    <a href="/dynamic-plain-text">dynamic <b>text/plain</b></a><br>
    <a href="/dynamic-html">dynamic <b>text/html</b></a><br>
    <a href="/js">application/javascript</a><br>
    <a href="/css">css</a><br>
"#,
    )
}

async fn handle_static_plain_text() -> &'static str {
    "<h1>static text/plain</h1>"
}

async fn handle_dynamic_plain_text() -> String {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    format!(
        "<h1>dynamic text/plain</h1> Time since epoch: {:?}</h1>",
        since_the_epoch
    )
}

async fn handle_dynamic_html() -> Html<String> {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    Html(format!(
        "<h1>dynamic text/html</h1> Time since epoch: {:?}</h1>",
        since_the_epoch
    ))
}

async fn handle_json() -> Json<Vec<String>> {
    // application/json
    let planets = vec![
        String::from("Mercury"),
        String::from("Venus"),
        String::from("Earth"),
    ];
    Json(planets)
}

async fn send_style_css() -> impl IntoResponse {
    let css = r#"
h1 {
    color: blue;
}
    "#;
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
    (headers, css)
}

async fn send_javascript() -> impl IntoResponse {
    let js = r#"
    alert("hi");
    "#;

    (
        StatusCode::OK, // This status code is optional and OK is the default
        [(header::CONTENT_TYPE, "application/javascript")],
        js,
    )
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(handle_main_page))
        .route("/static-plain-text", get(handle_static_plain_text))
        .route("/dynamic-plain-text", get(handle_dynamic_plain_text))
        .route("/dynamic-html", get(handle_dynamic_html))
        .route("/css", get(send_style_css))
        .route("/js", get(send_javascript))
        .route("/json", get(handle_json))
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
