// demonstrate how to build molecule manually

use gchemol::prelude::*;
use gchemol::{Atom, Bond, Molecule};

// construct atom with element and position

#[test]
fn test_new_atom() {
    let a = Atom::new("Fe", [0.0, 0.0, 0.0]);
    let b = Atom::new("C", [1.2, 1.2, 1.2]);
    assert_eq!(a.symbol(), "Fe");
    assert_eq!(b.position(), [1.2, 1.2, 1.2]);

    // or simply convert from a string:
    let mut a: Atom = "Fe 1.0 1.0 0.2".parse().expect("atom from string");
    assert_eq!(a.symbol(), "Fe");
    assert_eq!(a.position(), [1.0, 1.0, 0.2]);

    // set more attributes
    a.set_momentum([0.0, 5.0, 6.0]);
    assert_eq!([0.0, 5.0, 6.0], a.momentum());
}

// build molecule from atoms

#[test]
fn test_new_molecule() {
    let atoms = vec![
        Atom::new("C", [0.000000, 0.000000, 0.000000]),
        Atom::new("C", [0.000000, 0.000000, 1.089000]),
        Atom::new("C", [1.026719, 0.000000, -0.363000]),
        Atom::new("C", [-0.513360, -0.889165, -0.363000]),
        Atom::new("C", [-0.513360, 0.889165, -0.363000]),
    ];

    // create a molecule named as methane
    let mol = Molecule::new("methane");
    assert_eq!(mol.title(), "methane");

    // build molecule from atoms
    let mut mol = Molecule::from_atoms(atoms);
    assert_eq!(5, mol.natoms());

    // update positions
    let positions = vec![
        [-0.90203687, 0.62555259, 0.0081889],
        [-0.54538244, -0.38325741, 0.0081889],
        [-0.54536403, 1.12995078, 0.8654626],
        [-0.54536403, 1.12995078, -0.8654626],
        [-1.97203687, 0.62556577, 0.0081889],
    ];
    mol.set_positions(positions);

    // udpate symbols
    let symbols = vec!["C", "H", "H", "H", "H"];
    mol.set_symbols(symbols.clone());
    for (s1, s2) in mol.symbols().zip(symbols) {
        assert_eq!(s1, s2);
    }

    // molecular title
    mol.set_title(" CH4 \nthe second line will be omitted\n");
    assert_eq!("CH4".to_string(), mol.title());
}

// adhoc properties
// Set arbitrary properties for atom, bond, molecule for adhoc uses.

#[test]
fn test_molecule_properties() {
    // create Atom object
    let mut a = Atom::default();

    // store a key with an integer value
    let value = 12;
    a.properties.store("secret", value);
    let unpacked: isize = a.properties.load("secret").expect("secret property");
    assert_eq!(value, unpacked);

    // store a key with a list of integers
    let value = [1, 2, 3];
    a.properties.store("secret", value);

    // test if the property exists
    assert!(a.properties.contains_key("secret"));
    assert!(!a.properties.contains_key("blank"));

    // unpack the property
    let unpacked: Vec<isize> = a.properties.load("secret").expect("secret property");
    for i in 0..value.len() {
        assert_eq!(value[i], unpacked[i]);
    }

    // create Bond object
    let mut bond = Bond::single();
    let value = 1.2;
    bond.properties.store("bond-order", value);
    let unpacked: f64 = bond.properties.load("bond-order").expect("bond property");
    assert_eq!(value, unpacked);

    // create Molecule object
    let mut mol = Molecule::new("test");
    let value = 12;
    mol.properties.store("secret", value);
    let unpacked: isize = mol.properties.load("secret").expect("secret number");
    assert_eq!(value, unpacked);
}

// access atomic data
// access element data such as covalent radii, atomic masses.

#[test]
fn test_atom_data() {
    let h = Atom::new("H", [0.0; 3]);
    let x = Atom::new("X", [0.0; 3]);
    // hydrogen atom
    assert!(h.is_element());
    // dummy atom
    assert!(x.is_dummy());

    assert!(h.get_cov_radius().is_some());
    assert!(h.get_vdw_radius().is_some());
    assert!(h.get_mass().is_some());

    assert!(x.get_cov_radius().is_none());
    assert!(x.get_vdw_radius().is_none());
    assert!(x.get_mass().is_none());
}
