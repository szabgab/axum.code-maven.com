# Web development in Rust using axum

https://axum.code-maven.com/








## Ecosystem

We copy the ECOSYSTEM.md file from the source of axum. Use the `ecosystem.pl` file for this.

In our application we need to map the path part of each URL the user might visit to a function to handle that request.
For this we need a function to handle the request and we need the map the path portion of the URL to the function that will handle it.
For example if we would like to handle the URL `https://example.org/hello/world` then we need to map `/hello/world` path to the appropriate function in our application. In particular the address of the main page is `https://example.org/` and thus the path is `/`.

special handling of 400 errors (e.g. when the parameter is missing, value is incorrect or the parsing fails for other resonse
special handling of all the other errors we see (e.g. when sending a get request to a route defined only as post)

Askama
    - Other filters
    - Create a filter manually, commafy?
    - 3rd party filters?


Show example of external templates that are setup next to the binary (so no embedded in the binary)

Show redirect on condition (e.g. on Academy we have redirect to login and also a redirect after login)

Show how to handle a case when a path variable matches, but the value is still incorrect e.g. /user/ID and the ID is not in the database. (is it a 404 error?)

Add more details about tracing (how to save into a file with timestamp, how to set log level, how to filter areas of logging (only our app not axum)

In the nested example add a main page to the submodule as well to show it working along


