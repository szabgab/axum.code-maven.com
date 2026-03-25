# versioning - path parameter with fixed values

Sometimes the path-parameter is from a fixed set of values. For example if we are build an API and the first part of the path is the
version number of the API then we might accept the strings `v1`, `v2`, `v3`, but no other value.

In this example we see exactly that.

{% embed include file="src/examples/versioning/Cargo.toml" %}


{% embed include file="src/examples/versioning/src/main.rs" %}

TODO: Separate the tests

