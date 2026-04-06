use axum::{body::Body, http::Request, http::StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

use super::*;

#[tokio::test]
async fn test_main_page() {
    let response = create_router()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response.headers().get("content-type").unwrap();
    assert_eq!(content_type.to_str().unwrap(), "text/html; charset=utf-8");

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert!(html.contains("<h1>Static</h1>"));
}

#[tokio::test]
async fn test_static_page() {
    let response = create_router()
        .oneshot(
            Request::builder()
                .uri("/static/css/style.css")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response.headers().get("content-type").unwrap();
    assert_eq!(content_type.to_str().unwrap(), "text/css");

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let content = String::from_utf8(bytes.to_vec()).unwrap();

    let expected = std::fs::read_to_string("static/css/style.css").unwrap();

    assert_eq!(content, expected);
}
