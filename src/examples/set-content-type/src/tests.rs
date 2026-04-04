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

    assert!(html.contains("<h1>Set Content-Type</h1>"));
}

#[tokio::test]
async fn test_static_plain_text() {
    let response = create_router()
        .oneshot(
            Request::builder()
                .uri("/static-plain-text")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response.headers().get("content-type").unwrap();
    assert_eq!(content_type.to_str().unwrap(), "text/plain; charset=utf-8");

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert_eq!(html, "<h1>Plain text</h1>");
}

#[tokio::test]
async fn test_dynamic_plain_text() {
    let response = create_router()
        .oneshot(
            Request::builder()
                .uri("/dynamic-plain-text")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response.headers().get("content-type").unwrap();
    assert_eq!(content_type.to_str().unwrap(), "text/plain; charset=utf-8");

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert!(html.contains("<h1>Plain text. Time since epoch: "));
}

#[tokio::test]
async fn test_css() {
    let response = create_router()
        .oneshot(Request::builder().uri("/css").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response.headers().get("content-type").unwrap();
    assert_eq!(content_type.to_str().unwrap(), "text/css");

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let content = String::from_utf8(bytes.to_vec()).unwrap();

    let expected_css = r#"
h1 {
    color: blue;
}
    "#;

    assert_eq!(content, expected_css);
}

#[tokio::test]
async fn test_js() {
    let response = create_router()
        .oneshot(Request::builder().uri("/js").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response.headers().get("content-type").unwrap();
    assert_eq!(content_type.to_str().unwrap(), "application/javascript");

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let content = String::from_utf8(bytes.to_vec()).unwrap();

    let expected_js = r#"
    alert("hi");
    "#;

    assert_eq!(content, expected_js);
}

#[tokio::test]
async fn test_json() {
    let response = create_router()
        .oneshot(Request::builder().uri("/json").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response.headers().get("content-type").unwrap();
    assert_eq!(content_type.to_str().unwrap(), "application/json");

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let content: Vec<String> = serde_json::from_slice(&bytes).unwrap();

    let expected = vec![
        String::from("Mercury"),
        String::from("Venus"),
        String::from("Earth"),
    ];

    assert_eq!(content, expected);
}
