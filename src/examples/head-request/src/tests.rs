use super::*;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_get_main_page() {
    let app = create_route();

    let response = app
        .oneshot(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response.headers().get("content-type").unwrap();
    assert_eq!(content_type.to_str().unwrap(), "text/html; charset=utf-8");

    assert_eq!(
        response.headers().get("x-my-header").unwrap(),
        "header of main page"
    );

    let bytes = response.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();
    assert!(html.contains("<h1>HEAD</h1>"));
}

#[tokio::test]
async fn test_head_main_page() {
    let app = create_route();

    let response = app
        .oneshot(Request::head("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response.headers().get("content-type").unwrap();
    assert_eq!(content_type.to_str().unwrap(), "text/html; charset=utf-8");

    assert_eq!(
        response.headers().get("x-my-header").unwrap(),
        "header of main page"
    );

    let bytes = response.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();
    assert_eq!(html, "");
}

#[tokio::test]
async fn test_get() {
    let app = create_route();

    let response = app
        .oneshot(Request::get("/get-head").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()["x-my-header"],
        "header for GET get_head_handler"
    );

    let bytes = response.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();
    assert_eq!(html, "The content");
}

#[tokio::test]
async fn test_head() {
    let app = create_route();

    let response = app
        .oneshot(Request::head("/get-head").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()["x-my-header"],
        "header for HEAD get_head_handler"
    );

    let body = response.collect().await.unwrap().to_bytes();
    assert!(body.is_empty());
}

#[tokio::test]
async fn test_head_just_head() {
    let app = create_route();

    let response = app
        .oneshot(Request::head("/just-head").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()["x-my-header"],
        "header from HEAD of just_head"
    );

    let body = response.collect().await.unwrap().to_bytes();
    assert!(body.is_empty());
}

#[tokio::test]
async fn test_get_just_head() {
    let app = create_route();

    let response = app
        .oneshot(Request::get("/just-head").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);

    let body = response.collect().await.unwrap().to_bytes();
    assert!(body.is_empty());
}
