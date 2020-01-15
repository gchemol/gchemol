// header

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-new/gchemol-new.note::*header][header:1]]
//===============================================================================#
//   DESCRIPTION:  gchemol: a Graph-based CHEMical Objects Library
//
//       OPTIONS:  ---
//  REQUIREMENTS:  ---
//         NOTES:  rewrite from scratch using Rust
//        AUTHOR:  Wenping Guo <ybyygu@gmail.com>
//       LICENCE:  GPL version 2 or upper
//       CREATED:  <2018-04-10 Tue 15:46>
//       UPDATED:  <2020-02-05 Wed 08:51>
//===============================================================================#
// header:1 ends here

// exports

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-new/gchemol-new.note::*exports][exports:1]]
pub use gchemol_core::*;
pub use gchemol_readwrite as io;

pub mod prelude {
    pub use gchemol_readwrite::prelude::*;
    // pub use gchemol_geometry::*;
    // pub use gchemol_geometry::prelude::*;
}
// exports:1 ends here
