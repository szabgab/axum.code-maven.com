


```
curl https://raw.githubusercontent.com/tokio-rs/axum/refs/heads/main/ECOSYSTEM.md -o src/axum_ecosystem.md 
```



In our application we need to map the path part of each URL the user might visit to a function to handle that request.
For this we need a function to handle the request and we need the map the path portion of the URL to the function that will handle it.
For example if we would like to handle the URL `https://example.org/hello/world` then we need to map `/hello/world` path to the appropriate function in our application. In particular the address of the main page is `https://example.org/` and thus the path is `/`.

## Testing

Writing automated tests for your application can save you a lot of time down the road and you might even develop you application much faster if instead of checking it in a browser you write test. This is especially true if you are implementing an API which is designed to be consumed by other software anyway.


In `main.rs` we need to mention the test module:

```rust
#[cfg(test)]
mod tests;
```


