use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    tracing::warn!("listening on {}", listener.local_addr().unwrap());
    tracing::error!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, create_router()).await.unwrap();
}

fn create_router() -> Router {
    Router::new().route("/", get(handler))
}

async fn handler() -> Html<&'static str> {
    tracing::debug!("in handler");
    tracing::info!("in handler");
    Html("<h1>Hello, World!</h1>")
}
