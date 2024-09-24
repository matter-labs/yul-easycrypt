//!
//! Builder for a path inside EasyCrypt AST.
//!

use crate::easycrypt::syntax::Name;

use super::tracker::PathTracker;
use super::Path;
use super::Step;

///
/// Facilitates building an instance of [`Path`].
///
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Builder {
    elements: Path,
}

impl Builder {
    ///
    /// Create a new instance of the path builder.
    ///
    pub fn new(elements: Path) -> Self {
        Self { elements }
    }

    fn push(&mut self, step: &Step) {
        self.elements.stack.push(step.clone())
    }
}

impl PathTracker for Builder {
    fn here(&self) -> &Path {
        &self.elements
    }

    fn leave(&mut self) {
        self.elements.stack.pop();
    }

    fn enter_module(&mut self, ident: impl Into<Name>) {
        self.push(&Step::Module(ident.into()));
    }

    fn enter_procedure(&mut self, ident: impl Into<Name>) {
        self.push(&Step::Procedure(ident.into()));
    }
}
