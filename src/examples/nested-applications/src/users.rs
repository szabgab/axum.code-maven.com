use axum::{Router, extract::Path, response::Html, routing::get};

async fn user_page(Path(id): Path<String>) -> Html<String> {
    Html(format!("User id: {}", id))
}

pub fn create_router() -> Router {
    Router::new()
        .route("/{id}", get(user_page))
}


