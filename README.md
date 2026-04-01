

```
curl https://raw.githubusercontent.com/tokio-rs/axum/refs/heads/main/ECOSYSTEM.md -o src/axum_ecosystem.md 
```

In our application we need to map the path part of each URL the user might visit to a function to handle that request.
For this we need a function to handle the request and we need the map the path portion of the URL to the function that will handle it.
For example if we would like to handle the URL `https://example.org/hello/world` then we need to map `/hello/world` path to the appropriate function in our application. In particular the address of the main page is `https://example.org/` and thus the path is `/`.

special handling of 400 errors
special handling of all the other errors we see (e.g. when sending a get request to a route defined only as post)

Askama
    - conditional
    - vector of structs

