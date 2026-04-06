use axum::{Router, extract::Path, response::Html, routing::get};

async fn main_page() -> Html<&'static str> {
    Html(
        r#"
    <a href="/root/">/root/</a><br>
    <a href="/root/etc">/root/etc</a><br>
    <a href="/root/var/log/nginx.log">/root/var/log/nginx.log</a><br>
    "#,
    )
}

async fn access_path(Path(name): Path<String>) -> Html<String> {
    Html(format!("Path: <b>{}</b>", name))
}

async fn handle_root() -> Html<String> {
    Html(String::from("Root"))
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(main_page))
        .route("/root/{*name}", get(access_path))
        .route("/root/", get(handle_root))
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
