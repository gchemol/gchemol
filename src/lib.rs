// [[file:../gchemol.note::27d00877][27d00877]]
#![deny(warnings)]
//===============================================================================#
//   DESCRIPTION:  gchemol: a Graph-based CHEMical Objects Library
//
//       OPTIONS:  ---
//  REQUIREMENTS:  ---
//         NOTES:  rewrite from scratch using Rust
//        AUTHOR:  Wenping Guo <ybyygu@gmail.com>
//       LICENCE:  GPL version 2 or upper
//       CREATED:  <2018-04-10 Tue 15:46>
//===============================================================================#
// 27d00877 ends here

// [[file:../gchemol.note::66712791][66712791]]
pub use gchemol_core::*;

pub mod io {
    pub use gchemol_gut::fs::{read_file, write_to_file};
    pub use gchemol_readwrite::*;
}

pub mod geom {
    pub use gchemol_geometry::*;
}

pub mod prelude {
    pub use gchemol_geometry::prelude::*;
    pub use gchemol_readwrite::prelude::*;
}

/// Conversion factors from different units
#[allow(non_upper_case_globals)]
pub mod units {
    pub const eV: f64 = 1.0;

    pub const Angstrom: f64 = 1.0;

    pub const Hartree: f64 = 27.211386024367243;

    pub const Bohr: f64 = 0.5291772105638411;

    /// Boltzmann constant
    pub const kB: f64 = 8.617330337217213E-05;

    /// femtosecond
    pub const fs: f64 = 0.09822694788464063;
}
// 66712791 ends here
