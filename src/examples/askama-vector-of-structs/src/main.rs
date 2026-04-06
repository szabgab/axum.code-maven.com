use askama::Template;
use axum::{
    Router,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};

struct Planet {
    name: String,
    distance: f64,
    mass: f64,
}

#[derive(Template)]
#[template(path = "main.html")]
struct MainTemplate {
    planets: Vec<Planet>,
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

async fn main_page() -> impl IntoResponse {
    let planets = vec![
        Planet {
            name: String::from("Mercury"),
            distance: 0.4,
            mass: 0.055,
        },
        Planet {
            name: String::from("Venus"),
            distance: 0.7,
            mass: 0.815,
        },
        Planet {
            name: String::from("Earth"),
            distance: 1.0,
            mass: 1.0,
        }
    ];

    let template = MainTemplate { planets: planets };
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
