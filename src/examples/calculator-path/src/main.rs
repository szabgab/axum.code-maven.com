use axum::{
    Router,
    response::Html,
    routing::get,
};

mod v1calc;
mod v2calc;

async fn main_page() -> Html<&'static str> {
    Html(
        r#"
        <h1>Calculator</h1>
        <a href="/v1/add/2/3">add 2 + 3</a><br>
    "#,
    )
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(main_page))
        .nest("/v1", v1calc::create_router())
        .nest("/v2", v2calc::create_router())
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

