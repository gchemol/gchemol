
# gchemol

gchemol is a graph-based chemical object library implemented in Rust programming
language. This project is still in early development.

[![Crates.io](https://img.shields.io/crates/v/gchemol)](https://crates.io/crates/gchemol)


# Features

-   Fast and safe codes empowered by Rust.
-   Static build without external dependencies: easy to deploy in HPC environment.
-   Core graph data structure using [petgraph](https://github.com/bluss/petgraph)
-   Read/write molecules in [common chemcial file formats](https://github.com/gchemol/gchemol-readwrite/tree/master/src/formats).
-   Linear algebra backed by [nalgebra](http://nalgebra.org/)
-   Render molecule in user defined formats by templating with [handlebars](https://github.com/sunng87/handlebars-rust) and [tera](https://tera.netlify.com/docs)


# How to use


## Setup

follow the official guide:

-   [Installation · The Rust Programming Language](https://www.rust-lang.org/en-US/install.html)

add gchemol dependency to your Cargo.toml:

    [dependencies]
    gchemol = "0.1"


# Related projects

-   [ase](https://wiki.fysik.dtu.dk/ase/)
-   [lumol](https://github.com/lumol-org/lumol)


# References

-   [Lessons from sixteen years of molecular simulation in Python | Konrad Hinsen's Blog](https://khinsen.wordpress.com/2013/04/10/lessons-from-sixteen-years-of-molecular-simulation-in-python/)

