// [[file:../gchemol.note::*alignment][alignment:1]]
#[test]
fn test_alignment() {
    use approx::assert_relative_eq;
    use gchemol::geom::Superpose;
    use gchemol::prelude::*;
    use gchemol::Molecule;
    use vecfx::*;

    // load test molecules
    let mol1 = Molecule::from_file("tests/files/alignment/reference.mol2").expect("alignment reference");
    let mol2 = Molecule::from_file("tests/files/alignment/candidate.mol2").expect("alignment candidate");

    // take the first 5 atoms for superposition
    let reference: Vec<_> = mol1.positions().take(5).collect();
    let candidate: Vec<_> = mol2.positions().take(5).collect();

    // align the candidate onto the reference
    let sp = Superpose::new(&candidate).onto(&reference, None);

    // apply superposition to all atoms
    let new = sp.apply(&candidate);
    assert_relative_eq!(reference.to_matrix(), new.to_matrix(), epsilon = 1e-3);
}
// alignment:1 ends here
