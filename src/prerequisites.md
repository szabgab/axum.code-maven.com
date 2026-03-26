# Prerequisites

What do you need to know and what do you need to have on your computer to follow this book?

* You will need to have Rust, more specifically the Rust compiler and the toolchain installed on your computer.
* axum and the other dependencies will be installes using the Rust toolchain, you don't have to worry about that.
* You will have to be somewhat familiar with Rust. I'd recommend going through the [Rust book](https://rust-lang.org/learn/)


TODO: A little explanation about Rust and crates.

## HTTP request methods

Web browsers (or clients) communicate with servers using the HTTP protocol. The protocol defines a number of methods.
The most commonly used ones are called GET and POST. GET is usually used when you visit pages and POST is usually used when you submit forms. We'll see them through the examples.

See the full list of [HTTP request methods](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods)

## HTTP response status codes

When a web sever responds to a request it includes a status code. The status codes include a number and a default name.
For exampl "200 OK" means success. "404 NOT FOUND" means the requested page does not exist. "500 INTERNAL SERVER ERROR" means the server crashed. (e.g. there was an unhandled exception in the code.)

See the full list of [HTTP response status codes](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status).


