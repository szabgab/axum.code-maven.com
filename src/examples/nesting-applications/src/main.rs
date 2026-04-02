use axum::{Router, extract::Path, response::Html, routing::get};

async fn main_page() -> Html<&'static str> {
    Html(
        r#"
    <a href="/events/future">/events/future</a><br>
    <a href="/events/past">/events/past</a><br>
    <a href="/user/42">/user/42</a><br>
    "#,
    )
}

async fn future_events() -> Html<String> {
    Html(String::from("Future events"))
}

async fn past_events() -> Html<String> {
    Html(String::from("Past events"))
}

async fn user_page(Path(id): Path<u32>) -> Html<String> {
    Html(format!("User id: {}", id))
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(main_page))
        .route("/user/{id}", get(user_page))
        .route("/events/future", get(future_events))
        .route("/events/past", get(past_events))
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
