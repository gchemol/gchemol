// inertia
// :PROPERTIES:
// :header-args: :comments org :tangle tests/inertia.rs
// :END:

#[test]
fn test_molecule_inertia() {
    use gchemol::prelude::*;
    use gchemol::Molecule;
    use vecfx::*;

    let mol = Molecule::from_file("tests/files/xyz/H2O.xyz").expect("H2O");
    let imat = [
        [7.79383372e-01, 3.25014857e-01, 2.21867130e-31],
        [3.25014857e-01, 1.00944905e+00, 4.43734259e-31],
        [2.21867130e-31, 4.43734259e-31, 1.78883242e+00],
    ];
    let x = mol.inertia_matrix();
    approx::assert_relative_eq!(x.to_matrix(), imat.to_matrix(), epsilon = 1e-3);
}
