use tracing_subscriber::{
    Layer, filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt,
};

use axum::{
    Form, Router,
    response::Html,
    routing::{get, post},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    text: String,
}

async fn main_page() -> Html<&'static str> {
    tracing::info!("main page");
    Html(
        r#"
    <form method="post" action="/echo">
    <input type="text" name="text">
    <input type="submit" value="Echo">
    </form>
    "#,
    )
}

async fn echo(Form(params): Form<Params>) -> Html<String> {
    tracing::info!("params: {:?}", params);
    Html(format!(r#"You said: <b>{}</b>"#, params.text))
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(main_page))
        .route("/echo", post(echo))
}

#[tokio::main]
async fn main() {
    setup_tracing("demo");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, create_router()).await.unwrap();
}

fn setup_tracing(prefix: &str) {
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    std::fs::create_dir_all("logs").unwrap();
    let log_filename = format!("logs/{}-{}.log", prefix, date);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_writer(
                    std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(log_filename)
                        .unwrap(),
                )
                .with_filter(LevelFilter::DEBUG),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
#[cfg(test)]
mod tests;
