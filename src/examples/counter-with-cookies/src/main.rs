use axum::{Router, response::Html, routing::get};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

const COOKIE_NAME: &str = "counter";

async fn handle_main_page(cookies: Cookies) -> Html<String> {
    let mut counter: u32 = match cookies.get(COOKIE_NAME) {
        Some(cookie) => cookie.value().parse::<u32>().unwrap(),
        None => 0,
    };
    counter += 1;

    cookies.add(Cookie::new(COOKIE_NAME, counter.to_string()));

    Html(format!(
        r#"<h1>Count {counter}</h1><a href="/delete">delete</a>"#
    ))
}

async fn delete_cookie(cookies: Cookies) -> Html<String> {
    cookies.remove(Cookie::new(COOKIE_NAME, ""));

    Html(format!(r#"<a href="/">home</a>"#))
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(handle_main_page))
        .route("/delete", get(delete_cookie))
        .layer(CookieManagerLayer::new())
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
