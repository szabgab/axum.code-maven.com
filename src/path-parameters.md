# Path parameters

Show how to accept parameters in the path of the request. For example we to accept all the paths that look like this: `https://example.org/user/foobar`. This will accept any value after `/user/` except a slash.
So `/user/foo/bar` will not match. For that see the Wildcard Path parameters.


## Running

```sh
cargo run
```


## GET the main page

```
$ curl -i   http://localhost:3000/

HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 89
date: Tue, 18 Mar 2025 09:32:55 GMT


    <a href="/user/foo">/user/foo</a><br>
    <a href="/user/bar">/user/bar</a><br>
```


## Getting user Foo


```
$ curl -i   http://localhost:3000/user/Foo

HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 11
date: Tue, 18 Mar 2025 09:35:45 GMT

Hello, Foo!
```


## Try without a username

```
$ curl -i   http://localhost:3000/user/

HTTP/1.1 404 Not Found
content-length: 0
date: Tue, 18 Mar 2025 09:36:15 GMT

```

## Cargo.toml

{% embed include file="src/examples/path-parameters/Cargo.toml" %}

## The whole example

{% embed include file="src/examples/path-parameters/src/main.rs" %}

## Tests

{% embed include file="src/examples/path-parameters/src/tests.rs" %}

