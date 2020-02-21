// header

// [[file:~/Workspace/Programming/gchemol-rs/gchemol/gchemol.note::*header][header:1]]
//===============================================================================#
//   DESCRIPTION:  gchemol: a Graph-based CHEMical Objects Library
//
//       OPTIONS:  ---
//  REQUIREMENTS:  ---
//         NOTES:  rewrite from scratch using Rust
//        AUTHOR:  Wenping Guo <ybyygu@gmail.com>
//       LICENCE:  GPL version 2 or upper
//       CREATED:  <2018-04-10 Tue 15:46>
//       UPDATED:  <2020-02-21 Fri 13:07>
//===============================================================================#
// header:1 ends here

// exports

// [[file:~/Workspace/Programming/gchemol-rs/gchemol/gchemol.note::*exports][exports:1]]
pub use gchemol_core::*;

pub mod io {
    pub use gchemol_gut::fs::{read_file, write_to_file};
    pub use gchemol_readwrite::*;
}

pub mod geom {
    pub use gchemol_geometry::*;
}

pub mod prelude {
    pub use gchemol_readwrite::prelude::*;
}
// exports:1 ends here

// compat

// [[file:~/Workspace/Programming/gchemol-rs/gchemol/gchemol.note::*compat][compat:1]]
/// For maintaining compatibility
pub mod compat {
    use crate::{Atom, Molecule};

    pub trait GchemolCompat {
        fn positions_vec(&self) -> Vec<[f64; 3]> {
            todo!()
        }

        fn atoms_vec(&self) -> Vec<&Atom> {
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
    }
}

use crate::compat::*;
// compat:1 ends here
