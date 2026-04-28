use serde::Deserialize;

use axum::{
    Form,
    Router,
    extract::{FromRef, FromRequestParts, State},
    http::{StatusCode, request::Parts},
    response::Html,
    routing::{get, post},
};
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::net::TcpListener;

use std::time::Duration;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    counter: String,
}

async fn main_page(State(_pool): State<PgPool>) -> Html<String> {
    let mut html = String::from("<h1>Counter example</h1>");
    html += r#"<form method="POST" action="/count">
    <input name="counter">
    <input type="submit" value="Start">
    </form>
    "#;

    Html(html)
}

async fn count(
    State(pool): State<PgPool>,
    Form(params): Form<Params>,
) -> Result<String, (StatusCode, String)> {
    println!("{}", params.counter);
    let name  = params.counter;

    sqlx::query!("INSERT INTO counter ( name) VALUES ( $1 )", name)
                .execute(pool)
                .await?;        

    match sqlx::query_scalar("SELECT value FROM counter WHERE name = '%{name}%'")
        .fetch_one(&pool)
        .await {
        Ok(value) => {
        }
        Err(sqlx::Error::RowNotFound) => 0,
        }

    sqlx::query_scalar("SELECT * FROM counter WHERE name = '%{name}%'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)

}


/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

async fn set_up_connection_pool() -> PgPool {
    let username = "myuser";
    let password = "secret";
    let db_name = "mydb";
    let hostname = "localhost";
    let connector = format!(
        "postgres://{}:{}@{}/{}",
        username, password, hostname, db_name
    );
    let db_connection_str = std::env::var("DATABASE_URL").unwrap_or_else(|_| connector);

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database")
}

fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(main_page))
        .route("/count", post(count))
        .with_state(pool)
}

async fn create_database(pool: PgPool) {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS counter (
            name TEXT PRIMARY KEY,
            value INTEGER NOT NULL DEFAULT 0
        );
    "#,
    )
    .execute(&pool)
    .await
    .expect("failed to create database");
}

#[tokio::main]
async fn main() {
    let pool = set_up_connection_pool().await;
    create_database(pool.clone()).await;
    let app = create_router(pool);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
