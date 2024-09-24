//!
//! Tracker of paths from the root of a EasyCrypt syntax tree.
//!

use crate::easycrypt::syntax::Name;

use super::Path;

///
/// Tracker of paths from the root of a EasyCrypt syntax tree.
///
pub trait PathTracker {
    ///
    /// Currently constructed path from the root of EasyCrypt syntax tree.
    ///
    fn here(&self) -> &Path;

    ///
    /// Exit the last lexical block on the way from the root of EasyCrypt syntax tree.
    ///
    fn leave(&mut self);

    ///
    /// Enter a module on the way from the root of EasyCrypt syntax tree.
    ///
    fn enter_module(&mut self, identifier: impl Into<Name>);

    ///
    /// Enter a procedure on the way from the root of EasyCrypt syntax tree.
    ///
    fn enter_procedure(&mut self, identifier: impl Into<Name>);
}
