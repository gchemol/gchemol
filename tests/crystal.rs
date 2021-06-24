// [[file:../gchemol.note::*build lattice][build lattice:1]]
use vecfx::approx::assert_relative_eq;

#[test]
fn test_crystal() {
    use gchemol::{Lattice, Molecule};

    // build lattice from lattice parameters
    let _ = Lattice::from_params(
        3.84, // a
        3.84, // b
        3.84, // c
        120., // alpha
        90.,  // beta
        60.,  // gamma
    );

    // build lattice from cell vectors
    let lat = Lattice::new([
        [15.3643, 0.0000, 0.0000], // vector a
        [4.5807, 15.5026, 0.0000], // vector b
        [0.0000, 0.0000, 17.4858], // vector c
    ]);

    // create an empty molecule
    let mut mol = Molecule::new("empty");

    // set periodic boundary conditions for molecule
    mol.set_lattice(lat);

    // remove periodic boundary conditions
    mol.unbuild_crystal();
    assert!(!mol.is_periodic());
}
// build lattice:1 ends here

// [[file:../gchemol.note::*supercell][supercell:1]]
#[test]
fn test_supercell() {
    use gchemol::prelude::*;
    use gchemol::Molecule;

    let path = "tests/files/cif/babel.cif";
    let mol = Molecule::from_file(path).expect("test file: babel.cif");
    assert!(mol.is_periodic());

    let mol = mol.supercell(3, 3, 1).unwrap();
    assert_eq!(306, mol.natoms());
    let lat = mol.lattice.unwrap();
    let [a, b, c] = lat.lengths();
    assert_relative_eq!(a, 11.4600, epsilon = 1e-4);
    assert_relative_eq!(b, 19.5630, epsilon = 1e-4);
    assert_relative_eq!(c, 13.8670, epsilon = 1e-4);
}
// supercell:1 ends here
