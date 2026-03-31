# Testing

Writing automated tests for your application can save you a lot of time down the road and you might even develop you application much faster if instead of checking it in a browser you write test. This is especially true if you are implementing an API which is designed to be consumed by other software anyway.

In `main.rs` we need to mention the test module:

```rust
#[cfg(test)]
mod tests;
```


