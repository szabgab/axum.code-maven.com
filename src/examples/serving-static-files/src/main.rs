use axum::{Router, response::Html, routing::get};
use tower_http::services::ServeDir;

async fn handle_main_page() -> Html<&'static str> {
    Html(r#"<h1>Static</h1>
    <a href="/static/css/style.css">style.css</a>
    "#)
}

fn create_router() -> Router {
    Router::new().route("/", get(handle_main_page))
        .nest_service("/static", ServeDir::new("static"))
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
