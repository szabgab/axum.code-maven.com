use axum::{
    RequestPartsExt, Router,
    extract::{FromRequestParts, Path},
    http::{StatusCode, request::Parts},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use std::collections::HashMap;

async fn main_page() -> Html<&'static str> {
    Html(
        r#"
        <a href="/api/v1/status">v1</a><br>
        <a href="/api/v2/status">v2</a><br>
        <a href="/api/v3/status">v3</a><br>
        <a href="/api/v4/status">v4</a><br>
    "#,
    )
}

#[derive(Debug)]
enum Version {
    V1,
    V2,
    V3,
}

impl<S> FromRequestParts<S> for Version
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let params: Path<HashMap<String, String>> =
            parts.extract().await.map_err(IntoResponse::into_response)?;

        let version = params
            .get("version")
            .ok_or_else(|| (StatusCode::NOT_FOUND, "version param missing").into_response())?;

        match version.as_str() {
            "v1" => Ok(Version::V1),
            "v2" => Ok(Version::V2),
            "v3" => Ok(Version::V3),
            _ => Err((StatusCode::NOT_FOUND, "unknown version").into_response()),
        }
    }
}

async fn handle_api(version: Version) -> Html<String> {
    Html(format!("received request with version {version:?}"))
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(main_page))
        .route("/api/{version}/status", get(handle_api))
}

#[tokio::main]
async fn main() {
    let app = create_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests;
