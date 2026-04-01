use axum::{
    RequestPartsExt, Router,
    extract::{FromRequestParts, Path},
    http::{StatusCode, request::Parts},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use std::collections::HashMap;

// About https://www.meetup.com/code-mavens/
// Events: https://www.meetup.com/code-mavens/events/
// Members: https://www.meetup.com/code-mavens/members/
// Photos: https://www.meetup.com/code-mavens/photos/
// Discussions: https://www.meetup.com/code-mavens/discussions/
//
// Calendar: https://www.meetup.com/code-mavens/events/calendar/
// Upcoming events: https://www.meetup.com/code-mavens/events/?type=upcoming   (the same as without
// this type)
// Past events: https://www.meetup.com/code-mavens/events/?type=past
// Event drafts: https://www.meetup.com/code-mavens/events/?type=draft
// this does not show any event: https://www.meetup.com/code-mavens/events/?type=qqrq
// Event: https://www.meetup.com/code-mavens/events/313944233/?eventOrigin=group_events_list

async fn main_page() -> Html<&'static str> {
    Html(
        r#"
        <h1>Meetup</h1>
        <a href="/code-mavens/">About</a><br>
        <a href="/code-mavens/events/">Events</a><br>
        <a href="/code-mavens/members/">Members</a><br>
        <a href="/code-mavens/events/1234">Event ID 1234</a><br>
    "#,
    )
}

//#[derive(Debug)]
//enum Version {
//    V1,
//    V2,
//    V3,
//}
//
//impl<S> FromRequestParts<S> for Version
//where
//    S: Send + Sync,
//{
//    type Rejection = Response;
//
//    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
//        let params: Path<HashMap<String, String>> =
//            parts.extract().await.map_err(IntoResponse::into_response)?;
//
//        let version = params
//            .get("version")
//            .ok_or_else(|| (StatusCode::NOT_FOUND, "version param missing").into_response())?;
//
//        match version.as_str() {
//            "v1" => Ok(Version::V1),
//            "v2" => Ok(Version::V2),
//            "v3" => Ok(Version::V3),
//            _ => Err((StatusCode::NOT_FOUND, "unknown version").into_response()),
//        }
//    }
//}

//async fn handle_api(version: Version) -> Html<String> {
//    Html(format!("received request with version {version:?}"))
//}
//
async fn handle_about(Path(group): Path<String>) -> Html<String> {
    Html(format!("About group '{group}'"))
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(main_page))
        .route("/{group}/", get(handle_about))
}

#[tokio::main]
async fn main() {
    let app = create_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests;
