use axum::{Router, extract::Path, response::Json, routing::get};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Calc {
    a: u32,
    b: u32,
    sum: u32,
}

async fn handle_main_page(Path((a, b)): Path<(u32, u32)>) -> Json<Calc> {
    let data = Calc { a, b, sum: a + b };
    Json(data)
}

fn create_router() -> Router {
    Router::new().route("/{a}/{b}", get(handle_main_page))
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
