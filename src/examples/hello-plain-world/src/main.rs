use axum::{routing::get, Router};

async fn handle_main_page() -> &'static str {
    "<h1>Hello, World!</h1>"
}

fn app() -> Router {
    Router::new().route("/", get(handle_main_page))
}


#[tokio::main]
async fn main() {
    // build our application with a route
    let app = app();

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests;
