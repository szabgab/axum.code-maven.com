use axum::{
    Router,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
};

async fn main_page() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Custom missing page")
    // We could also set the content-type to be text/html
    //(StatusCode::NOT_FOUND, Html("Custom missing page"))
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(main_page))
        .fallback(handler_404)
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
