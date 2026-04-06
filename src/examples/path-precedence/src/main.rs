use axum::{Router, extract::Path, response::Html, routing::get};

async fn main_page() -> Html<&'static str> {
    Html(
        r#"
    <a href="/foo">/foo</a><br>
    <a href="/special">/special</a><br>
    "#,
    )
}

async fn name_page(Path(name): Path<String>) -> Html<String> {
    Html(format!("Hello, {}!", name))
}

async fn special_page() -> Html<String> {
    Html(String::from("Special"))
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(main_page))
        .route("/{name}", get(name_page))
        .route("/special", get(special_page))
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
