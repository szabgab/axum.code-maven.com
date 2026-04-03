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
    assert_eq!(content_type.to_str().unwrap(), "application/json");

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let message: Message = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(message, Message { text: String::from("Hello World!") });
}

