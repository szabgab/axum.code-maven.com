# Hello world on the command line

Before we start building web applications using axum, first let's go over step-by-step how you can start building an application.

In order to get started open a terminal window and type in the following:

```
cargo new hello-world
```

This will create a folder called `hello-world` with some basic files. Using the `tree` command available on Linux machines we can see the layout of the folder:

```
$ tree hello-world/
hello-world/
├── Cargo.toml
└── src
    └── main.rs

2 directories, 2 files
```

The `Cargo.toml` file contains the meta-data of the project sucha as the name of the crate thatwill be `hello-world` and the dependencies that we'll add soon.

{% embed include file="src/examples/hello-world/Cargo.toml" %}

The `main.rs` file contains a very basic Rust application printing `Hello, world!` on the screen.

{% embed include file="src/examples/hello-world/src/main.rs" %}

In addition a `.gitignore` file is also created

{% embed include file="src/examples/hello-world/.gitignore" %}

and a git repository is set up for the project. So you'll also find the `.git` folder.

## Run the "application"

Let's change into the directory of the project.

```
cd hello-world
```

and run the application:

```
cargo run
```

This will compile the application in the `target/` folder (hence it was listed in the `.gitignore` file.),
eventualy creating the `target/debug/hello-world` executable. This command will also run the executable
printing `Hello, world!` to the screen.

It will also create a file called `Cargo.lock` that will hold the exact version of all the dependencies (both direct and indirect) of the project. For now it is mostly empty.

## What's next?

On the next pages we'll see a number of examples. You can create a new crate for each example or your can simply replace the files in the current folder with the files of the example.


