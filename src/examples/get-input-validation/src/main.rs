use axum::{
    extract::{rejection::FormRejection, Form, FromRequest, Request},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use serde::{de::DeserializeOwned, Deserialize};
use thiserror::Error;
use tokio::net::TcpListener;
use validator::Validate;

fn create_router() -> Router {
    Router::new()
        .route("/", get(handle_main_page))
        .route("/echo", get(echo))
}

#[derive(Debug, Deserialize, Validate)]
pub struct TextInput {
    #[validate(length(min = 6, message = "Must be at least 6 characters"))]
    pub text: String,
}

async fn handle_main_page() -> Html<&'static str> {
    Html(
        r#"<h1>Echo</h1>
    <form method="GET" action="/echo">
    <input name="text">
    <input type="submit" value="Echo">
    </form>
"#,
    )
}

async fn echo(ValidatedForm(input): ValidatedForm<TextInput>) -> Html<String> {
    Html(format!("<h1>You typed in, <b>{}</b>!</h1>", input.text))
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}

#[tokio::main]
async fn main() {
    let app = create_router();

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests;
