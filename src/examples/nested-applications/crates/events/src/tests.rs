use axum::{body::Body, http::Request, http::StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

use super::*;

//#[tokio::test]
//async fn test_main_page() {
//    let html = get_html("/").await;
//    assert!(html.contains(r#"<a href="/user/42">/user/42</a>"#));
//}
//

#[tokio::test]
async fn test_future_events() {
    let html = get_html("/future").await;
    assert_eq!(html, "Future events");
}

#[tokio::test]
async fn test_past_events() {
    let html = get_html("/past").await;
    assert_eq!(html, "Past events");
}

async fn get_html(uri: &str) -> String {
    let response = create_router()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}
