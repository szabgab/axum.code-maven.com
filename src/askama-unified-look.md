# Askama - unified look

In most application we'll have different types of pages.
e.g. a main page, a page listing products, and a separate page for each product.

How can we make sure they will look similar?

For each page-type we'll have its own template file and we need to ensure that the top and bottom part of the HTML is the same in each file.  Instead of copying these structures to each template there are two main methods to handle this problem.

## include

In one method we move the top and bottom parts to separate template files called `header.html` and `footer.html` and then we `include` them in every template.

## layout

We move the top and bottom part to a single separate file and each template `extends` the layout template.

