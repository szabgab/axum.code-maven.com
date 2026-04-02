# Nesteting applications

In this examples we have 3 different routes, `/events/future`, `/events/past` and `/user/ID` where ID can be any number. Effectively there are 2 applications the `/events/` application and the `/user/` application.

Here we implemented separate functions for each one of them, all in the same crate.

{% embed include file="src/examples/nesting-applications/Cargo.toml" %}

{% embed include file="src/examples/nesting-applications/src/main.rs" %}

{% embed include file="src/examples/nesting-applications/src/tests.rs" %}
