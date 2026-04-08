# Askama - include header footer

```
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── main.rs
│   └── tests.rs
└── templates
    ├── incl
    │   ├── footer.html
    │   └── header.html
    ├── main.html
    └── page.html

```

{% embed include file="src/examples/askama-header-footer/Cargo.toml" %}

{% embed include file="src/examples/askama-header-footer/src/main.rs" %}

{% embed include file="src/examples/askama-header-footer/src/tests.rs" %}

{% embed include file="src/examples/askama-header-footer/templates/page.html" %}

{% embed include file="src/examples/askama-header-footer/templates/incl/header.html" %}

{% embed include file="src/examples/askama-header-footer/templates/incl/footer.html" %}


