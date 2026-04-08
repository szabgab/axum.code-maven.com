# Overlapping pathes an path precedence

If there are two routes that match the exact same path and handle the same http method then
we'll get a run-time panic.

```rust
async fn special_page() -> Html<String> {
    Html(String::from("Special"))
}

async fn other_page() -> Html<String> {
    Html(String::from("Other"))
}

fn create_router() -> Router {
    Router::new()
        .route("/special", get(special_page))
        .route("/special", get(other_page))
}
```

This is the panic:

```
Overlapping method route. Handler for `GET /special` already exists
```

Though it would be nicer if this was recognized during the compilation already, but
the panic happens when we try to create the `Router` before the server starts, so
the simplest test that calls the `create_router` function will catch this problem.

---

It is fine to map the same path for different HTTP methods (eg. one of them for GET and the other one for POST).

---

It can still happen that there are two routes that match a given path if one of them is a capturing variable. In that case the more specific matches. The order of the declaration does not matter.


## Cargo.toml

{% embed include file="src/examples/path-precedence/Cargo.toml" %}

## The whole example

{% embed include file="src/examples/path-precedence/src/main.rs" %}

## Tests

{% embed include file="src/examples/path-precedence/src/tests.rs" %}

