use axum::{body::Body, http::Request, http::StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

use super::*;

#[tokio::test]
async fn test_main_page() {
    check_contains("/", "<h1>Calculator</h1>").await;
}

#[tokio::test]
async fn test_add() {
    check_equals("/add/2/3", "2 + 3 = 5").await;
    check_equals("/add/7/8", "7 + 8 = 15").await;
}

#[tokio::test]
async fn test_subtraction() {
    check_equals("/sub/5/3", "5 - 3 = 2").await;
    check_equals("/sub/8/7", "8 - 7 = 1").await;
}


#[tokio::test]
async fn test_multiply() {
    check_equals("/mul/2/3", "2 * 3 = 6").await;
    check_equals("/mul/7/8", "7 * 8 = 56").await;
}

#[tokio::test]
async fn test_divide() {
    check_equals("/div/6/3", "6 / 3 = 2").await;
    check_equals("/div/120/10", "120 / 10 = 12").await;
}


async fn check_contains(uri: &str, expected: &str) {
    let html = get_page(uri).await;
    assert!(html.contains(expected));
}
async fn check_equals(uri: &str, expected: &str) {
    let html = get_page(uri).await;
    assert_eq!(html, expected);
}

async fn get_page(uri: &str) -> String {
    let response = create_router()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()

}
