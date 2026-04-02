use axum::{Router, response::Html, routing::get};

async fn future_events() -> Html<String> {
    Html(String::from("Future events"))
}

async fn past_events() -> Html<String> {
    Html(String::from("Past events"))
}

pub fn create_router() -> Router {
    Router::new()
        .route("/future", get(future_events))
        .route("/past", get(past_events))
}
