use axum::{
    Router,
    extract::Path,
    response::Html,
    routing::get,
};

async fn handle_calc(Path((op, a, b)): Path<(String, u32, u32)>) -> Html<String> {
    match op.as_str() {
        "add" => {
            let result = a + b;
            Html(format!("{a} + {b} = {result}"))
        },
        "sub" => {
            let result = a - b;
            Html(format!("{a} - {b} = {result}"))
        },
        "mul" => {
            let result = a * b;
            Html(format!("{a} * {b} = {result}"))
        },
        "div" => {
            let result = a / b;
            Html(format!("{a} / {b} = {result}"))
        },
         _ => panic!("Unhandled operator"),
    }
}

pub fn create_router() -> Router {
    Router::new()
        .route("/{op}/{a}/{b}", get(handle_calc))
}


