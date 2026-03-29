use super::*;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_main() {
    let response = create_route()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert!(html.contains("<h1>Redirect</h1>"));
}

#[tokio::test]
async fn test_target_page() {
    let response = create_route()
        .oneshot(
            Request::builder()
                .uri("/target-page")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert_eq!(html, "Arrived");
}

#[tokio::test]
async fn test_internal_redirect() {
    let response = create_route()
        .oneshot(
            Request::builder()
                .uri("/internal-redirect")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    let location = response.headers().get("Location").unwrap();
    assert_eq!(location, "/target-page");
}

#[tokio::test]
async fn test_external_redirect() {
    let response = create_route()
        .oneshot(
            Request::builder()
                .uri("/external-redirect")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::PERMANENT_REDIRECT);
    let location = response.headers().get("Location").unwrap();
    assert_eq!(location, "https://rust.code-maven.com/");
}
