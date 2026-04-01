# Askama - variable

{% embed include file="src/examples/askama-variable/Cargo.toml" %}

{% embed include file="src/examples/askama-variable/src/main.rs" %}

{% embed include file="src/examples/askama-variable/src/tests.rs" %}

{% embed include file="src/examples/askama-variable/templates/main.html" %}

{% embed include file="src/examples/askama-variable/templates/echo.html" %}


There are several issues with this solution:

1. When we get the response we would probably like to see the form again.
1. If the user submits the form without typing in anything we might want to show a different message.

