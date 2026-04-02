use axum::{body::Body, http::Request, http::StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

use super::*;

#[tokio::test]
async fn test_main_page() {
    check_page("/", "<h1>Meetup</h1>").await;
}

#[tokio::test]
async fn test_events_page() {
    check_page("/code-mavens/", "<h1>About code-mavens</h1>").await;
    check_page("/python-mavens/", "<h1>About python-mavens</h1>").await;
    check_page("/rust-mavens/", "<h1>About rust-mavens</h1>").await;
}

#[tokio::test]
async fn test_area_page() {
    check_page("/code-mavens/events", "<h1>area of code-mavens</h1>").await;
}


// #[tokio::test]
// async fn test_events_page() {
//     check_page("/code-mavens/events/", "<h1>code-mavens events</h1>").await;
// }



async fn check_page(uri: &str, expected: &str) {
    let response = create_router()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert!(html.contains(expected));
}
