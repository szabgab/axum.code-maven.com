use axum::{Router, response::Html, routing::get};

mod events;
mod users;

async fn main_page() -> Html<&'static str> {
    Html(
        r#"
    <a href="/events/future">/events/future</a><br>
    <a href="/events/past">/events/past</a><br>
    <a href="/user/42">/user/42</a><br>
    "#,
    )
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(main_page))
        .nest("/user", users::create_router())
        .nest("/events", events::create_router())
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
