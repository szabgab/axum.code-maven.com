# Embed external Static File with cache buster

The problem in both of the previous cases is that the browser will try to cache the content of the CSS file.
That means that even if we change the content of the CSS file inside the Rust file or even in the external file, the browser will not download the new version and we won't see the change.

We could manually clear the cache of the browser, but the end users have the same problem. If we deploy a new version of the CSS file the clients will not see the content until the cache expires.

This is a well known problem in the web development world.

We need a cache buster. One of the solutions is to change the name of the static file every time it changes.However one needs to be careful to also change the reference to the file to match the new filename.


{% embed include file="src/examples/embed-external-static-file-with-cache-buster/Cargo.toml" %}

## Code

{% embed include file="src/examples/embed-external-static-file-with-cache-buster/src/main.rs" %}

## CSS file

{% embed include file="src/examples/embed-external-static-file-with-cache-buster/src/static/style.css" %}

## Tests

{% embed include file="src/examples/embed-external-static-file-with-cache-buster/src/tests.rs" %}

