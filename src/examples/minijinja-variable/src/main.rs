use axum::extract::State;
use axum::http::StatusCode;
use axum::{Router, response::Html, routing::get};
use minijinja::{Environment, context};
use std::sync::Arc;

struct AppState {
    env: Environment<'static>,
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

fn create_router() -> Router {
    let mut env = Environment::new();
    env.add_template("main", include_str!("../templates/main.html"))
        .unwrap();

    let app_state = Arc::new(AppState { env });

    Router::new()
        .route("/", get(main_page))
        .with_state(app_state)
}

async fn main_page(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("main").unwrap();

    let rendered = template
        .render(context! {
            title => "Mini Jinja",
            welcome_text => "Hello World!",
        })
        .unwrap();

    Ok(Html(rendered))
}

#[cfg(test)]
mod tests;
