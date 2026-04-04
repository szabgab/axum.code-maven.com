# Embed external Static File

It is probably much better if we keep the CSS file outside of the Rust file
and embed its content during compilation using the `include_str!` macro.

There is also an `include_bytes!` macro for embeddig images and other binary files.


{% embed include file="src/examples/embed-external-static-file/Cargo.toml" %}

## Code

{% embed include file="src/examples/embed-external-static-file/src/main.rs" %}

## CSS file

{% embed include file="src/examples/embed-external-static-file/src/static/style.css" %}

## Tests

{% embed include file="src/examples/embed-external-static-file/src/tests.rs" %}

