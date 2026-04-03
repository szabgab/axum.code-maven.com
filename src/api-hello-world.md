# API Hello World

Return a simple JSON with a fixed string.

We create a `struct` to represent the data and use the `serde`-based `Json` serializer of axum to serialized the data and to set the content-type to `application/json`.

{% embed include file="src/examples/api-hello-world/Cargo.toml" %}

## Code

{% embed include file="src/examples/api-hello-world/src/main.rs" %}

## Tests

{% embed include file="src/examples/api-hello-world/src/tests.rs" %}

