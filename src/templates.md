# Templates

In a web application that return HTML there are at least 5 languages involved.

* The backend language. In our case it is Rust.
* SQL to access the relational database
* HTML
* CSS for styling
* JavaScript for certain client-side elements. E.g. converting the menu into the hamburger on small screens
or sorting tables.

Putting all these languages in the same file creates a mess. It is much easier of we can separate them.

HTML can include CSS and JavaScript from external files and a template system can make it easy to move all the HTML out from the Rust code.

Every template system is a mini-language with variables, conditional statements, loops and some similar constructs.


