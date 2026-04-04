# Set Content-type

axum provide some tools to set the response Content-type to some of the common values,
and provides us ways to set the Content-type to any arbitrary string.

A few of the common Content-type values

* `text/plain`
* `text/html` [Html](https://docs.rs/axum/0.8.8/axum/response/struct.Html.html)
* `application/json` [Json](https://docs.rs/axum/0.8.8/axum/struct.Json.html)
* `text/css`
* `application/javascript`

See [axum responses](https://docs.rs/axum/0.8.8/axum/response/index.html).
The [IntoResponse](https://docs.rs/axum/0.8.8/axum/response/trait.IntoResponse.html) trait.



{% embed include file="src/examples/set-content-type/Cargo.toml" %}

## Code

{% embed include file="src/examples/set-content-type/src/main.rs" %}


## Tests

{% embed include file="src/examples/set-content-type/src/tests.rs" %}

