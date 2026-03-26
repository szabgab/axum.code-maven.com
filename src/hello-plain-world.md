## Hello plain world!

In the very first example with axum we try to show **Hello World!**. In bold.

The application depends on `axum` and `tokio`. We also have tests. They have some other dependencies. See the `Cargo.toml` file for the list of dependencies.

You can either edit the Cargo.toml file manually to add the dependencies or you can run the following commands:

Add axum as a dependency: Mind you our examples use version 0.8.x and acording to the [source of axum](https://github.com/tokio-rs/axum) the 0.9.x versions will have some breaking changes. I am planning to update the examples once 0.9.x is released, bu you have to make sure you use the correct version of axum.

```
$ cargo add axum
```

We also need to add `tokio` with the `full` feature:

```
$ cargo add tokio -F full
```

The dependencies needed for testing:

```
$ cargo add --dev headers
$ cargo add --dev http-body-util
$ cargo add --dev tower -F util
```

## Cargo.toml

{% embed include file="src/examples/hello-plain-world/Cargo.toml" %}

## The code

At the bottom of this page you can find the full content of the `main.rs` and `tests.rs` files, but let's go over them step-by-step.

We'll use the `get` and `Router` objects of axum so in order to be able to write the short name we need to import them.

```rust
use axum::{routing::get, Router};
```

We create a function that will handle the request. axum is asynchronous, so we prefix our function definition with `async`.
The name of the function is arbitrary. Later we'll have a separate entry that maps the URL path to this function.
For now, for this simple case our function returns a static string which is just an HTML snippet.

```rust
async fn handle_main_page() -> &'static str {
    "<h1>Hello, World!</h1>"
}
```

The next thing is mapping the URL path(es) to the appropriate function(s). Currently we have one path mapped to one function, but of course in a bigger application we'll have many pathes mapped to many functions. The idea of this mapping is usually called "routing" in the web development world. So we create a `Router` object and map the `/` path to the `handle_main_page` function for http GET requests.

```rust
fn create_router() -> Router {
    Router::new().route("/", get(handle_main_page))
}
```

Finally let's put these to gether. We have an `async` main function that will be the entry point for tokio, the async system axum built on top of.

* We create the application.
* We setup a `TcpListener` on `127.0.0.1` making it accessible only from our local computer. We use port 3000.
* The we call `axum::serve` that will enter a loop waiting for connections.

```rust
#[tokio::main]
async fn main() {
    // build our application with a route
    let app = create_router();

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
```

That's it for the application.

## Run with

```
cargo run
```

then visit `http://localhost:3000/`

You will see `<h1>Hello, World!</h1>`, yes including the HTML tags. That happens as the Content-type of the response is `text/plain` instead of `text/html`.

We can see this by using `curl` in another terminal with the `-i` flag and later in the tests.

```
$ curl -i http://localhost:3000
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 22
date: Tue, 15 Apr 2025 09:48:31 GMT

<h1>Hello, World!</h1>
```

In the next example we'll see how to make axum set the content type to `text/html` to convince the browser to interpret the HTML.


## Testing

Before we go on to the next example, let's see how can we write a test for this one.

We can put the tests in the `main.rs`, but it is probably better to put them in a separate file called `tests.rs`.
Still we need to make our code aware of the tests by adding the following lines at the end of the `main.rs` file.
It will compile the tests when we try to run them.

```rust
#[cfg(test)]
mod tests;
```


In the `tests.rs` first we import a bunch of external dependencies:

```rust
use axum::{body::Body, http::Request, http::StatusCode};
use headers::ContentType;
use http_body_util::BodyExt;
use tower::ServiceExt;
```

Then all the functions from the `main.rs` file.

```rust
use super::*;
```

In reality we only need the `create_router` function so we could have written:

```rust
use super::create_router;
```

The testing code is rather verbose, but in a nutshel, it uses the `oneshot` method to send in a single get request to the `/` path. It does not launch a web server, it does not listen on any port. It runs it internally in axum.

We can then interrogate the `response` we received.

* Check the status code.
* Check the `Content-type` in two different ways! One seems to be more simple, the other one uses `headers::ContentType`.
* Check the body of the response, the actual HTML.


## main.rs

{% embed include file="src/examples/hello-plain-world/src/main.rs" %}

## tests.rs

{% embed include file="src/examples/hello-plain-world/src/tests.rs" %}


