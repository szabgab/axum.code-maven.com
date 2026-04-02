use axum::{
    RequestPartsExt, Router,
    extract::{FromRequestParts, Path},
    http::{StatusCode, request::Parts},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use std::collections::HashMap;

// Meetup: https://www.meetup.com/  redirects to Meetup Home: https://www.meetup.com/home/
// About: https://www.meetup.com/code-mavens/
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

// #[derive(Debug)]
// enum Area {
//     Events,
//     Members,
// }
// 
// impl<S> FromRequestParts<S> for Area
// where
//     S: Send + Sync,
// {
//     type Rejection = Response;
// 
//     async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
//         let params: Path<HashMap<String, String>> =
//             parts.extract().await.map_err(IntoResponse::into_response)?;
// 
//         let area = params
//             .get("area")
//             .ok_or_else(|| (StatusCode::NOT_FOUND, "area param missing").into_response())?;
// 
//         match area.as_str() {
//             "events" => Ok(Area::Events),
//             "members" => Ok(Area::Members),
//             _ => Err((StatusCode::NOT_FOUND, "unknown area").into_response()),
//         }
//     }
// }



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

//async fn handle_api(version: Version) -> Html<String> {
//    Html(format!("received request with version {version:?}"))
//}
//
async fn handle_about(Path(group): Path<String>) -> Html<String> {
    Html(format!("<h1>About {group}</h1>"))
}

async fn handle_area(Path((group, area)): Path<(String, String)>) -> Html<String> {
    Html(format!("<h1>{area} of {group}</h1>"))
}


fn create_router() -> Router {
    Router::new()
        .route("/", get(main_page))
        .route("/{group}/", get(handle_about))
        .route("/{group}/{area}", get(handle_area))
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
