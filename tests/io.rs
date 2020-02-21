// read/write molecule from/to file

#[test]
fn io_from_file_to_file() {
    use gchemol::prelude::*;
    use gchemol::Molecule;

    // 1. string io
    let txt1 = String::from_file("tests/files/mol2/alanine-gv.mol2").expect("txt1 from file");

    // write to a temp file
    let tfile = tempfile::Builder::new()
        .suffix(".mol2")
        .tempfile()
        .expect(".mol2 temp file");

    // validate
    txt1.to_file(&tfile).expect("write mol2");
    let txt2 = String::from_file(tfile).expect("txt2 from file");
    assert_eq!(txt1, txt2);

    // 2. molecule io
    let mol1 = Molecule::from_file("tests/files/mol2/alanine-gv.mol2").expect("mol1 from file");

    // write to a temp file in xyz format
    let tfile = tempfile::Builder::new()
        .suffix(".xyz")
        .tempfile()
        .expect(".xyz temp file");
    mol1.to_file(&tfile).expect("write mol2 file in xyz format");
    let mol2 = Molecule::from_file(tfile).expect("mol2 from file");
    assert_eq!(mol1.natoms(), mol2.natoms());
}

// read write all molecules in file

#[test]
fn read_write_molecules() {
    use gchemol::prelude::*;
    use gchemol::Molecule;

    let f = "./tests/files/mol2/multi-obabel.mol2";
    // loop over molecules parsed from path `f`
    for mol in gchemol::io::read(&f).expect("read mol2") {
        //
    }

    // read all molecules into a Vec
    let mols = gchemol::io::read_all(&f).expect("read all mol2");
    assert_eq!(mols.len(), 6);

    // write all molecules to a temp file
    let path = tempfile::Builder::new()
        .suffix(".mol2")
        .tempfile()
        .expect(".mol2 temp file");
    gchemol::io::write(&path, &mols).expect("write mol2");

    // force to write in xyz format
    gchemol::io::write_format(&path, &mols, "text/xyz").expect("write format xyz");

    // parse in specific format
    let file = std::fs::File::open(&path).expect("open file");
    let mols = gchemol::io::read_from(file, "text/xyz").expect("read xyz from");
    assert_eq!(mols.count(), 6, "Failed to read in xyz format");

    let s = String::from_file(&path).expect("read str");
    let mol = Molecule::from_str(&s, "text/xyz").expect("from xyz");
    // write in specific format
    let s = mol.format_as("text/xyz").expect("format xyz");
    assert!(!s.is_empty());

    // format molecule using user defined template
    let tpl = "./tests/files/templates/xyz.tera";
    let s = mol.render_with(tpl.as_ref()).expect("render tera");
    assert!(!s.is_empty());
}
