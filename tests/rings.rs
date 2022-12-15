// rings.rs
// :PROPERTIES:
// :header-args: :comments org :tangle tests/rings.rs
// :END:
// #+name: 9e40d9ac

#[test]
fn test_find_rings_xyz() {
    use gchemol::prelude::*;
    use gchemol::Molecule;
    use gchemol_gut::prelude::*;

    let mol = Molecule::from_file("tests/files/mol2/rings.mol2").unwrap();

    // output in NGPH for test against vitroid/CountRings
    // println!("@NGPH");
    // println!("{}", mol.natoms());
    // for (i, j, _) in mol.view_bonds() {
    //     println!("{} {}", i - 1, j - 1);
    // }
    // println!("-1 -1");

    let rings = mol.find_rings(6);

    // one 3-member-ring, one 4-member-ring, and one 6-member-ring
    let ns = rings.iter().map(|ri| ri.len()).sorted().collect_vec();
    assert_eq!(ns, [3, 4, 6]);
}
