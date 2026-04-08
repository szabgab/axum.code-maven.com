# Askama - `safe`

If you have an application where users can type in some data that you later display on web pages, especially pages that can be viewed by other then you have to make sure the data does not contain any syntax that would let one person run code in the browser of the other person.

The standard safeguard is to make sure any potential HTML tag submitted by users is escaped when we display them. To faciliate this Askama escapes all HTML contained in variables.

However, if that HTML comes from some trusted source, for example because your input system verified that no dangerous tages are permitted or you have markdown files and you generate HTML from them before passing to the template, then you want to tell Askama to embed the data as it is without escaping.

You can achieve this by using the `safe` filter.


{% embed include file="src/examples/askama-safe/Cargo.toml" %}

{% embed include file="src/examples/askama-safe/src/main.rs" %}

{% embed include file="src/examples/askama-safe/src/tests.rs" %}

{% embed include file="src/examples/askama-safe/templates/main.html" %}


