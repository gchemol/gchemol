// [[file:../gchemol.note::*header][header:1]]
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
//       UPDATED:  <2022-03-26 Sat 10:17>
//===============================================================================#
// header:1 ends here

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

// [[file:../gchemol.note::*compat][compat:1]]
#[cfg(feature = "adhoc")]
/// For maintaining compatibility
pub mod compat {
    use crate::{Atom, Molecule};

    pub trait GchemolCompat
    where
        Self: Sized,
    {
        fn positions_vec(&self) -> Vec<[f64; 3]> {
            todo!()
        }

        fn atoms_vec(&self) -> Vec<&Atom> {
            todo!()
        }

        fn symbols_vec(&self) -> Vec<&str> {
            todo!()
        }

        fn sorted(&self) -> Self {
            todo!()
        }

        fn fragment(&self) -> Vec<Self> {
            todo!()
        }
    }

    impl GchemolCompat for Molecule {
        /// replace old .positions() method
        fn positions_vec(&self) -> Vec<[f64; 3]> {
            self.positions().collect()
        }

        /// replace old .atoms() method
        fn atoms_vec(&self) -> Vec<&Atom> {
            self.atoms().map(|(_, a)| a).collect()
        }

        /// replace old .symbols() method
        fn symbols_vec(&self) -> Vec<&str> {
            self.symbols().collect()
        }

        /// replace old .sorted() method
        fn sorted(&self) -> Self {
            let numbers: Vec<_> = self.atomic_numbers().map(|n| std::cmp::Reverse(n)).collect();

            let mut mol = self.clone();
            mol.reorder(&numbers);
            mol
        }

        // FIXME: method name, fragment or something else?
        /// Break molecule into multiple fragments based on its bonding connectivity.
        fn fragment(&self) -> Vec<Self> {
            self.graph().connected_components().map(|g| Molecule::from_graph(g)).collect()
        }
    }
}
// compat:1 ends here
