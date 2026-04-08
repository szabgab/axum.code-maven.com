# Wildcard Path Parameters

We can put a star `*` in-front of the name we use for the path.
That means it will accept slashed as well and it will capture any path.

This can be useful if you'd like to map a filesystem, or if for some reason a `/` is an acceptable
value in the specific field. For example git allows branch-names to contain a slash `/`.


## Cargo.toml

{% embed include file="src/examples/wildcard-path-parameters/Cargo.toml" %}

## The whole example

{% embed include file="src/examples/wildcard-path-parameters/src/main.rs" %}

## Tests

{% embed include file="src/examples/wildcard-path-parameters/src/tests.rs" %}

