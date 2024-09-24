//!
//! Step on a path from the root of EasyCrypt syntax tree to a definition.
//!

use crate::easycrypt::syntax::Name;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Step {
    Module(Name),
    Procedure(Name),
}

impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Step::Module(name) | Step::Procedure(name) => f.write_str(name),
        }
    }
}
