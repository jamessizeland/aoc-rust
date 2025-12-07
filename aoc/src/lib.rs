mod helpers {
    pub mod file;
    pub mod grid;
    pub mod parsing;
}

pub mod prelude {
    #![allow(unused)]

    use super::helpers;
    pub use anyhow::{Result, anyhow};
    pub use helpers::file::*;
    pub use helpers::grid::*;
    pub use helpers::parsing::*;
    pub use itertools::Itertools;
    pub use rayon::prelude::*;
}
