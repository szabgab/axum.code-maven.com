# Logging - Tracing

axum uses the [tracing](https://crates.io/crates/tracing) and [tracing-subscriber](https://crates.io/crates/tracing-subscriber) crates for logging so we need to include both.

{% embed include file="src/examples/loggin/Cargo.toml" %}

{% embed include file="src/examples/logging/src/main.rs" %}

When we start the application with `cargo run` we'll see line like this on the terminal:

```
2026-03-29T15:49:37.789551Z DEBUG axum_logging: listening on 127.0.0.1:3000
2026-03-29T15:49:37.789601Z  INFO axum_logging: listening on 127.0.0.1:3000
2026-03-29T15:49:37.789613Z  WARN axum_logging: listening on 127.0.0.1:3000
2026-03-29T15:49:37.789623Z ERROR axum_logging: listening on 127.0.0.1:3000
```

When we access the main page with a browser we'll see two more lines:

```
2026-03-29T15:49:39.380861Z TRACE axum::serve: connection 127.0.0.1:44280 accepted
2026-03-29T15:49:39.381339Z DEBUG axum_logging: in handler
2026-03-29T15:49:39.381372Z  INFO axum_logging: in handler
```

