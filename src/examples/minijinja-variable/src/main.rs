use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::Html, routing::get, Router};
use minijinja::{context, Environment};
use std::sync::Arc;

struct AppState {
    env: Environment<'static>,
}

#[tokio::main]
async fn main() {
    // init template engine and add templates
    let mut env = Environment::new();
    env.add_template("main", include_str!("../templates/main.jinja"))
        .unwrap();

    let app_state = Arc::new(AppState { env });

    let app = Router::new()
        .route("/", get(main_page))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
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

