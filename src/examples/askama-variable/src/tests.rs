use super::*;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_main() {
    let response = create_router()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert!(html.contains(r#"<form method="get" action="/echo">"#));
}

#[tokio::test]
async fn test_echo_with_text() {
    let response = create_router()
        .oneshot(
            Request::builder()
                .uri("/echo?text=Hello%20World")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert_eq!(html, "You wrote: <b>Hello World</b>");
}

#[tokio::test]
async fn test_list() {
    let response = create_router()
        .oneshot(Request::builder().uri("/list").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert!(html.contains("<li>Mercury</li>"));
}

#[tokio::test]
async fn test_page() {
    let response = create_router()
        .oneshot(Request::builder().uri("/page").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert!(html.contains("<title>Title</title>"));
    assert!(html.contains("Page content"));
}

#[tokio::test]
async fn test_content() {
    let response = create_router()
        .oneshot(
            Request::builder()
                .uri("/content")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert!(html.contains("<title>Default Title</title>"));
    assert!(html.contains("Block Content"));
}

#[tokio::test]
async fn test_content_with_title() {
    let response = create_router()
        .oneshot(
            Request::builder()
                .uri("/content-with-title")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert!(html.contains("<title>Block title</title>"));
    assert!(html.contains("Content With Title"));
}
