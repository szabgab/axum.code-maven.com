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
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert!(html.contains(r#"<form method="get" action="/">"#));
}

#[tokio::test]
async fn test_echo_with_data() {
    let response = create_router()
        .oneshot(
            Request::builder()
                .uri("/?a=4&b=9")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert!(html.contains(r#"<form method="get" action="/">"#));
}

#[tokio::test]
async fn test_echo_missing_value() {
    let response = create_router()
        .oneshot(Request::builder().uri("/?a=").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    //assert!(html.contains(r#"<form method="get" action="/">"#));
    //
    assert_eq!(
        html,
        "Failed to deserialize query string: a: cannot parse integer from empty string"
    );
}
