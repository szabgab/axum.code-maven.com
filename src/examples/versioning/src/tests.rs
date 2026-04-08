use axum::{body::Body, http::Request, http::StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

use super::*;

#[tokio::test]
async fn test_v1() {
    let response = create_router()
        .oneshot(
            Request::builder()
                .uri("/api/v1/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response.headers().get("content-type").unwrap();
    assert_eq!(content_type.to_str().unwrap(), "text/html; charset=utf-8");

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert_eq!(html, "received request with version V1");
}

#[tokio::test]
async fn test_v4() {
    let response = create_router()
        .oneshot(
            Request::builder()
                .uri("/api/v4/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let content_type = response.headers().get("content-type").unwrap();
    assert_eq!(content_type.to_str().unwrap(), "text/plain; charset=utf-8");

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert_eq!(html, "unknown version");
}

#[tokio::test]
async fn test_api() {
    let response = create_router()
        .oneshot(Request::builder().uri("/api/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // no content-type
    assert!(response.headers().get("content-type").is_none());

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert_eq!(html, "");
}
