
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

-   [Installation · The Rust Programming Language](https://www.rust-lang.org/tools/install)

add gchemol dependency to your Cargo.toml:

    [dependencies]
    gchemol = "0.1"


## Atom

    use gchemol::Atom;
    
    // construct from element and position
    let a = Atom::new("C", [0.0, 0.0, 0.0]);
    let b = Atom::new("C", [1.2, 1.2, 1.2]);

or simply convert from a string:

    let a: Atom = "C 1.0 1.0 0.2"
        .parse()
        .expect("atom from string");


## Molecule

1.  Creating a molecule manually

        use gchemol::Molecule;
        
        let atoms = [
            Atom::new("C", [ 0.000000,   0.000000,  0.000000]),
            Atom::new("C", [ 0.000000,   0.000000,  1.089000]),
            Atom::new("C", [ 1.026719,   0.000000, -0.363000]),
            Atom::new("C", [-0.513360,  -0.889165, -0.363000]),
            Atom::new("C", [-0.513360,   0.889165, -0.363000])];
        
        // create a molecule from these atoms
        let mut mol = Molecule::from_atoms(atoms);

2.  Reading and writing molecules

        use gchemol::io;
        use gchemol::prelude::*;
        use gchemol::Molecule;
        
        // Read an xyz file and write to a Gaussian Input file.
        let mol = Molecule::from_file("path/to/file.xyz")?;
        mol.to_file("methane.gjf")?;
        
        // get the total number of atoms
        let na = mol.natoms();
        // get the total number of bonds
        let nb = mol.nbonds();
        
        // read multiple molecules (trajectory) from a chemical file
        // the number of atoms in different frame could be different
        let mols = io::read("path/to/trajectory.xyz")?;


# Related projects

-   [ase](https://wiki.fysik.dtu.dk/ase/)
-   [lumol](https://github.com/lumol-org/lumol)


# References

-   [Lessons from sixteen years of molecular simulation in Python | Konrad Hinsen's Blog](https://khinsen.wordpress.com/2013/04/10/lessons-from-sixteen-years-of-molecular-simulation-in-python/)

