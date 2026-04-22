use axum::{Router, response::Html, routing::get};
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    name: String,
}

async fn handle_main_page() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

fn create_router() -> Router {
    Router::new().route("/", get(handle_main_page))
}

#[tokio::main]
async fn main() {
    let config_content = std::fs::read_to_string("config.toml").expect("Failed to read config.toml");
    let config: Config = toml::from_str(&config_content).expect("Failed to parse config.toml");
    println!("Config name: {}", config.name);

    let app = create_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests;
