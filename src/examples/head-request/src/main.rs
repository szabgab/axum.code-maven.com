use axum::response::{Html, IntoResponse, Response};
use axum::{Router, http, routing::get, routing::head};

async fn main_page() -> Response {
    let content = String::from(
        r#"
        <h1>HEAD</h1>
    "#,
    );
    ([("x-my-header", "header of main page")], Html(content)).into_response()
}

async fn just_head() -> Response {
    ([("x-my-header", "header from HEAD of just_head")]).into_response()
}

async fn get_head_handler(method: http::Method) -> Response {
    if method == http::Method::HEAD {
        return ([("x-my-header", "header for HEAD get_head_handler")]).into_response();
    }

    println!("do some heavy computing task in GET");

    (
        [("x-my-header", "header for GET get_head_handler")],
        Html("The content"),
    )
        .into_response()
}

fn create_route() -> Router {
    Router::new()
        .route("/", get(main_page))
        .route("/just-head", head(just_head))
        .route("/get-head", get(get_head_handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, create_route()).await.unwrap();
}

#[cfg(test)]
mod tests;
