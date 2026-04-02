use axum::{
    Router,
    extract::Path,
    response::Html,
    routing::get,
};

async fn handle_add(Path((a, b)): Path<(u32, u32)>) -> Html<String> {
    let result = a + b;
    Html(format!("{a} + {b} = {result}"))
}
async fn handle_multiply(Path((a, b)): Path<(u32, u32)>) -> Html<String> {
    let result = a * b;
    Html(format!("{a} * {b} = {result}"))
}

async fn handle_divide(Path((a, b)): Path<(u32, u32)>) -> Html<String> {
    let result = a / b;
    Html(format!("{a} / {b} = {result}"))
}

async fn handle_subtraction(Path((a, b)): Path<(u32, u32)>) -> Html<String> {
    let result = a - b;
    Html(format!("{a} - {b} = {result}"))
}

pub fn create_router() -> Router {
    Router::new()
        .route("/add/{a}/{b}", get(handle_add))
        .route("/mul/{a}/{b}", get(handle_multiply))
        .route("/div/{a}/{b}", get(handle_divide))
        .route("/sub/{a}/{b}", get(handle_subtraction))
}


