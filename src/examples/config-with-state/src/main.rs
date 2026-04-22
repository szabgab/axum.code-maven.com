use axum::{Router, extract::State, response::Html, routing::get};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
struct Config {
    name: String,
}

async fn handle_main_page(State(config): State<Arc<Config>>) -> Html<String> {
    Html(format!("<h1>Hello, {}!</h1>", config.name))
}

fn create_router(config: Arc<Config>) -> Router {
    Router::new()
        .route("/", get(handle_main_page))
        .with_state(config)
}

#[tokio::main]
async fn main() {
    let config_content =
        std::fs::read_to_string("config.toml").expect("Failed to read config.toml");
    let config: Config = toml::from_str(&config_content).expect("Failed to parse config.toml");

    let config = Arc::new(config);

    let app = create_router(config);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests;
