// basic stats
// #+name: dc805a7a

use gchemol::prelude::*;
use gchemol::Molecule;
use vecfx::approx::assert_relative_eq;

// mol
// #+name: 91197950

#[test]
fn test_mol_measure_geom() {
    let mol = Molecule::from_file("./tests/files/xyz/c2h4.xyz").unwrap();
    assert_relative_eq!(1.35520, mol.get_distance(1, 4).unwrap(), epsilon = 1E-3);
    assert_relative_eq!(119.887, mol.get_angle(3, 1, 4).unwrap().to_degrees(), epsilon = 1E-3);
    assert_relative_eq!(0.0, mol.get_torsion(2, 1, 4, 6).unwrap().to_degrees(), epsilon = 1E-3);
}
