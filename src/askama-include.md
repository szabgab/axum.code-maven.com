# Askama - include

If there template snippets we would like to use in more than one page, we can move those snippets
to a separate template file and then `include` that file in some of the templates.
I personally like to separate such templates - that are supposed to be included - in a subfolder called `incl`. That's what we did in this example.

1. We moved the form to a separate file.
1. We included the `incl/echo_form.html` in both the main page and the `echo.html` file.
1. We update the tests to verify that the form appears on both pages.


{% embed include file="src/examples/askama-include/Cargo.toml" %}

{% embed include file="src/examples/askama-include/src/main.rs" %}

{% embed include file="src/examples/askama-include/src/tests.rs" %}

{% embed include file="src/examples/askama-include/templates/main.html" %}

{% embed include file="src/examples/askama-include/templates/echo.html" %}

{% embed include file="src/examples/askama-include/templates/incl/echo_form.html" %}


