# Hello HTML World

In the previous version of the standard "Hello World" application we sent some HTML, but the content type was set by axum to be `text/plain` and thus the browser showed the HTML tags in their natural beauty without renedring them.

Let's improve that.

You can reuse the same crate or create a new one. It is up to you.

This is how our `Cargo.toml` file looks like. I think besides the name, it is the same as in the previous example.

{% embed include file="src/examples/hello-html-world/Cargo.toml" %}

## The code

Let's only focus on the differences from the previous example.

We are also going to use the [Html struct](https://docs.rs/axum/0.8.8/axum/response/struct.Html.html)  from `axum::response` so we import it:

```rust
use axum::{response::Html, routing::get, Router};
```


In the function that handles the request we wrap the result in the `Html` struct. We need to do it both in the signature of the function and in the actual return value.

```rust
async fn handle_main_page() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
```

All the rest is the same.


To run the application type in the following command:

```
$ cargo run
```

This will compile the code, run the server and print the following to the terminal:

```
listening on http://127.0.0.1:3000
```

You can then open your browser and visit that address.

You should see **Hello World!** in big letters.


## Handling other pages

If you try to visit some other page on your server e.g. `http://127.0.0.1:3000/hi` you will get a blank page.
We'll see what happens there and how to make axum to display a [custom 404 error page](./global-404-handler.md).


## Checking with curl

The [curl](https://curl.se/) command allows you to access web sites from the command line. It is a very handy tool.

Let's see how can we use it with our web site.

Accessing the main page:

```
$ curl http://localhost:3000/
<h1>Hello, World!</h1>
```

We can also observe what happens if we try to access a page that does not exist. It seems that nothing happens which is rather inconvenient. This is the blank page we saw earlier.

```
$ curl http://localhost:3000/hi
```

Using the `-i` flag we can ask `curl` to also display the HTTP header the server sent us.
Using the upper-case `-I` flag `curl` would print only the header that was sent by the server.
It might be more convenient.

Accessing the main page we get the following:

```
$ curl -I http://localhost:3000/
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 22
date: Fri, 14 Mar 2025 08:27:44 GMT

<h1>Hello, World!</h1>
```

The first line includes the status code. This time it is `200 OK` success status.


Accessing a page that does not exists we get a `404 Not Found` error status.

```
$ curl -I http://localhost:3000/hi
HTTP/1.1 404 Not Found
date: Fri, 14 Mar 2025 08:27:41 GMT
```

## Shutting down our local server

Once you are done with this experiment you will want to shut down this local web server. Return to the terminal where you ran it and press `Ctrl-C`.

## Editing this example

Feel free to edit this example and see what happens. However, remember that after each change you'll need to stop the server and start it again.  This is rather inconvenient. Later we'll see how to make Rust automatically recompile and restart the server every time to make some changes.

## Improving the 404 page

You might dislike the fact visiting a non-existent path returns a blank page.

Check out the example showing the [404 handler](./global-404-handler.md).

## The full example

{% embed include file="src/examples/hello-html-world/src/main.rs" %}

## tests.rs

{% embed include file="src/examples/hello-html-world/src/tests.rs" %}


