use axum::{
    Router,
    response::{Html, Redirect},
    routing::get,
};

async fn main_page() -> Html<&'static str> {
    Html(
        r#"
        <h1>Redirect</h1>
        <a href="/internal-redirect">Internal Redirect</a><br>
        <a href="/external-redirect">External Redirect</a><br>
        <a href="/target-page">Target page</a>
    "#,
    )
}

async fn internal_redirect() -> Redirect {
    Redirect::temporary("/target-page")
}

async fn external_redirect() -> Redirect {
    Redirect::permanent("https://rust.code-maven.com/")
}


async fn target_page() -> Html<&'static str> {
    Html("Arrived")
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(main_page))
        .route("/internal-redirect", get(internal_redirect))
        .route("/external-redirect", get(external_redirect))
        .route("/target-page", get(target_page))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, create_router()).await.unwrap();
}

#[cfg(test)]
mod tests;
