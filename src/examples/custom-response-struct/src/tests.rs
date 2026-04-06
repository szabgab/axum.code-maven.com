use super::*;
use axum::{body::Body, http::Request, http::StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_main_page() {
    let response = create_router()
        .oneshot(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response.headers().get("content-type").unwrap();
    assert_eq!(content_type.to_str().unwrap(), "text/html; charset=utf-8");

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert!(html.contains("<h1>Main page</h1>"));
}

#[tokio::test]
async fn test_english() {
    let response = create_router()
        .oneshot(Request::get("/english").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response.headers().get("content-type").unwrap();
    assert_eq!(content_type.to_str().unwrap(), "text/html; charset=utf-8");

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert_eq!(html, "Text: Some text comes here in language: en");
}

#[tokio::test]
async fn test_hungarian() {
    let response = create_router()
        .oneshot(Request::get("/hungarian").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response.headers().get("content-type").unwrap();
    assert_eq!(content_type.to_str().unwrap(), "text/html; charset=utf-8");

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert_eq!(html, "Text: Magyarul is lehet in language: hu");
}
