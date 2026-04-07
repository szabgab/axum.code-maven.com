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

    let headers = response.headers().clone();
    let content_type = headers.get("content-type").unwrap();
    assert_eq!(content_type.to_str().unwrap(), "text/html; charset=utf-8");

    let cookies = headers.get_all("set-cookie");
    let counter_cookie = cookies
        .iter()
        .find(|c| c.to_str().unwrap().contains("counter="))
        .expect("Counter cookie should be set");
    assert_eq!(counter_cookie, "counter=1");

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert_eq!(html, "<h1>Count 1</h1><a href=\"/delete\">delete</a>");

    // new request, now with cookie
    let response = create_router()
        .oneshot(
            Request::builder()
                .uri("/")
                .header("Cookie", counter_cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response.headers().get("content-type").unwrap();
    assert_eq!(content_type.to_str().unwrap(), "text/html; charset=utf-8");

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert_eq!(html, "<h1>Count 2</h1><a href=\"/delete\">delete</a>");
}
