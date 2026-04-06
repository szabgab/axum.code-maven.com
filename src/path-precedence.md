# Path precedence

If there are two routes that can match a given path (as one of them as a capturing variable)
then the more specific matches. The order of the declaration does not matter.


## Cargo.toml

{% embed include file="src/examples/path-precedence/Cargo.toml" %}

## The whole example

{% embed include file="src/examples/path-precedence/src/main.rs" %}

## Tests

{% embed include file="src/examples/path-precedence/src/tests.rs" %}

