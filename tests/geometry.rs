// basic stats
// #+name: dc805a7a

use gchemol::prelude::*;
use gchemol::Molecule;
use vecfx::approx::assert_relative_eq;

// measurement
// #+name: 91197950

#[test]
fn test_mol_measure_geom() {
    let mol = Molecule::from_file("./tests/files/xyz/c2h4.xyz").unwrap();
    assert_relative_eq!(1.35520, mol.get_distance(1, 4).unwrap(), epsilon = 1E-3);
    assert_relative_eq!(119.887, mol.get_angle(3, 1, 4).unwrap().to_degrees(), epsilon = 1E-3);
    assert_relative_eq!(0.0, mol.get_torsion(2, 1, 4, 6).unwrap().to_degrees(), epsilon = 1E-3);
}

// superimpose
// #+name: 049603c7

#[test]
fn test_superimpose() {
    let mut mol = Molecule::from_database("CH4");
    let mol_ref = mol.clone();
    mol.translate([0.5, 1.0, 2.0]);
    let rmsd = mol.superimpose_onto(&mol_ref, None);
    assert_relative_eq!(rmsd, 0.0, epsilon = 1E-5);

    let rmsd = mol.superimpose_onto(&mol_ref, Some(&[1, 2, 3, 4]));
    assert_relative_eq!(rmsd, 0.0, epsilon = 1E-5);
}
