use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

use super::*;

async fn get_html(response: Response<Body>) -> String {
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}

#[tokio::test]
async fn test_main_page() {
    let response = create_router()
        .oneshot(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let html = get_html(response).await;
    assert!(html.contains("<h1>Echo</h1>"));
    assert!(html.contains(r#"<form method="GET" action="/echo">"#));
}

#[tokio::test]
async fn test_no_param() {
    let response = create_router()
        .oneshot(Request::get("/echo").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let html = get_html(response).await;
    assert_eq!(html, "Failed to deserialize form: missing field `text`");
}

#[tokio::test]
async fn test_with_param_without_value() {
    let response = create_router()
        .oneshot(Request::get("/echo?text=").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let html = get_html(response).await;
    assert_eq!(
        html,
        "Input validation error: [text: Must be at least 6 characters]"
    );
}

#[tokio::test]
async fn test_with_param_with_short_value() {
    let response = create_router()
        .oneshot(
            Request::get("/echo?text=short")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let html = get_html(response).await;
    assert_eq!(
        html,
        "Input validation error: [text: Must be at least 6 characters]"
    );
}

#[tokio::test]
async fn test_with_param_and_value() {
    let response = create_router()
        .oneshot(
            Request::get("/echo?text=Long+text")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let html = get_html(response).await;
    assert_eq!(html, "<h1>You typed in, <b>Long text</b>!</h1>");
}
