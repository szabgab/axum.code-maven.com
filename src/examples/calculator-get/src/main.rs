use askama::Template;
use axum::{
    Router,
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};
use serde::Deserialize;

//#[derive(Debug)]
//enum Operator {
//    Add,
//    Deduct,
//    Multiply,
//    Divide,
//}
//
//impl<S> FromRequestParts<S> for Operator
//where
//    S: Send + Sync,
//{
//    type Rejection = Response;
//
//    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
//        let params: HashMap<String, String> =
//            parts.extract().await.map_err(IntoResponse::into_response)?;
//
//        let operation = params
//            .get("op")
//            .ok_or_else(|| (StatusCode::NOT_FOUND, "version param missing").into_response())?;
//
//        match operation.as_str() {
//            "add" => Ok(Operator::Add),
//            "deduct" => Ok(Operator::Deduct),
//            "mul" => Ok(Operator::Multiply),
//            "div" => Ok(Operator::Divide),
//            _ => Err((StatusCode::NOT_FOUND, "unknown operator").into_response()),
//        }
//    }
//}
//

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    a: Option<i32>,
    b: Option<i32>,
    //    op: Operator
}

#[derive(Template)]
#[template(path = "main.html")]
struct MainTemplate {
    a: i32,
    b: i32,
    result: i32,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

async fn main_page(Query(params): Query<Params>) -> impl IntoResponse {
    let a = params.a.unwrap_or_default();
    let b = params.b.unwrap_or_default();
    let result = 0;

    if params.a.is_none() || params.b.is_none() {
        let template = MainTemplate { a, b, result };
        return HtmlTemplate(template);
    }
    let result = a + b;

    let template = MainTemplate { a, b, result };
    HtmlTemplate(template)
}

fn create_router() -> Router {
    Router::new().route("/", get(main_page))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, create_router()).await.unwrap();
}

#[cfg(test)]
mod tests;
