//!
//! Path from the root of EasyCrypt syntax tree to some location in it.
//!

use era_yul::util::iter::prefixes;

use self::step::Step;

pub mod builder;
pub mod step;
pub mod tracker;

///
/// Path from the root of EasyCrypt syntax tree to a specific lexical block in it,
/// including all the blocks on the way from root.
///
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Path {
    pub stack: Vec<Step>,
}

impl Path {
    ///
    /// Iterate over all parents of this path, starting from the path itself.
    ///
    pub fn parents(&self) -> impl '_ + Iterator<Item = Path> {
        prefixes(self.stack.as_slice())
            .rev()
            .map(|s| Path { stack: s.to_vec() })
    }

    ///
    /// Returns a new instance of an empty [`Path`].
    ///
    pub fn empty() -> Path {
        Path { stack: vec![] }
    }
}
