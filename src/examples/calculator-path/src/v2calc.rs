use axum::{
    Router, extract::Path, http::StatusCode, response::Html, response::IntoResponse, routing::get,
};

async fn handle_calc(Path((op, a, b)): Path<(String, u32, u32)>) -> impl IntoResponse {
    match op.as_str() {
        "add" => {
            let result = a + b;
            (StatusCode::OK, Html(format!("{a} + {b} = {result}")))
        }
        "sub" => {
            let result = a - b;
            (StatusCode::OK, Html(format!("{a} - {b} = {result}")))
        }
        "mul" => {
            let result = a * b;
            (StatusCode::OK, Html(format!("{a} * {b} = {result}")))
        }
        "div" => {
            let result = a / b;
            (StatusCode::OK, Html(format!("{a} / {b} = {result}")))
        }
        _ => (
            StatusCode::NOT_FOUND,
            Html(format!("Unhandled operator: {op}")),
        ),
    }
}

pub fn create_router() -> Router {
    Router::new().route("/{op}/{a}/{b}", get(handle_calc))
}
