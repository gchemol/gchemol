// selection.rs
// :PROPERTIES:
// :header-args: :comments org :tangle tests/selection.rs
// :END:
// #+name: d97f367b

use gchemol_gut::prelude::*;
use gchemol_gut::utils::abbreviate_numbers_human_readable;

#[cfg(feature = "adhoc")]
#[test]
fn test_selection_by_bond() -> Result<()> {
    use gchemol::prelude::*;
    use gchemol::Molecule;

    let file = "./tests/files/mol2/Ni211.mol2";
    let mol = Molecule::from_file(file)?;
    let s = mol.selection_by_expanding_bond(44, 1);
    let s = abbreviate_numbers_human_readable(s.into_iter().collect_vec().as_slice())?;
    let s_expected = "28,40,42,44,46,56,60,62";
    assert_eq!(s, s_expected);
    let s = mol.selection_by_expanding_bond(44, 2);
    let s = abbreviate_numbers_human_readable(s.into_iter().collect_vec().as_slice())?;
    let s_expected = "12,18-19,24-26,28,30,34-35,40-44,46-48,50-51,56-58,60,62-64,72,76,78";
    assert_eq!(s, s_expected);

    Ok(())
}
