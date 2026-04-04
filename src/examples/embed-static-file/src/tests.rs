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

    assert!(html.contains("<h1>Hello, World!</h1>"));
}

#[tokio::test]
async fn test_css() {
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

    let expected = r#"
h1 {
    color: blue;
}
    "#;

    assert_eq!(content, expected);
}
