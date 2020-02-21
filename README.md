
# gchemol

gchemol is a graph-based chemical object library implemented in Rust programming
language. This project is still in early development.

[![Crates.io](https://img.shields.io/crates/v/gchemol)](https://crates.io/crates/gchemol)
[![Built with Spacemacs](https://cdn.rawgit.com/syl20bnr/spacemacs/442d025779da2f62fc86c2082703697714db6514/assets/spacemacs-badge.svg)](http://spacemacs.org)


# Features

-   Fast and safe powered by Rust.
-   Easy to deploy in server environment.
-   core graph data structure using [petgraph](https://github.com/bluss/petgraph)
-   read molecules in various formats using [nom](https://github.com/Geal/nom) parser combinators
-   linear algebra backed by [nalgebra](http://nalgebra.org/)
-   render molecule in user defined formats by templating with [handlebars](https://github.com/sunng87/handlebars-rust) and [tera](https://tera.netlify.com/docs)


# How to use


## install rust

follow the official guide:

-   [Installation · The Rust Programming Language](https://www.rust-lang.org/en-US/install.html)


## setup

add gchemol dependency to your Cargo.toml:

    [dependencies]
    gchemol = "0.0.39"


# Related projects

-   lumol
-   ase
-   pymatgen


# References

-   [Lessons from sixteen years of molecular simulation in Python | Konrad Hinsen's Blog](https://khinsen.wordpress.com/2013/04/10/lessons-from-sixteen-years-of-molecular-simulation-in-python/)

