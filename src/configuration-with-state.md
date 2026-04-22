# Configuration with state

If we have some static data that we would like to make available in (some of) the routes, the best thing might be to
load it up-front and then pass it on as `State`.

We can do this by adding the object to the application using the `with_state` and then in the routes where would like to access this value
we put it in the list of parameters of the function handling the route.

We use [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html) as it is a thread-safe reference-counting pointer.
So the data will not be copied to each one of the threads handling the requests.

{% embed include file="src/examples/config-with-state/Cargo.toml" %}

{% embed include file="src/examples/config-with-state/config.toml" %}

{% embed include file="src/examples/config-with-state/src/main.rs" %}

{% embed include file="src/examples/config-with-state/src/tests.rs" %}

