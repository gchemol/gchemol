// [[file:../gchemol.note::985c6117][985c6117]]
#![allow(non_upper_case_globals)]

pub const eV: f64 = 1.0;

pub const Angstrom: f64 = 1.0;

pub const Hartree: f64 = 27.211386024367243;

pub const Bohr: f64 = 0.5291772105638411;

/// Boltzmann constant
pub const kB: f64 = 8.617330337217213E-05;

/// femtosecond in a.u.
pub const fs: f64 = 0.09822694788464063;
// 985c6117 ends here

// [[file:../gchemol.note::a14f268e][a14f268e]]
/// selected energy units
pub fn energy(unit: &str) -> uom::si::f64::Energy {
    use uom::si::energy::*;
    use uom::si::f64::Energy;

    match unit {
        "eV" => Energy::new::<electronvolt>(1.0),
        "MeV" => Energy::new::<megaelectronvolt>(1.0),
        "KeV" => Energy::new::<kiloelectronvolt>(1.0),
        "Hartree" | "Ha" | "a.u." => Energy::new::<electronvolt>(Hartree),
        "kcal" => Energy::new::<kilocalorie>(1.0),
        "kJ" => Energy::new::<kilojoule>(1.0),
        _ => todo!(),
    }
}

/// selected length units
pub fn length(unit: &str) -> uom::si::f64::Length {
    use uom::si::f64::Length;
    use uom::si::length::*;

    match unit {
        "Angstrom" | "A" | "Å" => Length::new::<angstrom>(1.0),
        "Bohr" => Length::new::<angstrom>(Bohr),
        "fm" => Length::new::<femtometer>(1.0),
        "pm" => Length::new::<picometer>(1.0),
        "nm" => Length::new::<nanometer>(1.0),
        "µm" => Length::new::<micrometer>(1.0),
        _ => todo!(),
    }
}

#[test]
fn test_units_uom() {
    assert_eq!((energy("Hartree") / energy("eV")).value, Hartree);
}
// a14f268e ends here
