# Custom 404 page

By default axum will return an empty page when the user accesses a path that is not handled by any of the routes.

We can add a special handler to be called by axum if no route was match. That function can create any response.

{% embed include file="src/examples/custom-404-page/Cargo.toml" %}

{% embed include file="src/examples/custom-404-page/src/main.rs" %}

{% embed include file="src/examples/custom-404-page/src/tests.rs" %}

