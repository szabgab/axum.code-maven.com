# Special handling for HEAD requests

For ever GET route axum automatically installs the same HEAD route.

However we can create special handling for HEAD requests.

It rarely needed.

{% embed include file="src/examples/head-request/Cargo.toml" %}

{% embed include file="src/examples/head-request/src/main.rs" %}

{% embed include file="src/examples/head-request/src/tests.rs" %}
