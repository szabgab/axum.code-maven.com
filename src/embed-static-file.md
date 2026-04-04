# Embed Static File

Beside the HTML we might want to server some other static content. For example CSS, JavaScript, or even images. We can embed the content that we would like to serve in the Rust source code and we can create a route setting the Content-Type to the proper value.


{% embed include file="src/examples/embed-static-file/Cargo.toml" %}

## Code

{% embed include file="src/examples/embed-static-file/src/main.rs" %}

## Tests

{% embed include file="src/examples/embed-static-file/src/tests.rs" %}

