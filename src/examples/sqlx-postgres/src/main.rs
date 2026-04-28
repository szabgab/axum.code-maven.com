use axum::{
    Router,
    extract::{FromRef, FromRequestParts, State},
    http::{StatusCode, request::Parts},
    response::Html,
    routing::get,
};
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::net::TcpListener;

use std::time::Duration;

async fn main_page() -> Html<&'static str> {
    Html(
        r#"
        <h2>Connection Pool Extractor</h2>
        <a href="/v1">Select text</a><br>
        <hr>
        <a href="/v2">Version 2</a><br>
    "#,
    )
}

// we can extract the connection pool with `State`
async fn using_connection_pool_extractor(
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("SELECT 'Hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

async fn using_connection_extractor(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("SELECT version();")
        .fetch_one(&mut *conn)
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
        .route("/v1", get(using_connection_pool_extractor))
        .route("/v2", get(using_connection_extractor))
        .with_state(pool)
}

#[tokio::main]
async fn main() {
    let pool = set_up_connection_pool().await;
    let app = create_router(pool);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
