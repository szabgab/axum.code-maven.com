use axum::{
    RequestPartsExt, Router,
    extract::{FromRequestParts, Path},
    http::{StatusCode, request::Parts},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use std::collections::HashMap;

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
        <h1>Calculator</h1>
        <a href="/v1/add/2/3">add 2 + 3</a><br>
    "#,
    )
}

//async fn handle_api(version: Version) -> Html<String> {
//    Html(format!("received request with version {version:?}"))
//}
//
//async fn handle_about(Path(group): Path<String>) -> Html<String> {
//    Html(format!("<h1>About {group}</h1>"))
//}
//
async fn handle_add(Path((a, b)): Path<(u32, u32)>) -> Html<String> {
    let result = a + b;
    Html(format!("{a} + {b} = {result}"))
}
async fn handle_multiply(Path((a, b)): Path<(u32, u32)>) -> Html<String> {
    let result = a * b;
    Html(format!("{a} * {b} = {result}"))
}

async fn handle_divide(Path((a, b)): Path<(u32, u32)>) -> Html<String> {
    let result = a / b;
    Html(format!("{a} / {b} = {result}"))
}

async fn handle_subtraction(Path((a, b)): Path<(u32, u32)>) -> Html<String> {
    let result = a - b;
    Html(format!("{a} - {b} = {result}"))
}


fn create_router() -> Router {
    let v1 = create_v1_router();
    Router::new()
        .route("/", get(main_page))
        .nest("/v1", v1)
}

fn create_v1_router() -> Router {
    Router::new()
        .route("/add/{a}/{b}", get(handle_add))
        .route("/mul/{a}/{b}", get(handle_multiply))
        .route("/div/{a}/{b}", get(handle_divide))
        .route("/sub/{a}/{b}", get(handle_subtraction))
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
